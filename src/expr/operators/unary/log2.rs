use super::{unary_operator, Expr, Token};

pub trait Log2 {
  type Output;
  fn log2(self) -> Self::Output;
}
unary_trait_ref!(Log2, log2);

impl Log2 for Expr {
  type Output = Self;
  fn log2(self) -> Self::Output {
    unary_operator(self, Token::Func("log2".into(), Some(1)))
  }
}
