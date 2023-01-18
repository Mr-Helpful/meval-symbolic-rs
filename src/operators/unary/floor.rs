use super::{Constant, Expr, UnaryOp};

pub trait Floor {
  type Output;
  fn floor(self) -> Self::Output;
}
unary_trait_ref!(Floor, floor);

impl<Num: Constant> Floor for Expr<Num> {
  type Output = Self;
  fn floor(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Floor", &|x: Num| x.floor()), Box::new(self))
  }
}
