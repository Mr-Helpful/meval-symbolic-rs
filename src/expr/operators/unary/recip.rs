use super::{Expr, Token};

pub trait Recip {
  type Output;
  fn recip(self) -> Self::Output;
}
unary_trait_ref!(Recip, recip);

impl Recip for Expr {
  type Output = Self;
  fn recip(self) -> Self::Output {
    self.unary_operator(Token::Func("recip".into(), Some(1)))
  }
}
