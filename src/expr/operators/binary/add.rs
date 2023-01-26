use super::{binary_operator, Expr, Operation, Token};
pub use std::ops::Add;

impl<Rhs: Into<Expr>> Add<Rhs> for Expr {
  type Output = Self;
  fn add(self, rhs: Rhs) -> Self::Output {
    binary_operator(self, rhs, Token::Binary(Operation::Plus))
  }
}

impl<Rhs: Into<Expr>> Add<Rhs> for &Expr {
  type Output = Expr;
  fn add(self, rhs: Rhs) -> Self::Output {
    self.clone().add(rhs)
  }
}
