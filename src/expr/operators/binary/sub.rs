use super::{Expr, Operation, Token};
pub use std::ops::Sub;

impl<Rhs: Into<Expr>> Sub<Rhs> for Expr {
  type Output = Self;
  fn sub(self, rhs: Rhs) -> Self::Output {
    self.binary_operator(rhs, Token::Binary(Operation::Minus))
  }
}

impl<Rhs: Into<Expr>> Sub<Rhs> for &Expr {
  type Output = Expr;
  fn sub(self, rhs: Rhs) -> Self::Output {
    self.clone().sub(rhs)
  }
}
