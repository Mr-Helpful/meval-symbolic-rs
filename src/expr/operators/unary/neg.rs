use super::{Expr, Operation, Token};
pub use std::ops::Neg;

impl Neg for Expr {
  type Output = Self;
  fn neg(self) -> Self::Output {
    self.unary_operator(Token::Unary(Operation::Minus))
  }
}

impl Neg for &Expr {
  type Output = Expr;
  fn neg(self) -> Self::Output {
    self.clone().neg()
  }
}
