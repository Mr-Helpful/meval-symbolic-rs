use self::{shunting_yard::to_rpn, tokenizer::tokenize};
use super::{Error, Expr, Operation, Token};
use std::str::FromStr;

pub mod shunting_yard;
pub mod tokenizer;

impl FromStr for Expr {
  type Err = Error;
  /// Constructs an expression by parsing a string.
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let tokens = tokenize(s)?;
    let rpn = to_rpn(tokens)?;

    Ok(Expr(rpn))
  }
}

#[cfg(feature = "serde")]
use super::Expr;
#[cfg(feature = "serde")]
pub mod de;
