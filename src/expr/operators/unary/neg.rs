use super::{unary_operator, Expr, Operation, Token};
pub use std::ops::Neg;

impl Neg for Expr {
  type Output = Self;
  fn neg(self) -> Self::Output {
    unary_operator(self, Token::Unary(Operation::Minus))
  }
}

impl Neg for &Expr {
  type Output = Expr;
  fn neg(self) -> Self::Output {
    self.clone().neg()
  }
}
