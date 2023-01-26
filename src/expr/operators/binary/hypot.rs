use super::{binary_operator, Expr, Token};

pub trait Hypot<Rhs = Self> {
  type Output;
  fn hypot(self, rhs: Rhs) -> Self::Output;
}
binary_trait_ref!(Hypot, hypot);

impl<Rhs: Into<Expr>> Hypot<Rhs> for Expr {
  type Output = Self;
  fn hypot(self, rhs: Rhs) -> Self::Output {
    binary_operator(self, rhs, Token::Func("hypot".into(), Some(2)))
  }
}
