use super::{binary_operator, Expr, Operation, Token};
pub use std::ops::Mul;

impl<Rhs: Into<Expr>> Mul<Rhs> for Expr {
  type Output = Self;
  fn mul(self, rhs: Rhs) -> Self::Output {
    binary_operator(self, rhs, Token::Binary(Operation::Times))
  }
}

impl<Rhs: Into<Expr>> Mul<Rhs> for &Expr {
  type Output = Expr;
  fn mul(self, rhs: Rhs) -> Self::Output {
    self.clone().mul(rhs)
  }
}
