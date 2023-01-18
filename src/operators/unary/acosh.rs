use super::{Constant, Expr, UnaryOp};

pub trait ACosh {
  type Output;
  fn acosh(self) -> Self::Output;
}
unary_trait_ref!(ACosh, acosh);

impl<Num: Constant> ACosh for Expr<Num> {
  type Output = Self;
  fn acosh(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("ACosh", &|x: Num| x.acosh()), Box::new(self))
  }
}
