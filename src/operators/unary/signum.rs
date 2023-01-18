use super::{Constant, Expr, UnaryOp};

pub trait Signum {
  type Output;
  fn signum(self) -> Self::Output;
}
unary_trait_ref!(Signum, signum);

impl<Num: Constant> Signum for Expr<Num> {
  type Output = Self;
  fn signum(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Signum", &|x: Num| x.signum()), Box::new(self))
  }
}
