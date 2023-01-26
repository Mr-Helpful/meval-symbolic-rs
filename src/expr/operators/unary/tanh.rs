use super::{unary_operator, Expr, Token};

pub trait Tanh {
  type Output;
  fn tanh(self) -> Self::Output;
}
unary_trait_ref!(Tanh, tanh);

impl Tanh for Expr {
  type Output = Self;
  fn tanh(self) -> Self::Output {
    unary_operator(self, Token::Func("tanh".into(), Some(1)))
  }
}
