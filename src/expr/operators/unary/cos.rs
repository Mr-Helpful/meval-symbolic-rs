use super::Expr;
use crate::tokenizer::Token;

pub trait Cos {
  type Output;
  fn cos(self) -> Self::Output;
}
unary_trait_ref!(Cos, cos);

impl Cos for Expr {
  type Output = Self;
  fn cos(self) -> Self::Output {
    self.unary_operator(Token::Func("cos".into(), Some(1)))
  }
}
