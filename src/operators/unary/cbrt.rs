use super::{Constant, Expr, UnaryOp};

pub trait Cbrt {
  type Output;
  fn cbrt(self) -> Self::Output;
}
unary_trait_ref!(Cbrt, cbrt);

impl<Num: Constant> Cbrt for Expr<Num> {
  type Output = Self;
  fn cbrt(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Cbrt", &|x: Num| x.cbrt()), Box::new(self))
  }
}
