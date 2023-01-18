use super::{Constant, Expr, UnaryOp};

pub trait Ceil {
  type Output;
  fn ceil(self) -> Self::Output;
}
unary_trait_ref!(Ceil, ceil);

impl<Num: Constant> Ceil for Expr<Num> {
  type Output = Self;
  fn ceil(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Ceil", &|x: Num| x.ceil()), Box::new(self))
  }
}
