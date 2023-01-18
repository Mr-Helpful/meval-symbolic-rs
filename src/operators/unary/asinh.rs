use super::{Constant, Expr, UnaryOp};

pub trait ASinh {
  type Output;
  fn asinh(self) -> Self::Output;
}
unary_trait_ref!(ASinh, asinh);

impl<Num: Constant> ASinh for Expr<Num> {
  type Output = Self;
  fn asinh(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("ASinh", &|x: Num| x.asinh()), Box::new(self))
  }
}
