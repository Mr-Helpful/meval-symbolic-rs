use super::{Constant, Expr, UnaryOp};

pub trait Sinh {
  type Output;
  fn sinh(self) -> Self::Output;
}
unary_trait_ref!(Sinh, sinh);

impl<Num: Constant> Sinh for Expr<Num> {
  type Output = Self;
  fn sinh(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Sinh", &|x: Num| x.sinh()), Box::new(self))
  }
}
