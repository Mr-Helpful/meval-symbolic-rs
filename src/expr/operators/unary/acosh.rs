use super::{Expr, Token};

pub trait ACosh {
  type Output;
  fn acosh(self) -> Self::Output;
}
unary_trait_ref!(ACosh, acosh);

impl ACosh for Expr {
  type Output = Self;
  fn acosh(self) -> Self::Output {
    self.unary_operator(Token::Func("acosh".into(), Some(1)))
  }
}
