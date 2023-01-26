use super::{unary_operator, Expr, Token};

pub trait ATanh {
  type Output;
  fn atanh(self) -> Self::Output;
}
unary_trait_ref!(ATanh, atanh);

impl ATanh for Expr {
  type Output = Self;
  fn atanh(self) -> Self::Output {
    unary_operator(self, Token::Func("atanh".into(), Some(1)))
  }
}
