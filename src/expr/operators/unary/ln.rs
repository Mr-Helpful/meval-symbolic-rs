use super::Expr;
use crate::tokenizer::Token;

pub trait Ln {
  type Output;
  fn ln(self) -> Self::Output;
}
unary_trait_ref!(Ln, ln);

impl Ln for Expr {
  type Output = Self;
  fn ln(self) -> Self::Output {
    self.unary_operator(Token::Func("ln".into(), Some(1)))
  }
}
