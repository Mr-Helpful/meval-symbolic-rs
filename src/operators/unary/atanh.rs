use super::{Constant, Expr, UnaryOp};

pub trait ATanh {
  type Output;
  fn atanh(self) -> Self::Output;
}
unary_trait_ref!(ATanh, atanh);

impl<Num: Constant> ATanh for Expr<Num> {
  type Output = Self;
  fn atanh(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("ATanh", &|x: Num| x.atanh()), Box::new(self))
  }
}
