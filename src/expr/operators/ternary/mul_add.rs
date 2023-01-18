use super::Expr;
use crate::tokenizer::Token;

pub trait MulAdd<Mid, Rhs> {
  type Output;
  fn mul_add(self, mid: Mid, rhs: Rhs) -> Self::Output;
}
ternary_trait_ref!(MulAdd, mul_add);

impl<Mid: Into<Expr>, Rhs: Into<Expr>> MulAdd<Mid, Rhs> for Expr {
  type Output = Self;
  fn mul_add(self, mid: Mid, rhs: Rhs) -> Self::Output {
    self.ternary_operator(mid, rhs, Token::Func("mul_add".into(), Some(3)))
  }
}
