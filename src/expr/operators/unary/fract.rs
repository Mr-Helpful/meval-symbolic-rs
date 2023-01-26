use super::{unary_operator, Expr, Token};

pub trait Fract {
  type Output;
  fn fract(self) -> Self::Output;
}
unary_trait_ref!(Fract, fract);

impl Fract for Expr {
  type Output = Self;
  fn fract(self) -> Self::Output {
    unary_operator(self, Token::Func("fract".into(), Some(1)))
  }
}
