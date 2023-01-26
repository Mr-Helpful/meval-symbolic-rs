use super::{unary_operator, Expr, Token};

pub trait Trunc {
  type Output;
  fn trunc(self) -> Self::Output;
}
unary_trait_ref!(Trunc, trunc);

impl Trunc for Expr {
  type Output = Self;
  fn trunc(self) -> Self::Output {
    unary_operator(self, Token::Func("trunc".into(), Some(1)))
  }
}
