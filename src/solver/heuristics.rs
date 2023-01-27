use super::{Eqtn, Expr, Token};

/// A trait used to define a ranking over some node value.
/// This differs from the standard definition of Ord in that it allows
/// multiple different rankings / strategies to be defined on the same type.
pub trait Heuristic<N> {
  type Order: Ord;
  fn value(&self, node: &N) -> Self::Order;
}

impl<N, H1, H2> Heuristic<N> for (H1, H2)
where
  H1: Heuristic<N>,
  H2: Heuristic<N>,
{
  type Order = (H1::Order, H2::Order);
  fn value(&self, node: &N) -> Self::Order {
    (self.0.value(node), self.1.value(node))
  }
}
impl<N, H1, H2, H3> Heuristic<N> for (H1, H2, H3)
where
  H1: Heuristic<N>,
  H2: Heuristic<N>,
  H3: Heuristic<N>,
{
  type Order = (H1::Order, H2::Order, H3::Order);
  fn value(&self, node: &N) -> Self::Order {
    (self.0.value(node), self.1.value(node), self.2.value(node))
  }
}

/// A helper function to translate heuristics on expressions into heuristics on
/// equations. We define the ordering via a tuple that prioritises the LHS.
fn rank_eqtn<H: Heuristic<Expr>>(heur: &H, Eqtn(lhs, rhs): &Eqtn) -> (H::Order, H::Order) {
  (heur.value(lhs), heur.value(rhs))
}

/// A trait used to initialise some Heuristic over variables
pub trait OnVar {
  fn on(var: String) -> Self;
}

impl<O1: OnVar, O2: OnVar> OnVar for (O1, O2) {
  fn on(var: String) -> Self {
    (OnVar::on(var.clone()), OnVar::on(var))
  }
}
impl<O1: OnVar, O2: OnVar, O3: OnVar> OnVar for (O1, O2, O3) {
  fn on(var: String) -> Self {
    (
      OnVar::on(var.clone()),
      OnVar::on(var.clone()),
      OnVar::on(var),
    )
  }
}

/// Ranking by the maximum depth a variable occurs at
pub struct MaxNesting(String);
impl Heuristic<Expr> for MaxNesting {
  type Order = usize;
  fn value(&self, node: &Expr) -> Self::Order {
    use self::Token::Var;
    node.fold_expr(|vs, t| {
      if Var(self.0.clone()) == t {
        return 1;
      }
      let max = vs.into_iter().max().unwrap_or(0);
      if max > 0 {
        max + 1
      } else {
        max
      }
    })
  }
}
impl Heuristic<Eqtn> for MaxNesting {
  type Order = (usize, usize);
  fn value(&self, node: &Eqtn) -> Self::Order {
    rank_eqtn(self, node)
  }
}
impl OnVar for MaxNesting {
  fn on(var: String) -> Self {
    Self(var)
  }
}

/// Ranking by the minimum depth a variable occurs at
pub struct MinNesting(String);
impl Heuristic<Expr> for MinNesting {
  type Order = usize;
  fn value(&self, node: &Expr) -> Self::Order {
    use self::Token::Var;
    node.fold_expr(|vs, t| {
      if Var(self.0.clone()) == t {
        return 1;
      }
      let min = vs.into_iter().min().unwrap_or(0);
      if min > 0 {
        min + 1
      } else {
        min
      }
    })
  }
}
impl Heuristic<Eqtn> for MinNesting {
  type Order = (usize, usize);
  fn value(&self, node: &Eqtn) -> Self::Order {
    rank_eqtn(self, node)
  }
}
impl OnVar for MinNesting {
  fn on(var: String) -> Self {
    Self(var)
  }
}

/// Ranking by the no. occurences of a variable
pub struct NoOccurences(String);
impl Heuristic<Expr> for NoOccurences {
  type Order = usize;
  fn value(&self, node: &Expr) -> Self::Order {
    use self::Token::Var;
    node.iter().filter(|t| t == &&Var(self.0.clone())).count()
  }
}
impl Heuristic<Eqtn> for NoOccurences {
  type Order = (usize, usize);
  fn value(&self, node: &Eqtn) -> Self::Order {
    rank_eqtn(self, node)
  }
}
impl OnVar for NoOccurences {
  fn on(var: String) -> Self {
    Self(var)
  }
}

/// Ranking by the overall length
pub struct Length;
impl Heuristic<Expr> for Length {
  type Order = usize;
  fn value(&self, node: &Expr) -> Self::Order {
    node.len()
  }
}
impl Heuristic<Eqtn> for Length {
  type Order = (usize, usize);
  fn value(&self, node: &Eqtn) -> Self::Order {
    rank_eqtn(self, node)
  }
}
impl OnVar for Length {
  fn on(_: String) -> Self {
    Self
  }
}
