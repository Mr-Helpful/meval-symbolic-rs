use super::Expr;
use crate::tokenizer::Token;

pub trait ExpM1 {
  type Output;
  fn exp_m1(self) -> Self::Output;
}
unary_trait_ref!(ExpM1, exp_m1);

impl ExpM1 for Expr {
  type Output = Self;
  fn exp_m1(self) -> Self::Output {
    self.unary_operator(Token::Func("exp_m1".into(), Some(1)))
  }
}
