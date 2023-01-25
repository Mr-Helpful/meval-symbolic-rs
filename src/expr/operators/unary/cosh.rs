use super::{Expr, Token};

pub trait Cosh {
  type Output;
  fn cosh(self) -> Self::Output;
}
unary_trait_ref!(Cosh, cosh);

impl Cosh for Expr {
  type Output = Self;
  fn cosh(self) -> Self::Output {
    self.unary_operator(Token::Func("cosh".into(), Some(1)))
  }
}
