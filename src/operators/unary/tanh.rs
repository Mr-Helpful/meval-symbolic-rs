use super::{Constant, Expr, UnaryOp};

pub trait Tanh {
  type Output;
  fn tanh(self) -> Self::Output;
}
unary_trait_ref!(Tanh, tanh);

impl<Num: Constant> Tanh for Expr<Num> {
  type Output = Self;
  fn tanh(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Tanh", &|x: Num| x.tanh()), Box::new(self))
  }
}
