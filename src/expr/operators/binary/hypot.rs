use super::Expr;
use crate::tokenizer::Token;

pub trait Hypot<Rhs = Self> {
  type Output;
  fn hypot(self, rhs: Rhs) -> Self::Output;
}
binary_trait_ref!(Hypot, hypot);

impl<Rhs: Into<Expr>> Hypot<Rhs> for Expr {
  type Output = Self;
  fn hypot(self, rhs: Rhs) -> Self::Output {
    self.binary_operator(rhs, Token::Func("hypot".into(), Some(2)))
  }
}
