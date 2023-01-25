use super::{Expr, Token};

pub trait Sinh {
  type Output;
  fn sinh(self) -> Self::Output;
}
unary_trait_ref!(Sinh, sinh);

impl Sinh for Expr {
  type Output = Self;
  fn sinh(self) -> Self::Output {
    self.unary_operator(Token::Func("sinh".into(), Some(1)))
  }
}
