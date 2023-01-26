use super::{unary_operator, Expr, Token};

pub trait Ln {
  type Output;
  fn ln(self) -> Self::Output;
}
unary_trait_ref!(Ln, ln);

impl Ln for Expr {
  type Output = Self;
  fn ln(self) -> Self::Output {
    unary_operator(self, Token::Func("ln".into(), Some(1)))
  }
}
