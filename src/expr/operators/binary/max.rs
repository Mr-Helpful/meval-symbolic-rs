use super::{binary_operator, Expr, Token};

pub trait Max<Rhs = Self> {
  type Output;
  fn max(self, rhs: Rhs) -> Self::Output;
}
binary_trait_ref!(Max, max);

impl<Rhs: Into<Expr>> Max<Rhs> for Expr {
  type Output = Self;
  fn max(self, rhs: Rhs) -> Self::Output {
    binary_operator(self, rhs, Token::Func("max".into(), Some(2)))
  }
}
