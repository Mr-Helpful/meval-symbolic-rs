use super::{unary_operator, Expr, Token};

pub trait ASinh {
  type Output;
  fn asinh(self) -> Self::Output;
}
unary_trait_ref!(ASinh, asinh);

impl ASinh for Expr {
  type Output = Self;
  fn asinh(self) -> Self::Output {
    unary_operator(self, Token::Func("asinh".into(), Some(1)))
  }
}
