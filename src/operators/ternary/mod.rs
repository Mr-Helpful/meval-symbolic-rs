use super::{Constant, Expr};

mod operator;

macro_rules! ternary_trait_ref {
  ($trait_name:ident, $method_name:ident) => {
    impl<Mid, Rhs, Lhs: $trait_name<Mid, Rhs> + Clone> $trait_name<Mid, Rhs> for &Lhs {
      type Output = Lhs::Output;
      fn $method_name(self, mid: Mid, rhs: Rhs) -> Self::Output {
        self.clone().$method_name(mid, rhs)
      }
    }
  };
}

impl<T, Mid, Rhs, Output> Ternary<Mid, Rhs, Output> for T where T: MulAdd<Mid, Rhs, Output = Output> {}

pub use self::{mul_add::MulAdd, operator::TernaryOp};

pub trait Ternary<Mid = Self, Rhs = Self, Output = Self>:
  MulAdd<Mid, Rhs, Output = Output>
{
}

mod mul_add;
