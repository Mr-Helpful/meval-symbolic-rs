use super::{Expr, Token};

pub trait ATan {
  type Output;
  fn atan(self) -> Self::Output;
}
unary_trait_ref!(ATan, atan);

impl ATan for Expr {
  type Output = Self;
  fn atan(self) -> Self::Output {
    self.unary_operator(Token::Func("atan".into(), Some(1)))
  }
}
