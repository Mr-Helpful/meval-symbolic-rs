use super::{BinaryOp, Constant, Expr};
use std::ops::Mul;

impl<Num: Constant, Rhs: Into<Expr<Num>>> Mul<Rhs> for Expr<Num> {
  type Output = Self;
  fn mul(self, rhs: Rhs) -> Self::Output {
    Expr::Binary(
      BinaryOp::new("Mul(x*y)", &|x: Num, y| x * y),
      Box::new(self),
      Box::new(rhs.into()),
    )
  }
}

impl<Num: Constant, Rhs: Into<Expr<Num>>> Mul<Rhs> for &Expr<Num> {
  type Output = Expr<Num>;
  fn mul(self, rhs: Rhs) -> Self::Output {
    Expr::Binary(
      BinaryOp::new("Mul(x*y)", &|x: Num, y| x * y),
      Box::new(self.clone()),
      Box::new(rhs.into()),
    )
  }
}
