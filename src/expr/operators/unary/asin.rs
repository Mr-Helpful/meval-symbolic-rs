use super::{Expr, Token};

pub trait ASin {
  type Output;
  fn asin(self) -> Self::Output;
}
unary_trait_ref!(ASin, asin);

impl ASin for Expr {
  type Output = Self;
  fn asin(self) -> Self::Output {
    self.unary_operator(Token::Func("asin".into(), Some(1)))
  }
}
