use std::{
  collections::HashMap,
  ops::{Deref, DerefMut},
};

use super::{Expr, Token};

#[derive(Debug, PartialEq)]
pub struct Substitutions(HashMap<String, Expr>);

impl Substitutions {
  pub fn new() -> Self {
    Self(HashMap::new())
  }
}

impl<const N: usize, S1: Into<String>, S2: Into<String>> From<[(S1, S2); N]> for Substitutions {
  fn from(value: [(S1, S2); N]) -> Self {
    let mut subs = Self::new();
    for (name, expr) in value {
      subs.insert(name.into(), expr.into().parse().unwrap());
    }
    subs
  }
}

impl Deref for Substitutions {
  type Target = HashMap<String, Expr>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Substitutions {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[derive(Debug, PartialEq)]
pub enum SubstituteError {
  Inconsistent(String, Expr, Expr),
  NotMatching,
}

impl Expr {
  fn no_children(tkn: &Token) -> usize {
    use self::Token::*;
    match tkn {
      Binary(_) => 2,
      Unary(_) => 1,
      Func(_, Some(n)) => n.clone(),
      Number(_) | Var(_) => 0,
      _ => panic!("expression wasn't parsed correctly!"),
    }
  }

  /// Returns the children of a given token in self, from last to first.
  fn children_at<'a>(
    &'a self,
    self_ptrs: &'a Vec<usize>,
    i: usize,
  ) -> impl Iterator<Item = usize> + 'a {
    use self::Token::*;
    let n = match self.0[i] {
      Binary(_) => 2,
      Unary(_) => 1,
      Func(_, Some(n)) => n,
      Number(_) | Var(_) => 0,
      _ => panic!("expression wasn't parsed correctly!"),
    };

    // We use .max(1) to avoid overflow panics in here.
    // This is safe as we'll always stop before returning any negative index.
    (0..n).scan(i.max(1) - 1, move |i, _| {
      let res = *i;
      *i = self_ptrs[*i].max(1) - 1;
      Some(res)
    })
  }

  /// Finds a vector of pointers to the beginning of each subexpression
  /// for each Token within an expression. Mainly useful when attempting to
  /// pattern match an expression
  ///
  /// e.g. `start_pointers({3, x, +, z, *}) = {0, 1, 0, 3, 0}`
  pub(crate) fn start_pointers(&self) -> Vec<usize> {
    let mut ptrs = Vec::with_capacity(self.0.len());

    for (mut i, t) in self.0.iter().enumerate() {
      for _ in 0..Self::no_children(t) {
        i = ptrs[i - 1]
      }
      ptrs.push(i);
    }

    ptrs
  }

  pub(crate) fn fold_expr<T>(&self, f: impl Fn(Vec<T>, Token) -> T) -> T {
    let mut vals = vec![];

    for t in self.0.iter() {
      let n = Self::no_children(t);
      let vs = vals.split_off(vals.len() - n);
      vals.push(f(vs, t.clone()))
    }

    vals.pop().unwrap()
  }

  /// Matches a subexpression within self, returning any variables within
  /// `to_match` bound to the expression in the same place within self.
  ///
  /// Returns:
  /// - `Ok(Substitutions)` if matching successful
  /// - `Err(NotMatching)` if the structure of `to_match` doesn't match self
  /// - `Err(Inconsistent)` if a variable is assigned different subexpressions
  ///
  /// # Examples
  ///
  /// ```rust
  /// use meval_symbolic::Expr;
  /// let expr: Expr = "(x + 7) * (2*x + 9)".parse().unwrap();
  /// let mtch: Expr = "(  a  ) * ( b  + c)".parse().unwrap();
  /// let subs = expr.extract(&mtch).unwrap();
  ///
  /// assert_eq!(subs, [
  ///   ("a", "x + 7"),
  ///   ("b", "2*x"),
  ///   ("c", "9"),
  /// ].into());
  /// ```
  pub fn extract(&self, term: &Expr) -> Result<Substitutions, SubstituteError> {
    use self::SubstituteError::{Inconsistent, NotMatching};
    use self::Token::*;

    let term_ptrs = term.start_pointers();
    let self_ptrs = self.start_pointers();
    let mut subs = Substitutions::new();
    let mut to_check = vec![(self_ptrs.len() - 1, term_ptrs.len() - 1)];

    while let Some((i, j)) = to_check.pop() {
      if let Var(ident) = &term.0[j] {
        // If we have a variable in our term to match, bind the current
        // subexpression to it in the substitutions returned
        let sub_expr = Expr(Vec::from(&self.0[self_ptrs[i]..=i]));

        let prev = subs.insert(ident.clone(), sub_expr.clone());
        if prev.clone().map_or(false, |expr| expr != sub_expr) {
          return Err(Inconsistent(ident.clone(), prev.unwrap(), sub_expr));
        }
      } else if self.0[i] == term.0[j] {
        // If the current tokens match, recurse on the subexpressions
        let is = self.children_at(&self_ptrs, i);
        let js = term.children_at(&term_ptrs, j);
        to_check.extend(is.into_iter().zip(js.into_iter()))
      } else {
        return Err(NotMatching);
      }
    }

    Ok(subs)
  }

  /// Attempts to match the entire expression to `to_replace` and replaces it
  /// with `replacement`, replacing all matched variables from `to_replace`
  /// within `replacement`.
  ///
  /// Returns:
  /// - Ok(Expr) if the match and replacement was successful
  /// - Err(SubstituteError) if extracting `to_replace` fails
  ///
  /// # Examples
  ///
  /// ```rust
  /// use meval_symbolic::Expr;
  /// let expr: Expr = "(x + 7) * (2*x + 9)".parse().unwrap();
  /// let mtch: Expr = "(  a  ) * ( b  + c)".parse().unwrap();
  /// let rplc: Expr = "a*b + a*c".parse().unwrap();
  /// let subs = expr.replace(&mtch, &rplc).unwrap();
  ///
  /// assert_eq!(subs, "(x+7)*(2*x) + (x+7)*9".parse().unwrap());
  /// ```
  pub fn replace(&self, term: &Expr, rplc: &Expr) -> Result<Expr, SubstituteError> {
    use self::Token::Var;

    let subs = self.extract(term)?;
    let try_sub = |tkn: &Token| {
      if let Var(ident) = tkn {
        if let Some(expr) = subs.get(ident) {
          return expr.0.clone().into_iter();
        }
      }
      vec![(tkn.clone())].into_iter()
    };

    Ok(Expr(rplc.iter().flat_map(try_sub).collect()))
  }

  /// Substitutes subexpressions matching term with the replacement term.
  ///
  /// Note on implementation:
  /// We apply subsitutions from the top down, only processing the first
  /// substitution within each branch to avoid exponential growth due to terms
  /// like `x` -> `x + x`
  ///
  /// Another advantage of applying from top down is that the items are
  /// replaced from back to front, ensuring that no start pointers need to be
  /// updated.
  /// If we went from front to back, the size of the initial section of the
  /// expression would change, requiring us to increment or decrement the
  /// pointers after this initial section.
  pub fn substitute(&self, term: &Expr, rplc: &Expr) -> Result<Expr, SubstituteError> {
    let self_ptrs = self.start_pointers();
    let mut res = self.clone();
    let mut to_sub = vec![false; res.len()];
    to_sub.last_mut().map(|x| *x = true);

    // iterate backwards through tokens
    for j in (0..res.len()).rev() {
      if !to_sub[j] {
        continue;
      }
      let i = self_ptrs[j];
      let sub_expr = Expr(self.0[i..=j].iter().cloned().collect());

      if let Ok(expr) = sub_expr.replace(term, rplc) {
        // replace end and don't recurse
        res.0.splice(i..=j, expr.0);
      } else {
        // mark all children to be checked
        for k in self.children_at(&self_ptrs, j) {
          to_sub[k] = true;
        }
      }
    }

    Ok(res)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn extract_reflexive_ok() {
    let expr: Expr = "abs(+7 + 10 * -2)".parse().unwrap();
    assert_eq!(expr.extract(&expr), Ok(Substitutions::new()));
  }

  #[test]
  fn extract_reflexive_var() {
    let expr: Expr = "sqrt((x + 5.0e1) % 2.0)".parse().unwrap();
    assert_eq!(expr.extract(&expr), Ok([("x", "x")].into()));
  }

  #[test]
  fn extract_single_var_same() {
    let expr: Expr = "an(odd(combo), of(funcs(and), vars))".parse().unwrap();
    let mtch: Expr = "x".parse().unwrap();
    let subs = expr.extract(&mtch).unwrap();
    assert_eq!(subs.get("x"), Some(&expr));
  }

  #[test]
  fn extract_mismatch_fails() {
    use self::SubstituteError::NotMatching;
    let expr: Expr = "-b + sqrt(b^2 - 4*a*c)".parse().unwrap();
    let mtch: Expr = "5.0".parse().unwrap();
    assert_eq!(expr.extract(&mtch), Err(NotMatching));
  }

  #[test]
  fn extract_inconsistent_fails() {
    use self::SubstituteError::Inconsistent;
    let expr: Expr = "-b + sqrt(b^2 - 4*a*c)".parse().unwrap();
    let mtch: Expr = "-x + x".parse().unwrap();
    let expected = Inconsistent(
      "x".into(),
      "b".parse().unwrap(),
      "sqrt(b^2 - 4*a*c)".parse().unwrap(),
    );
    assert_eq!(expr.extract(&mtch), Err(expected));
  }

  #[test]
  fn extract_two_variables_ok() {
    let expr: Expr = "-b + sqrt(b^2 - 4*a*c)".parse().unwrap();
    let mtch: Expr = "-x + sqrt(y)".parse().unwrap();
    assert_eq!(
      expr.extract(&mtch),
      Ok([("x", "b"), ("y", "b^2 - 4*a*c")].into())
    );
  }

  #[test]
  fn replace_reflexive_unit() {
    let expr: Expr = "(x + 3)^2 + abs(y % 2)".parse().unwrap();
    let mtch: Expr = "x + y".parse().unwrap();
    assert_eq!(expr.replace(&mtch, &mtch), Ok(expr));
  }

  #[test]
  fn replace_two_variables_ok() {
    let expr: Expr = "(x + 2) / (7 + y)".parse().unwrap();
    let mtch: Expr = "a / b".parse().unwrap();
    let rplc: Expr = "a * b".parse().unwrap();
    assert_eq!(
      expr.replace(&mtch, &rplc),
      Ok("(x + 2) * (7 + y)".parse().unwrap())
    );
  }

  #[test]
  fn replace_mismatch_fails() {
    use self::SubstituteError::*;
    let expr: Expr = "(x * 2) + x".parse().unwrap();
    let mtch: Expr = "6".parse().unwrap();
    let rplc: Expr = "_".parse().unwrap();
    assert_eq!(expr.replace(&mtch, &rplc), Err(NotMatching));
  }

  #[test]
  fn replace_inconsistent_fails() {
    use self::SubstituteError::*;
    let expr: Expr = "(x * 2) + x".parse().unwrap();
    let mtch: Expr = "u + u".parse().unwrap();
    let rplc: Expr = "_".parse().unwrap();
    assert_eq!(
      expr.replace(&mtch, &rplc),
      Err(Inconsistent(
        "u".into(),
        "x * 2".parse().unwrap(),
        "x".parse().unwrap()
      ))
    )
  }

  #[test]
  fn substitute_reflexive_id() {
    let expr: Expr = "-b - sqrt(b^2 - 4*a*c)".parse().unwrap();
    let mtch: Expr = "x - y".parse().unwrap();
    assert_eq!(expr.substitute(&mtch, &mtch), Ok(expr))
  }

  #[test]
  fn substitute_top_down() {
    let expr: Expr = "(w + x) + y".parse().unwrap();
    let mtch: Expr = "a + b".parse().unwrap();
    let rplc: Expr = "a".parse().unwrap();
    assert_eq!(expr.substitute(&mtch, &rplc), Ok("w + x".parse().unwrap()))
  }

  #[test]
  fn substitute_multiple() {
    let expr: Expr = "(x + 2) / (7 + y)".parse().unwrap();
    let mtch: Expr = "a + b".parse().unwrap();
    let rplc: Expr = "a".parse().unwrap();
    assert_eq!(expr.substitute(&mtch, &rplc), Ok("x / 7".parse().unwrap()))
  }
}
