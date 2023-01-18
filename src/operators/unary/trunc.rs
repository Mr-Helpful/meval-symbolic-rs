use super::{Constant, Expr, UnaryOp};

pub trait Trunc {
  type Output;
  fn trunc(self) -> Self::Output;
}
unary_trait_ref!(Trunc, trunc);

impl<Num: Constant> Trunc for Expr<Num> {
  type Output = Self;
  fn trunc(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Trunc", &|x: Num| x.trunc()), Box::new(self))
  }
}
