use super::{Expr, Token};

pub trait Sqrt {
  type Output;
  fn sqrt(self) -> Self::Output;
}
unary_trait_ref!(Sqrt, sqrt);

impl Sqrt for Expr {
  type Output = Self;
  fn sqrt(self) -> Self::Output {
    self.unary_operator(Token::Func("sqrt".into(), Some(1)))
  }
}
