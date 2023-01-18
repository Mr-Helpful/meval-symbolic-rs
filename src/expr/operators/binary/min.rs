use super::Expr;
use crate::tokenizer::Token;

pub trait Min<Rhs = Self> {
  type Output;
  fn min(self, rhs: Rhs) -> Self::Output;
}
binary_trait_ref!(Min, min);

impl<Rhs: Into<Expr>> Min<Rhs> for Expr {
  type Output = Self;
  fn min(self, rhs: Rhs) -> Self::Output {
    self.binary_operator(rhs, Token::Func("min".into(), Some(2)))
  }
}
