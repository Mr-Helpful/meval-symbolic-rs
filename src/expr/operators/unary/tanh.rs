use super::Expr;
use crate::tokenizer::Token;

pub trait Tanh {
  type Output;
  fn tanh(self) -> Self::Output;
}
unary_trait_ref!(Tanh, tanh);

impl Tanh for Expr {
  type Output = Self;
  fn tanh(self) -> Self::Output {
    self.unary_operator(Token::Func("tanh".into(), Some(1)))
  }
}
