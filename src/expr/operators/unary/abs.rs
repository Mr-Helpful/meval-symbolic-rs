use super::{unary_operator, Expr, Token};

pub trait Abs {
  type Output;
  fn abs(self) -> Self::Output;
}
unary_trait_ref!(Abs, abs);

impl Abs for Expr {
  type Output = Self;
  fn abs(self) -> Self::Output {
    unary_operator(self, Token::Func("abs".into(), Some(1)))
  }
}
