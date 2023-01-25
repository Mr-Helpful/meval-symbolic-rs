use super::{Expr, Operation, Token};
pub use std::ops::Add;

impl<Rhs: Into<Expr>> Add<Rhs> for Expr {
  type Output = Self;
  fn add(self, rhs: Rhs) -> Self::Output {
    self.binary_operator(rhs, Token::Binary(Operation::Plus))
  }
}

impl<Rhs: Into<Expr>> Add<Rhs> for &Expr {
  type Output = Expr;
  fn add(self, rhs: Rhs) -> Self::Output {
    self.clone().add(rhs)
  }
}
