use super::Expr;
use crate::tokenizer::Token;

pub trait Sin {
  type Output;
  fn sin(self) -> Self::Output;
}
unary_trait_ref!(Sin, sin);

impl Sin for Expr {
  type Output = Self;
  fn sin(self) -> Self::Output {
    self.unary_operator(Token::Func("sin".into(), Some(1)))
  }
}
