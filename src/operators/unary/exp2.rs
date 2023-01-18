use super::{Constant, Expr, UnaryOp};

pub trait Exp2 {
  type Output;
  fn exp2(self) -> Self::Output;
}
unary_trait_ref!(Exp2, exp2);

impl<Num: Constant> Exp2 for Expr<Num> {
  type Output = Self;
  fn exp2(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Exp2", &|x: Num| x.exp2()), Box::new(self))
  }
}
