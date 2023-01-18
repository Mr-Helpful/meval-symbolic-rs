use super::{BinaryOp, Constant, Expr};

pub trait Min<Rhs = Self> {
  type Output;
  fn min(self, rhs: Rhs) -> Self::Output;
}
binary_trait_ref!(Min, min);

impl<Num: Constant, Rhs: Into<Expr<Num>>> Min<Rhs> for Expr<Num> {
  type Output = Self;
  fn min(self, rhs: Rhs) -> Self::Output {
    Expr::Binary(
      BinaryOp::new("Min", &|x: Num, y| x.min(y)),
      Box::new(self),
      Box::new(rhs.into()),
    )
  }
}
