use super::{Constant, Expr, UnaryOp};

pub trait Ln1p {
  type Output;
  fn ln_1p(self) -> Self::Output;
}
unary_trait_ref!(Ln1p, ln_1p);

impl<Num: Constant> Ln1p for Expr<Num> {
  type Output = Self;
  fn ln_1p(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Ln1p", &|x: Num| x.ln_1p()), Box::new(self))
  }
}
