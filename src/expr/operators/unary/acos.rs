use super::{unary_operator, Expr, Token};

pub trait ACos {
  type Output;
  fn acos(self) -> Self::Output;
}
unary_trait_ref!(ACos, acos);

impl ACos for Expr {
  type Output = Self;
  fn acos(self) -> Self::Output {
    unary_operator(self, Token::Func("acos".into(), Some(1)))
  }
}
