use super::Expr;
use crate::tokenizer::Token;

pub trait Cbrt {
  type Output;
  fn cbrt(self) -> Self::Output;
}
unary_trait_ref!(Cbrt, cbrt);

impl Cbrt for Expr {
  type Output = Self;
  fn cbrt(self) -> Self::Output {
    self.unary_operator(Token::Func("cbrt".into(), Some(1)))
  }
}
