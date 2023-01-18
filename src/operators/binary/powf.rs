use super::{BinaryOp, Constant, Expr};

pub trait Powf<Rhs = Self> {
  type Output;
  fn powf(self, rhs: Rhs) -> Self::Output;
}
binary_trait_ref!(Powf, powf);

impl<Num: Constant, Rhs: Into<Expr<Num>>> Powf<Rhs> for Expr<Num> {
  type Output = Self;
  fn powf(self, rhs: Rhs) -> Self::Output {
    Expr::Binary(
      BinaryOp::new("Powf", &|x: Num, y| x.powf(y)),
      Box::new(self),
      Box::new(rhs.into()),
    )
  }
}
