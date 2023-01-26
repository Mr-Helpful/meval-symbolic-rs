use super::{unary_operator, Expr, Token};

pub trait Sqrt {
  type Output;
  fn sqrt(self) -> Self::Output;
}
unary_trait_ref!(Sqrt, sqrt);

impl Sqrt for Expr {
  type Output = Self;
  fn sqrt(self) -> Self::Output {
    unary_operator(self, Token::Func("sqrt".into(), Some(1)))
  }
}
