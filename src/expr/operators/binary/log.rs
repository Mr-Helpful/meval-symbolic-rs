use super::{binary_operator, Expr, Token};

pub trait Log<Rhs = Self> {
  type Output;
  fn log(self, rhs: Rhs) -> Self::Output;
}
binary_trait_ref!(Log, log);

impl<Rhs: Into<Expr>> Log<Rhs> for Expr {
  type Output = Self;
  fn log(self, rhs: Rhs) -> Self::Output {
    binary_operator(self, rhs, Token::Func("log".into(), Some(2)))
  }
}
