use super::{BinaryOp, Constant, Expr};
use std::ops::Sub;

impl<Num: Constant, Rhs: Into<Expr<Num>>> Sub<Rhs> for Expr<Num> {
  type Output = Self;
  fn sub(self, rhs: Rhs) -> Self::Output {
    Expr::Binary(
      BinaryOp::new("Sub(x-y)", &|x: Num, y| x - y),
      Box::new(self),
      Box::new(rhs.into()),
    )
  }
}

impl<Num: Constant, Rhs: Into<Expr<Num>>> Sub<Rhs> for &Expr<Num> {
  type Output = Expr<Num>;
  fn sub(self, rhs: Rhs) -> Self::Output {
    Expr::Binary(
      BinaryOp::new("Sub(x-y)", &|x: Num, y| x % y),
      Box::new(self.clone()),
      Box::new(rhs.into()),
    )
  }
}
