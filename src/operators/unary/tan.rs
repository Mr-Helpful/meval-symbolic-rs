use super::{Constant, Expr, UnaryOp};

pub trait Tan {
  type Output;
  fn tan(self) -> Self::Output;
}
unary_trait_ref!(Tan, tan);

impl<Num: Constant> Tan for Expr<Num> {
  type Output = Self;
  fn tan(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Tan", &|x: Num| x.tan()), Box::new(self))
  }
}
