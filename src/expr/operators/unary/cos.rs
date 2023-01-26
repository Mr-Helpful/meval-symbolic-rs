use super::{unary_operator, Expr, Token};

pub trait Cos {
  type Output;
  fn cos(self) -> Self::Output;
}
unary_trait_ref!(Cos, cos);

impl Cos for Expr {
  type Output = Self;
  fn cos(self) -> Self::Output {
    unary_operator(self, Token::Func("cos".into(), Some(1)))
  }
}
