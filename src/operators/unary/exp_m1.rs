use super::{Constant, Expr, UnaryOp};

pub trait ExpM1 {
  type Output;
  fn exp_m1(self) -> Self::Output;
}
unary_trait_ref!(ExpM1, exp_m1);

impl<Num: Constant> ExpM1 for Expr<Num> {
  type Output = Self;
  fn exp_m1(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("ExpM1", &|x: Num| x.exp_m1()), Box::new(self))
  }
}
