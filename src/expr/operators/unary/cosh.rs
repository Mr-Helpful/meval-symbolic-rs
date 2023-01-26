use super::{unary_operator, Expr, Token};

pub trait Cosh {
  type Output;
  fn cosh(self) -> Self::Output;
}
unary_trait_ref!(Cosh, cosh);

impl Cosh for Expr {
  type Output = Self;
  fn cosh(self) -> Self::Output {
    unary_operator(self, Token::Func("cosh".into(), Some(1)))
  }
}
