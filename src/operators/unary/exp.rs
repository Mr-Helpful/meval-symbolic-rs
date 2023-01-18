use super::{Constant, Expr, UnaryOp};

pub trait Exp {
  type Output;
  fn exp(self) -> Self::Output;
}
unary_trait_ref!(Exp, exp);

impl<Num: Constant> Exp for Expr<Num> {
  type Output = Self;
  fn exp(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Exp", &|x: Num| x.exp()), Box::new(self))
  }
}
