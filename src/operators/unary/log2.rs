use super::{Constant, Expr, UnaryOp};

pub trait Log2 {
  type Output;
  fn log2(self) -> Self::Output;
}
unary_trait_ref!(Log2, log2);

impl<Num: Constant> Log2 for Expr<Num> {
  type Output = Self;
  fn log2(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Log2", &|x: Num| x.log2()), Box::new(self))
  }
}
