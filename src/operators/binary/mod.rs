use super::{Constant, Expr};

mod operator;

macro_rules! binary_trait_ref {
  ($trait_name:ident, $method_name:ident) => {
    impl<Rhs, Lhs: $trait_name<Rhs> + Clone> $trait_name<Rhs> for &Lhs {
      type Output = Lhs::Output;
      fn $method_name(self, rhs: Rhs) -> Self::Output {
        self.clone().$method_name(rhs)
      }
    }
  };
}

pub use self::{
  abs_sub::AbsSub, atan2::ATan2, hypot::Hypot, log::Log, max::Max, min::Min, operator::BinaryOp,
  powf::Powf,
};

pub trait Binary<Rhs = Self, Output = Self>:
  AbsSub<Rhs, Output = Output>
  + ATan2<Rhs, Output = Output>
  + Hypot<Rhs, Output = Output>
  + Max<Rhs, Output = Output>
  + Min<Rhs, Output = Output>
  + Powf<Rhs, Output = Output>
{
}

impl<T, Rhs, Output> Binary<Rhs, Output> for T where
  T: AbsSub<Rhs, Output = Output>
    + ATan2<Rhs, Output = Output>
    + Hypot<Rhs, Output = Output>
    + Max<Rhs, Output = Output>
    + Min<Rhs, Output = Output>
    + Powf<Rhs, Output = Output>
{
}

mod abs_sub;
mod add;
mod atan2;
mod div;
mod hypot;
mod log;
mod max;
mod min;
mod mul;
mod powf;
mod rem;
mod sub;
