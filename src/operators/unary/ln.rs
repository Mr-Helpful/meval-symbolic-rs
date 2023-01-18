use super::{Constant, Expr, UnaryOp};

pub trait Ln {
  type Output;
  fn ln(self) -> Self::Output;
}
unary_trait_ref!(Ln, ln);

impl<Num: Constant> Ln for Expr<Num> {
  type Output = Self;
  fn ln(self) -> Self::Output {
    Expr::Unary(UnaryOp::new("Ln", &|x: Num| x.ln()), Box::new(self))
  }
}
