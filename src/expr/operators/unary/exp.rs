use super::Expr;
use crate::tokenizer::Token;

pub trait Exp {
  type Output;
  fn exp(self) -> Self::Output;
}
unary_trait_ref!(Exp, exp);

impl Exp for Expr {
  type Output = Self;
  fn exp(self) -> Self::Output {
    self.unary_operator(Token::Func("exp".into(), Some(1)))
  }
}
