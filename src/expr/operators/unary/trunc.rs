use super::{Expr};
use crate::tokenizer::Token;

pub trait Trunc {
  type Output;
  fn trunc(self) -> Self::Output;
}
unary_trait_ref!(Trunc, trunc);

impl Trunc for Expr {
  type Output = Self;
  fn trunc(self) -> Self::Output {
    self.unary_operator(Token::Func("trunc".into(), Some(1)))
  }
}
