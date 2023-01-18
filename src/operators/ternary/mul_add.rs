use super::{Constant, Expr, TernaryOp};

pub trait MulAdd<Mid, Rhs> {
  type Output;
  fn mul_add(self, mid: Mid, rhs: Rhs) -> Self::Output;
}
ternary_trait_ref!(MulAdd, mul_add);

impl<Num, Mid, Rhs> MulAdd<Mid, Rhs> for Expr<Num>
where
  Num: Constant,
  Mid: Into<Expr<Num>>,
  Rhs: Into<Expr<Num>>,
{
  type Output = Self;
  fn mul_add(self, mid: Mid, rhs: Rhs) -> Self::Output {
    Expr::Ternary(
      TernaryOp::new("MulAdd", &&|x: Num, y, z| x.mul_add(y, z)),
      Box::new(self),
      Box::new(mid.into()),
      Box::new(rhs.into()),
    )
  }
}
