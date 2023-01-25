use super::{Expr, Operation, Token};

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

impl Expr {
  /// Private helper function to construct a new expression from the current
  /// expression, a reference to a right hand side and a operation to apply.
  fn binary_operator(mut self, rhs: impl Into<Expr>, tkn: Token) -> Self {
    self.0.append(&mut rhs.into().to_vec());
    self.0.push(tkn);
    self
  }
}

pub use self::{
  add::Add, atan2::ATan2, div::Div, hypot::Hypot, log::Log, max::Max, min::Min, mul::Mul,
  powf::Powf, rem::Rem, sub::Sub,
};

pub trait Binary<Rhs = Self, Output = Self>:
  Add<Rhs, Output = Output>
  + Sub<Rhs, Output = Output>
  + Div<Rhs, Output = Output>
  + Mul<Rhs, Output = Output>
  + Rem<Rhs, Output = Output>
  + ATan2<Rhs, Output = Output>
  + Hypot<Rhs, Output = Output>
  + Max<Rhs, Output = Output>
  + Min<Rhs, Output = Output>
  + Powf<Rhs, Output = Output>
{
}

impl<T, Rhs, Output> Binary<Rhs, Output> for T where
  T: Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Rem<Rhs, Output = Output>
    + ATan2<Rhs, Output = Output>
    + Hypot<Rhs, Output = Output>
    + Max<Rhs, Output = Output>
    + Min<Rhs, Output = Output>
    + Powf<Rhs, Output = Output>
{
}

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
