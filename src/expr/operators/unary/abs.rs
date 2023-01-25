use super::{Expr, Token};

pub trait Abs {
  type Output;
  fn abs(self) -> Self::Output;
}
unary_trait_ref!(Abs, abs);

impl Abs for Expr {
  type Output = Self;
  fn abs(self) -> Self::Output {
    self.unary_operator(Token::Func("abs".into(), Some(1)))
  }
}
