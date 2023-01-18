use super::Expr;
use crate::tokenizer::Token;

pub trait Exp2 {
  type Output;
  fn exp2(self) -> Self::Output;
}
unary_trait_ref!(Exp2, exp2);

impl Exp2 for Expr {
  type Output = Self;
  fn exp2(self) -> Self::Output {
    self.unary_operator(Token::Func("exp2".into(), Some(1)))
  }
}
