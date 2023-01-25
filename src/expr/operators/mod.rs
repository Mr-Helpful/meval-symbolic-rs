use super::{Expr, Operation, Token};

pub mod binary;
pub mod ternary;
pub mod unary;

use self::{binary::*, ternary::*, unary::*};

pub trait ExprNum<Mid = Self, Rhs = Self, Output = Self>:
  Unary<Output> + Binary<Rhs, Output> + Ternary<Mid, Rhs, Output>
{
}

impl ExprNum for Expr {}
