use super::{Expr, Operation, Token};
pub use std::ops::Rem;

impl<Rhs: Into<Expr>> Rem<Rhs> for Expr {
  type Output = Self;
  fn rem(self, rhs: Rhs) -> Self::Output {
    self.binary_operator(rhs, Token::Binary(Operation::Rem))
  }
}

impl<Rhs: Into<Expr>> Rem<Rhs> for &Expr {
  type Output = Expr;
  fn rem(self, rhs: Rhs) -> Self::Output {
    self.clone().rem(rhs)
  }
}
