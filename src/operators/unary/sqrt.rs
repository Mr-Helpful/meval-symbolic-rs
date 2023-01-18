use super::{Constant, Expr, UnaryOp};

pub trait Sqrt {
  type Output;
  fn sqrt(self) -> Self::Output;
}
unary_trait_ref!(Sqrt, sqrt);

impl<Num: Constant> Sqrt for Expr<Num> {
  type Output = Self;
  fn sqrt(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Sqrt", &|x: Num| x.sqrt()), Box::new(self))
  }
}
