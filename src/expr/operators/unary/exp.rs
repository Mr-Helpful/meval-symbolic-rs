use super::{unary_operator, Expr, Token};

pub trait Exp {
  type Output;
  fn exp(self) -> Self::Output;
}
unary_trait_ref!(Exp, exp);

impl Exp for Expr {
  type Output = Self;
  fn exp(self) -> Self::Output {
    unary_operator(self, Token::Func("exp".into(), Some(1)))
  }
}
