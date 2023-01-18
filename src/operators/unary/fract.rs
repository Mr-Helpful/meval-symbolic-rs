use super::{Constant, Expr, UnaryOp};

pub trait Fract {
  type Output;
  fn fract(self) -> Self::Output;
}
unary_trait_ref!(Fract, fract);

impl<Num: Constant> Fract for Expr<Num> {
  type Output = Self;
  fn fract(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Fract", &|x: Num| x.fract()), Box::new(self))
  }
}
