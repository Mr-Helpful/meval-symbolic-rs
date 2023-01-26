use super::{unary_operator, Expr, Token};

pub trait Signum {
  type Output;
  fn signum(self) -> Self::Output;
}
unary_trait_ref!(Signum, signum);

impl Signum for Expr {
  type Output = Self;
  fn signum(self) -> Self::Output {
    unary_operator(self, Token::Func("signum".into(), Some(1)))
  }
}
