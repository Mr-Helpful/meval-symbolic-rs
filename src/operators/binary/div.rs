use super::{BinaryOp, Constant, Expr};
use std::ops::Div;

impl<Num: Constant, Rhs: Into<Expr<Num>>> Div<Rhs> for Expr<Num> {
  type Output = Self;
  fn div(self, rhs: Rhs) -> Self::Output {
    Expr::Binary(
      BinaryOp::new("Div(x/y)", &|x: Num, y| x / y),
      Box::new(self),
      Box::new(rhs.into()),
    )
  }
}

impl<Num: Constant, Rhs: Into<Expr<Num>>> Div<Rhs> for &Expr<Num> {
  type Output = Expr<Num>;
  fn div(self, rhs: Rhs) -> Self::Output {
    Expr::Binary(
      BinaryOp::new("Div(x/y)", &|x: Num, y| x / y),
      Box::new(self.clone()),
      Box::new(rhs.into()),
    )
  }
}
