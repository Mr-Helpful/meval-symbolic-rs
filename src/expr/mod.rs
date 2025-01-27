use std::ops::Deref;
use std::str::FromStr;

use self::{extra_math::factorial, parser::tokenizer::Operation};
use crate::Evaluatable_Trait;
use Error;

pub use self::{
  context::{builtin, max_array, min_array, ArgGuard, Context, ContextProvider},
  errors::FuncEvalError,
  parser::{
    shunting_yard::{self, RPNError},
    tokenizer::{self, ParseError, Token},
  },
  symbolic::SubstituteError,
};

mod context;
mod errors;
mod extra_math;
mod operators;
mod parser;
mod symbolic;

/// Representation of a parsed expression.
///
/// The expression is internally stored in the [reverse Polish notation (RPN)][RPN] as a sequence
/// of `Token`s.
///
/// Methods `bind`, `bind_with_context`, `bind2`, ... can be used to create  closures from
/// the expression that then can be passed around and used as any other `Fn` closures.
///
/// ```rust
/// let func = "x^2".parse::<meval::Expr>().unwrap().bind("x").unwrap();
/// let r = Some(2.).map(func);
/// assert_eq!(r, Some(4.));
/// ```
///
/// [RPN]: https://en.wikipedia.org/wiki/Reverse_Polish_notation
#[derive(Debug, Clone, PartialEq)]
pub struct Expr(pub(crate) Vec<Token>);

impl Expr {
  /// Evaluates the expression with variables given by the argument.
  pub fn eval_with_context<C: ContextProvider>(&self, ctx: C) -> Result<f64, Error> {
    use self::Operation::*;
    use self::Token::*;

    let mut stack = Vec::with_capacity(16);

    for token in &self.0 {
      match *token {
        Var(ref n) => {
          if let Some(v) = ctx.get_var(n) {
            stack.push(v);
          } else {
            return Err(Error::UnknownVariable(n.clone()));
          }
        }
        Number(f) => stack.push(f),
        Binary(op) => {
          let right = stack.pop().unwrap();
          let left = stack.pop().unwrap();
          let r = match op {
            Plus => left + right,
            Minus => left - right,
            Times => left * right,
            Div => left / right,
            Rem => left % right,
            Pow => left.powf(right),
            _ => {
              return Err(Error::EvalError(format!(
                "Unimplemented binary operation: {:?}",
                op
              )));
            }
          };
          stack.push(r);
        }
        Unary(op) => {
          let x = stack.pop().unwrap();
          let r = match op {
            Plus => x,
            Minus => -x,
            Fact => {
              // Check to make sure x has no fractional component (can be converted to int without loss)
              match factorial(x) {
                Ok(res) => res,
                Err(e) => return Err(Error::EvalError(String::from(e))),
              }
            }
            _ => {
              return Err(Error::EvalError(format!(
                "Unimplemented unary operation: {:?}",
                op
              )));
            }
          };
          stack.push(r);
        }
        Func(ref n, Some(i)) => {
          if stack.len() < i {
            return Err(Error::EvalError(format!(
              "eval: stack does not have enough arguments for function token \
                             {:?}",
              token
            )));
          }
          match ctx.eval_func(n, &stack[stack.len() - i..]) {
            Ok(r) => {
              let nl = stack.len() - i;
              stack.truncate(nl);
              stack.push(r);
            }
            Err(e) => return Err(Error::Function(n.to_owned(), e)),
          }
        }
        _ => return Err(Error::EvalError(format!("Unrecognized token: {:?}", token))),
      }
    }

    let r = stack.pop().expect("Stack is empty, this is impossible.");
    if !stack.is_empty() {
      return Err(Error::EvalError(format!(
        "There are still {} items on the stack.",
        stack.len()
      )));
    }
    Ok(r)
  }

  /// Checks that the value of every variable in the expression is specified by
  /// the context `ctx`.
  ///
  /// # Failure
  ///
  /// Returns `Err` if a missing variable is detected.
  pub(crate) fn check_context<C: ContextProvider>(&self, ctx: C) -> Result<(), Error> {
    for t in &self.0 {
      match *t {
        Token::Var(ref name) => {
          if ctx.get_var(name).is_none() {
            return Err(Error::UnknownVariable(name.clone()));
          }
        }
        Token::Func(ref name, Some(i)) => {
          let v = vec![0.; i];
          if let Err(e) = ctx.eval_func(name, &v) {
            return Err(Error::Function(name.to_owned(), e));
          }
        }
        Token::Func(_, None) => {
          return Err(Error::EvalError(format!(
            "expr::check_context: Unexpected token: {:?}",
            *t
          )));
        }
        Token::LParen
        | Token::RParen
        | Token::Binary(_)
        | Token::Unary(_)
        | Token::Comma
        | Token::Number(_) => {}
      }
    }
    Ok(())
  }
}

Evaluatable_Trait!(Expr f64);

/// Evaluates a string with built-in constants and functions.
pub fn eval_str<S: AsRef<str>>(expr: S) -> Result<f64, Error> {
  let expr = Expr::from_str(expr.as_ref())?;

  expr.eval_with_context(builtin())
}

impl From<&Expr> for Expr {
  fn from(value: &Expr) -> Self {
    value.clone()
  }
}

/// Evaluates a string with the given context.
///
/// No built-ins are defined in this case.
pub fn eval_str_with_context<S: AsRef<str>, C: ContextProvider>(
  expr: S,
  ctx: C,
) -> Result<f64, Error> {
  let expr = Expr::from_str(expr.as_ref())?;

  expr.eval_with_context(ctx)
}

impl Deref for Expr {
  type Target = [Token];

  fn deref(&self) -> &[Token] {
    &self.0
  }
}

#[cfg(test)]
mod tests {
  use crate::expr::errors::FuncEvalError;

  use super::*;
  use std::str::FromStr;
  use Error;

  #[test]
  fn test_eval() {
    assert_eq!(eval_str("2 + 3"), Ok(5.));
    assert_eq!(eval_str("2 + (3 + 4)"), Ok(9.));
    assert_eq!(eval_str("-2^(4 - 3) * (3 + 4)"), Ok(-14.));
    assert_eq!(eval_str("-2*3! + 1"), Ok(-11.));
    assert_eq!(eval_str("-171!"), Ok(std::f64::NEG_INFINITY));
    assert_eq!(eval_str("150!/148!"), Ok(22350.));
    assert_eq!(eval_str("a + 3"), Err(Error::UnknownVariable("a".into())));
    assert_eq!(eval_str("round(sin (pi) * cos(0))"), Ok(0.));
    assert_eq!(eval_str("round( sqrt(3^2 + 4^2)) "), Ok(5.));
    assert_eq!(eval_str("max(1.)"), Ok(1.));
    assert_eq!(eval_str("max(1., 2., -1)"), Ok(2.));
    assert_eq!(eval_str("min(1., 2., -1)"), Ok(-1.));
    assert_eq!(
      eval_str("sin(1.) + cos(2.)"),
      Ok((1f64).sin() + (2f64).cos())
    );
    assert_eq!(eval_str("10 % 9"), Ok(10f64 % 9f64));

    match eval_str("0.5!") {
      Err(Error::EvalError(_)) => {}
      _ => panic!("Cannot evaluate factorial of non-integer"),
    }
  }

  #[test]
  fn test_builtins() {
    assert_eq!(eval_str("atan2(1.,2.)"), Ok((1f64).atan2(2.)));
  }

  #[test]
  fn test_eval_func_ctx() {
    use std::collections::{BTreeMap, HashMap};
    let y = 5.;
    assert_eq!(
      eval_str_with_context("phi(2.)", Context::new().func("phi", |x| x + y + 3.)),
      Ok(2. + y + 3.)
    );
    assert_eq!(
      eval_str_with_context(
        "phi(2., 3.)",
        Context::new().func2("phi", |x, y| x + y + 3.)
      ),
      Ok(2. + 3. + 3.)
    );
    assert_eq!(
      eval_str_with_context(
        "phi(2., 3., 4.)",
        Context::new().func3("phi", |x, y, z| x + y * z)
      ),
      Ok(2. + 3. * 4.)
    );
    assert_eq!(
      eval_str_with_context(
        "phi(2., 3.)",
        Context::new().funcn("phi", |xs: &[f64]| xs[0] + xs[1], 2)
      ),
      Ok(2. + 3.)
    );
    let mut m = HashMap::new();
    m.insert("x", 2.);
    m.insert("y", 3.);
    assert_eq!(eval_str_with_context("x + y", &m), Ok(2. + 3.));
    assert_eq!(
      eval_str_with_context("x + z", m),
      Err(Error::UnknownVariable("z".into()))
    );
    let mut m = BTreeMap::new();
    m.insert("x", 2.);
    m.insert("y", 3.);
    assert_eq!(eval_str_with_context("x + y", &m), Ok(2. + 3.));
    assert_eq!(
      eval_str_with_context("x + z", m),
      Err(Error::UnknownVariable("z".into()))
    );
  }

  #[test]
  fn test_bind() {
    let expr = Expr::from_str("x + 3").unwrap();
    let func = expr.clone().bind("x").unwrap();
    assert_eq!(func(1.), 4.);

    assert_eq!(
      expr.clone().bind("y").err(),
      Some(Error::UnknownVariable("x".into()))
    );

    let ctx = (("x", 2.), builtin());
    let func = expr.bind_with_context(&ctx, "y").unwrap();
    assert_eq!(func(1.), 5.);

    let expr = Expr::from_str("x + y + 2.").unwrap();
    let func = expr.clone().bind2("x", "y").unwrap();
    assert_eq!(func(1., 2.), 5.);
    assert_eq!(
      expr.clone().bind2("z", "y").err(),
      Some(Error::UnknownVariable("x".into()))
    );
    assert_eq!(
      expr.bind2("x", "z").err(),
      Some(Error::UnknownVariable("y".into()))
    );

    let expr = Expr::from_str("x + y^2 + z^3").unwrap();
    let func = expr.clone().bind3("x", "y", "z").unwrap();
    assert_eq!(func(1., 2., 3.), 32.);

    let expr = Expr::from_str("sin(x)").unwrap();
    let func = expr.clone().bind("x").unwrap();
    assert_eq!(func(1.), (1f64).sin());

    let expr = Expr::from_str("sin(x,2)").unwrap();
    match expr.clone().bind("x") {
      Err(Error::Function(_, FuncEvalError::NumberArgs(1))) => {}
      _ => panic!("bind did not error"),
    }
    let expr = Expr::from_str("hey(x,2)").unwrap();
    match expr.clone().bind("x") {
      Err(Error::Function(_, FuncEvalError::UnknownFunction)) => {}
      _ => panic!("bind did not error"),
    }
  }
}
