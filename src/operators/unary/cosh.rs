use super::{Constant, Expr, UnaryOp};

pub trait Cosh {
  type Output;
  fn cosh(self) -> Self::Output;
}
unary_trait_ref!(Cosh, cosh);

impl<Num: Constant> Cosh for Expr<Num> {
  type Output = Self;
  fn cosh(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Cosh", &|x: Num| x.cosh()), Box::new(self))
  }
}
