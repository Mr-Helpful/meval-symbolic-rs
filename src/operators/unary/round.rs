use super::{Constant, Expr, UnaryOp};

pub trait Round {
  type Output;
  fn round(self) -> Self::Output;
}
unary_trait_ref!(Round, round);

impl<Num: Constant> Round for Expr<Num> {
  type Output = Self;
  fn round(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Round", &|x: Num| x.round()), Box::new(self))
  }
}
