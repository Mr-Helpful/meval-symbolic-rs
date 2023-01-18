use super::{Constant, Expr, UnaryOp};

pub trait Sin {
  type Output;
  fn sin(self) -> Self::Output;
}
unary_trait_ref!(Sin, sin);

impl<Num: Constant> Sin for Expr<Num> {
  type Output = Self;
  fn sin(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Sin", &|x: Num| x.sin()), Box::new(self))
  }
}
