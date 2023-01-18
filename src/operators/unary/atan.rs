use super::{Constant, Expr, UnaryOp};

pub trait ATan {
  type Output;
  fn atan(self) -> Self::Output;
}
unary_trait_ref!(ATan, atan);

impl<Num: Constant> ATan for Expr<Num> {
  type Output = Self;
  fn atan(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("ATan", &|x: Num| x.atan()), Box::new(self))
  }
}
