use super::{Expr, Token};

pub trait Fract {
  type Output;
  fn fract(self) -> Self::Output;
}
unary_trait_ref!(Fract, fract);

impl Fract for Expr {
  type Output = Self;
  fn fract(self) -> Self::Output {
    self.unary_operator(Token::Func("fract".into(), Some(1)))
  }
}
