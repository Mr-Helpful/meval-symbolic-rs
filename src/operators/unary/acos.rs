use super::{Constant, Expr, UnaryOp};

pub trait ACos {
  type Output;
  fn acos(self) -> Self::Output;
}
unary_trait_ref!(ACos, acos);

impl<Num: Constant> ACos for Expr<Num> {
  type Output = Self;
  fn acos(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("ACos", &|x: Num| x.acos()), Box::new(self))
  }
}
