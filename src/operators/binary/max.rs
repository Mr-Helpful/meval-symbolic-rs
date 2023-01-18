use super::{BinaryOp, Constant, Expr};

pub trait Max<Rhs = Self> {
  type Output;
  fn max(self, rhs: Rhs) -> Self::Output;
}
binary_trait_ref!(Max, max);

impl<Num: Constant, Rhs: Into<Expr<Num>>> Max<Rhs> for Expr<Num> {
  type Output = Self;
  fn max(self, rhs: Rhs) -> Self::Output {
    Expr::Binary(
      BinaryOp::new("Max", &|x: Num, y| x.max(y)),
      Box::new(self),
      Box::new(rhs.into()),
    )
  }
}
