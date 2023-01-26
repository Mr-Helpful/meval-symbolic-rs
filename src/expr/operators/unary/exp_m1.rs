use super::{unary_operator, Expr, Token};

pub trait ExpM1 {
  type Output;
  fn exp_m1(self) -> Self::Output;
}
unary_trait_ref!(ExpM1, exp_m1);

impl ExpM1 for Expr {
  type Output = Self;
  fn exp_m1(self) -> Self::Output {
    unary_operator(self, Token::Func("exp_m1".into(), Some(1)))
  }
}
