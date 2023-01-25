use super::{Expr, Operation, Token};
pub use std::ops::Mul;

impl<Rhs: Into<Expr>> Mul<Rhs> for Expr {
  type Output = Self;
  fn mul(self, rhs: Rhs) -> Self::Output {
    self.binary_operator(rhs, Token::Binary(Operation::Times))
  }
}

impl<Rhs: Into<Expr>> Mul<Rhs> for &Expr {
  type Output = Expr;
  fn mul(self, rhs: Rhs) -> Self::Output {
    self.clone().mul(rhs)
  }
}
