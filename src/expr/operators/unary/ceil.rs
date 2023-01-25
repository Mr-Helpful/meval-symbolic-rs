use super::{Expr, Token};

pub trait Ceil {
  type Output;
  fn ceil(self) -> Self::Output;
}
unary_trait_ref!(Ceil, ceil);

impl Ceil for Expr {
  type Output = Self;
  fn ceil(self) -> Self::Output {
    self.unary_operator(Token::Func("ceil".into(), Some(1)))
  }
}
