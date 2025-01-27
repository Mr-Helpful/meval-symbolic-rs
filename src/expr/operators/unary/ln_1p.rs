use super::{unary_operator, Expr, Token};

pub trait Ln1p {
  type Output;
  fn ln_1p(self) -> Self::Output;
}
unary_trait_ref!(Ln1p, ln_1p);

impl Ln1p for Expr {
  type Output = Self;
  fn ln_1p(self) -> Self::Output {
    unary_operator(self, Token::Func("ln_1p".into(), Some(1)))
  }
}
