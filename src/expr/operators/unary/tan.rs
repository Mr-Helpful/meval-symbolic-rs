use super::{unary_operator, Expr, Token};

pub trait Tan {
  type Output;
  fn tan(self) -> Self::Output;
}
unary_trait_ref!(Tan, tan);

impl Tan for Expr {
  type Output = Self;
  fn tan(self) -> Self::Output {
    unary_operator(self, Token::Func("tan".into(), Some(1)))
  }
}
