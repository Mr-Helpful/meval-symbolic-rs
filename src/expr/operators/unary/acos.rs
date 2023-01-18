use super::Expr;
use crate::tokenizer::Token;

pub trait ACos {
  type Output;
  fn acos(self) -> Self::Output;
}
unary_trait_ref!(ACos, acos);

impl ACos for Expr {
  type Output = Self;
  fn acos(self) -> Self::Output {
    self.unary_operator(Token::Func("acos".into(), Some(1)))
  }
}
