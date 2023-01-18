use super::{Constant, Expr, UnaryOp};

pub trait Log10 {
  type Output;
  fn log10(self) -> Self::Output;
}
unary_trait_ref!(Log10, log10);

impl<Num: Constant> Log10 for Expr<Num> {
  type Output = Self;
  fn log10(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Log10", &|x: Num| x.log10()), Box::new(self))
  }
}
