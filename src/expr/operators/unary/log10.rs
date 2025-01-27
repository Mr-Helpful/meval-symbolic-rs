use super::{unary_operator, Expr, Token};

pub trait Log10 {
  type Output;
  fn log10(self) -> Self::Output;
}
unary_trait_ref!(Log10, log10);

impl Log10 for Expr {
  type Output = Self;
  fn log10(self) -> Self::Output {
    unary_operator(self, Token::Func("log10".into(), Some(1)))
  }
}
