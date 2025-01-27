use super::{binary_operator, Expr, Token};

pub trait ATan2<Rhs = Self> {
  type Output;
  fn atan2(self, rhs: Rhs) -> Self::Output;
}
binary_trait_ref!(ATan2, atan2);

impl<Rhs: Into<Expr>> ATan2<Rhs> for Expr {
  type Output = Self;
  fn atan2(self, rhs: Rhs) -> Self::Output {
    binary_operator(self, rhs, Token::Func("atan2".into(), Some(2)))
  }
}
