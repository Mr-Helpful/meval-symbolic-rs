use crate::tokenizer::Token;

use super::Expr;

macro_rules! unary_trait_ref {
  ($trait_name:ident, $method_name:ident) => {
    impl<Lhs: $trait_name + Clone> $trait_name for &Lhs {
      type Output = Lhs::Output;
      fn $method_name(self) -> Self::Output {
        self.clone().$method_name()
      }
    }
  };
}

impl Expr {
  /// Private helper function to construct a new expression from the current
  /// expression and a operation to apply.
  fn unary_operator(mut self, tkn: Token) -> Self {
    self.0.push(tkn);
    self
  }
}

pub use self::{
  abs::Abs, acos::ACos, acosh::ACosh, asin::ASin, asinh::ASinh, atan::ATan, atanh::ATanh,
  cbrt::Cbrt, ceil::Ceil, cos::Cos, cosh::Cosh, exp::Exp, exp2::Exp2, exp_m1::ExpM1, floor::Floor,
  fract::Fract, ln::Ln, ln_1p::Ln1p, log10::Log10, log2::Log2, neg::Neg, recip::Recip,
  round::Round, signum::Signum, sin::Sin, sinh::Sinh, sqrt::Sqrt, tan::Tan, tanh::Tanh,
  trunc::Trunc,
};

pub trait Unary<Output = Self>:
  Neg<Output = Output>
  + Abs<Output = Output>
  + ACos<Output = Output>
  + ACosh<Output = Output>
  + ASin<Output = Output>
  + ASinh<Output = Output>
  + ATan<Output = Output>
  + ATanh<Output = Output>
  + Cbrt<Output = Output>
  + Ceil<Output = Output>
  + Cos<Output = Output>
  + Cosh<Output = Output>
  + ExpM1<Output = Output>
  + Exp<Output = Output>
  + Exp2<Output = Output>
  + Floor<Output = Output>
  + Fract<Output = Output>
  + Ln1p<Output = Output>
  + Ln<Output = Output>
  + Log2<Output = Output>
  + Log10<Output = Output>
  + Recip<Output = Output>
  + Round<Output = Output>
  + Signum<Output = Output>
  + Sin<Output = Output>
  + Sinh<Output = Output>
  + Sqrt<Output = Output>
  + Tan<Output = Output>
  + Tanh<Output = Output>
  + Trunc<Output = Output>
{
}

impl<T, Output> Unary<Output> for T where
  T: Neg<Output = Output>
    + Abs<Output = Output>
    + ACos<Output = Output>
    + ACosh<Output = Output>
    + ASin<Output = Output>
    + ASinh<Output = Output>
    + ATan<Output = Output>
    + ATanh<Output = Output>
    + Cbrt<Output = Output>
    + Ceil<Output = Output>
    + Cos<Output = Output>
    + Cosh<Output = Output>
    + ExpM1<Output = Output>
    + Exp<Output = Output>
    + Exp2<Output = Output>
    + Floor<Output = Output>
    + Fract<Output = Output>
    + Ln1p<Output = Output>
    + Ln<Output = Output>
    + Log2<Output = Output>
    + Log10<Output = Output>
    + Recip<Output = Output>
    + Round<Output = Output>
    + Signum<Output = Output>
    + Sin<Output = Output>
    + Sinh<Output = Output>
    + Sqrt<Output = Output>
    + Tan<Output = Output>
    + Tanh<Output = Output>
    + Trunc<Output = Output>
{
}

mod abs;
mod acos;
mod acosh;
mod asin;
mod asinh;
mod atan;
mod atanh;
mod cbrt;
mod ceil;
mod cos;
mod cosh;
mod exp;
mod exp2;
mod exp_m1;
mod floor;
mod fract;
mod ln;
mod ln_1p;
mod log10;
mod log2;
mod neg;
mod recip;
mod round;
mod signum;
mod sin;
mod sinh;
mod sqrt;
mod tan;
mod tanh;
mod trunc;
