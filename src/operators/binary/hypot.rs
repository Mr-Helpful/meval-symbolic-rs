use super::{BinaryOp, Constant, Expr};

pub trait Hypot<Rhs = Self> {
  type Output;
  fn hypot(self, rhs: Rhs) -> Self::Output;
}
binary_trait_ref!(Hypot, hypot);

impl<Num: Constant, Rhs: Into<Expr<Num>>> Hypot<Rhs> for Expr<Num> {
  type Output = Self;
  fn hypot(self, rhs: Rhs) -> Self::Output {
    Expr::Binary(
      BinaryOp::new("Hypot", &|x: Num, y| x.hypot(y)),
      Box::new(self),
      Box::new(rhs.into()),
    )
  }
}
