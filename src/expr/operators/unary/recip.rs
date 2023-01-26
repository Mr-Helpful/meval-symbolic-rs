use super::{unary_operator, Expr, Token};

pub trait Recip {
  type Output;
  fn recip(self) -> Self::Output;
}
unary_trait_ref!(Recip, recip);

impl Recip for Expr {
  type Output = Self;
  fn recip(self) -> Self::Output {
    unary_operator(self, Token::Func("recip".into(), Some(1)))
  }
}
