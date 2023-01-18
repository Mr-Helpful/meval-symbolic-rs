use super::{Constant, Expr, UnaryOp};

pub trait Recip {
  type Output;
  fn recip(self) -> Self::Output;
}
unary_trait_ref!(Recip, recip);

impl<Num: Constant> Recip for Expr<Num> {
  type Output = Self;
  fn recip(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Recip", &|x: Num| x.recip()), Box::new(self))
  }
}
