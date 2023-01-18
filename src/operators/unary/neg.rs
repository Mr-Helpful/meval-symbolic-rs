use super::{Constant, Expr, UnaryOp};
use std::ops::Neg;

impl<Num: Constant> Neg for Expr<Num> {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Neg(-x)", &|x: Num| -x), Box::new(self))
  }
}

impl<Num: Constant> Neg for &Expr<Num> {
  type Output = Expr<Num>;
  fn neg(self) -> Self::Output {
    Expr::Unary(
      UnaryOp::new("Neg(-x)", &|x: Num| -x),
      Box::new(self.clone()),
    )
  }
}
