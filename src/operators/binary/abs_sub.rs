use super::{BinaryOp, Constant, Expr};

pub trait AbsSub<Rhs = Self> {
  type Output;
  fn abs_sub(self, rhs: Rhs) -> Self::Output;
}
binary_trait_ref!(AbsSub, abs_sub);

impl<Num: Constant, Rhs: Into<Expr<Num>>> AbsSub<Rhs> for Expr<Num> {
  type Output = Self;
  fn abs_sub(self, rhs: Rhs) -> Self::Output {
    Expr::Binary(
      BinaryOp::new("AbsSub", &|x: Num, y| x.abs_sub(y)),
      Box::new(self),
      Box::new(rhs.into()),
    )
  }
}
