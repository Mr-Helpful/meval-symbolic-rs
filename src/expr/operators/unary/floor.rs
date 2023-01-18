use super::Expr;
use crate::tokenizer::Token;

pub trait Floor {
  type Output;
  fn floor(self) -> Self::Output;
}
unary_trait_ref!(Floor, floor);

impl Floor for Expr {
  type Output = Self;
  fn floor(self) -> Self::Output {
    self.unary_operator(Token::Func("floor".into(), Some(1)))
  }
}
