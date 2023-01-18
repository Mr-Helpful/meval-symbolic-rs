use super::{BinaryOp, Constant, Expr};

pub trait ATan2<Rhs = Self> {
  type Output;
  fn atan2(self, rhs: Rhs) -> Self::Output;
}
binary_trait_ref!(ATan2, atan2);

impl<Num: Constant, Rhs: Into<Expr<Num>>> ATan2<Rhs> for Expr<Num> {
  type Output = Self;
  fn atan2(self, rhs: Rhs) -> Self::Output {
    Expr::Binary(
      BinaryOp::new("ATan2", &|x: Num, y| x.atan2(y)),
      Box::new(self),
      Box::new(rhs.into()),
    )
  }
}
