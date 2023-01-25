use super::{Expr, Operation, Token};
pub use std::ops::Div;

impl<Rhs: Into<Expr>> Div<Rhs> for Expr {
  type Output = Self;
  fn div(self, rhs: Rhs) -> Self::Output {
    self.binary_operator(rhs, Token::Binary(Operation::Div))
  }
}

impl<Rhs: Into<Expr>> Div<Rhs> for &Expr {
  type Output = Expr;
  fn div(self, rhs: Rhs) -> Self::Output {
    self.clone().div(rhs)
  }
}
