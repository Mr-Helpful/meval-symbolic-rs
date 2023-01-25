use super::{Expr, Token};

pub trait ATanh {
  type Output;
  fn atanh(self) -> Self::Output;
}
unary_trait_ref!(ATanh, atanh);

impl ATanh for Expr {
  type Output = Self;
  fn atanh(self) -> Self::Output {
    self.unary_operator(Token::Func("atanh".into(), Some(1)))
  }
}
