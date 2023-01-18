use super::{Constant, Expr, UnaryOp};

pub trait ASin {
  type Output;
  fn asin(self) -> Self::Output;
}
unary_trait_ref!(ASin, asin);

impl<Num: Constant> ASin for Expr<Num> {
  type Output = Self;
  fn asin(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("ASin", &|x: Num| x.asin()), Box::new(self))
  }
}
