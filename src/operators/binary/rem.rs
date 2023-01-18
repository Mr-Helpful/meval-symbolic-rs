use super::{BinaryOp, Constant, Expr};
use std::ops::Rem;

impl<Num: Constant, Rhs: Into<Expr<Num>>> Rem<Rhs> for Expr<Num> {
  type Output = Self;
  fn rem(self, rhs: Rhs) -> Self::Output {
    Expr::Binary(
      BinaryOp::new("Rem(x%y)", &|x: Num, y| x % y),
      Box::new(self),
      Box::new(rhs.into()),
    )
  }
}

impl<Num: Constant, Rhs: Into<Expr<Num>>> Rem<Rhs> for &Expr<Num> {
  type Output = Expr<Num>;
  fn rem(self, rhs: Rhs) -> Self::Output {
    Expr::Binary(
      BinaryOp::new("Rem(x%y)", &|x: Num, y| x % y),
      Box::new(self.clone()),
      Box::new(rhs.into()),
    )
  }
}
