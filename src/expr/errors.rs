use std::fmt::{self, Display, Formatter};

/// Function evaluation error.
#[derive(Debug, Clone, PartialEq)]
pub enum FuncEvalError {
  TooFewArguments,
  TooManyArguments,
  NumberArgs(usize),
  UnknownFunction,
}

impl Display for FuncEvalError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match *self {
      FuncEvalError::UnknownFunction => write!(f, "Unknown function"),
      FuncEvalError::NumberArgs(i) => write!(f, "Expected {} arguments", i),
      FuncEvalError::TooFewArguments => write!(f, "Too few arguments"),
      FuncEvalError::TooManyArguments => write!(f, "Too many arguments"),
    }
  }
}
