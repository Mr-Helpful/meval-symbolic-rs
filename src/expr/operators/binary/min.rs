use super::{binary_operator, Expr, Token};

pub trait Min<Rhs = Self> {
  type Output;
  fn min(self, rhs: Rhs) -> Self::Output;
}
binary_trait_ref!(Min, min);

impl<Rhs: Into<Expr>> Min<Rhs> for Expr {
  type Output = Self;
  fn min(self, rhs: Rhs) -> Self::Output {
    binary_operator(self, rhs, Token::Func("min".into(), Some(2)))
  }
}
