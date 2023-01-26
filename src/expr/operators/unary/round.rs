use super::{unary_operator, Expr, Token};

pub trait Round {
  type Output;
  fn round(self) -> Self::Output;
}
unary_trait_ref!(Round, round);

impl Round for Expr {
  type Output = Self;
  fn round(self) -> Self::Output {
    unary_operator(self, Token::Func("round".into(), Some(1)))
  }
}
