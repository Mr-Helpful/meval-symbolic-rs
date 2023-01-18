use super::{Constant, Expr, UnaryOp};

pub trait Cos {
  type Output;
  fn cos(self) -> Self::Output;
}
unary_trait_ref!(Cos, cos);

impl<Num: Constant> Cos for Expr<Num> {
  type Output = Self;
  fn cos(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Cos", &|x: Num| x.cos()), Box::new(self))
  }
}
