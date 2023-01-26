use super::{Expr, Token};

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

/// Private helper function to construct a new expression from the current
/// expression, a reference to a mid and right side and a operation to apply.
fn ternary_operator(mut lhs: Expr, mid: impl Into<Expr>, rhs: impl Into<Expr>, tkn: Token) -> Expr {
  lhs.0.append(&mut mid.into().to_vec());
  lhs.0.append(&mut rhs.into().to_vec());
  lhs.0.push(tkn);
  lhs
}

impl<T, Mid, Rhs, Output> Ternary<Mid, Rhs, Output> for T where T: MulAdd<Mid, Rhs, Output = Output> {}

pub use self::mul_add::MulAdd;

pub trait Ternary<Mid = Self, Rhs = Self, Output = Self>:
  MulAdd<Mid, Rhs, Output = Output>
{
}

mod mul_add;
