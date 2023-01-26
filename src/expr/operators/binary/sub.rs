use super::{binary_operator, Expr, Operation, Token};
pub use std::ops::Sub;

impl<Rhs: Into<Expr>> Sub<Rhs> for Expr {
  type Output = Self;
  fn sub(self, rhs: Rhs) -> Self::Output {
    binary_operator(self, rhs, Token::Binary(Operation::Minus))
  }
}

impl<Rhs: Into<Expr>> Sub<Rhs> for &Expr {
  type Output = Expr;
  fn sub(self, rhs: Rhs) -> Self::Output {
    self.clone().sub(rhs)
  }
}
