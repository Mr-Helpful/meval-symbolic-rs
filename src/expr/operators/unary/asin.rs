use super::{unary_operator, Expr, Token};

pub trait ASin {
  type Output;
  fn asin(self) -> Self::Output;
}
unary_trait_ref!(ASin, asin);

impl ASin for Expr {
  type Output = Self;
  fn asin(self) -> Self::Output {
    unary_operator(self, Token::Func("asin".into(), Some(1)))
  }
}
