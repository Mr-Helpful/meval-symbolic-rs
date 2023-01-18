use super::{BinaryOp, Constant, Expr};
use std::ops::Add;

impl<Num: Constant, Rhs: Into<Expr<Num>>> Add<Rhs> for Expr<Num> {
  type Output = Self;
  fn add(self, rhs: Rhs) -> Self::Output {
    Expr::Binary(
      BinaryOp::new("Add(x+y)", &|x: Num, y| x + y),
      Box::new(self),
      Box::new(rhs.into()),
    )
  }
}

impl<Num: Constant, Rhs: Into<Expr<Num>>> Add<Rhs> for &Expr<Num> {
  type Output = Expr<Num>;
  fn add(self, rhs: Rhs) -> Self::Output {
    Expr::Binary(
      BinaryOp::new("Add(x+y)", &|x: Num, y| x + y),
      Box::new(self.clone()),
      Box::new(rhs.into()),
    )
  }
}
