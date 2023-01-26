use super::{unary_operator, Expr, Token};

pub trait ACosh {
  type Output;
  fn acosh(self) -> Self::Output;
}
unary_trait_ref!(ACosh, acosh);

impl ACosh for Expr {
  type Output = Self;
  fn acosh(self) -> Self::Output {
    unary_operator(self, Token::Func("acosh".into(), Some(1)))
  }
}
