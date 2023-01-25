use super::{Expr, Operation, Token};

pub trait Powf<Rhs = Self> {
  type Output;
  fn powf(self, rhs: Rhs) -> Self::Output;
}
binary_trait_ref!(Powf, powf);

impl<Rhs: Into<Expr>> Powf<Rhs> for Expr {
  type Output = Self;
  fn powf(self, rhs: Rhs) -> Self::Output {
    self.binary_operator(rhs, Token::Binary(Operation::Pow))
  }
}
