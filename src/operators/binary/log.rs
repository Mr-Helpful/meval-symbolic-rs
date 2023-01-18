use super::{BinaryOp, Constant, Expr};

pub trait Log<Rhs = Self> {
  type Output;
  fn log(self, rhs: Rhs) -> Self::Output;
}
binary_trait_ref!(Log, log);

impl<Num: Constant, Rhs: Into<Expr<Num>>> Log<Rhs> for Expr<Num> {
  type Output = Self;
  fn log(self, rhs: Rhs) -> Self::Output {
    Expr::Binary(
      BinaryOp::new("Log", &|x: Num, y| x.log(y)),
      Box::new(self),
      Box::new(rhs.into()),
    )
  }
}
