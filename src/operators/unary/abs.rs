use super::{Constant, Expr, UnaryOp};

pub trait Abs {
  type Output;
  fn abs(self) -> Self::Output;
}
unary_trait_ref!(Abs, abs);

impl<Num: Constant> Abs for Expr<Num> {
  type Output = Self;
  fn abs(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Abs", &|x: Num| x.abs()), Box::new(self))
  }
}
