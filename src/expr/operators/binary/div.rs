use super::{binary_operator, Expr, Operation, Token};
pub use std::ops::Div;

impl<Rhs: Into<Expr>> Div<Rhs> for Expr {
  type Output = Self;
  fn div(self, rhs: Rhs) -> Self::Output {
    binary_operator(self, rhs, Token::Binary(Operation::Div))
  }
}

impl<Rhs: Into<Expr>> Div<Rhs> for &Expr {
  type Output = Expr;
  fn div(self, rhs: Rhs) -> Self::Output {
    self.clone().div(rhs)
  }
}
