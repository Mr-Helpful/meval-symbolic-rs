use super::{binary_operator, Expr, Operation, Token};
pub use std::ops::Rem;

impl<Rhs: Into<Expr>> Rem<Rhs> for Expr {
  type Output = Self;
  fn rem(self, rhs: Rhs) -> Self::Output {
    binary_operator(self, rhs, Token::Binary(Operation::Rem))
  }
}

impl<Rhs: Into<Expr>> Rem<Rhs> for &Expr {
  type Output = Expr;
  fn rem(self, rhs: Rhs) -> Self::Output {
    self.clone().rem(rhs)
  }
}
