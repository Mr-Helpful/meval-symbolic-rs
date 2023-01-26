use super::{unary_operator, Expr, Token};

pub trait Floor {
  type Output;
  fn floor(self) -> Self::Output;
}
unary_trait_ref!(Floor, floor);

impl Floor for Expr {
  type Output = Self;
  fn floor(self) -> Self::Output {
    unary_operator(self, Token::Func("floor".into(), Some(1)))
  }
}
