use super::{Expr, Token};

pub trait ASinh {
  type Output;
  fn asinh(self) -> Self::Output;
}
unary_trait_ref!(ASinh, asinh);

impl ASinh for Expr {
  type Output = Self;
  fn asinh(self) -> Self::Output {
    self.unary_operator(Token::Func("asinh".into(), Some(1)))
  }
}
