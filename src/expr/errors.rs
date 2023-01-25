use std::error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

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

impl error::Error for FuncEvalError {
  fn description(&self) -> &str {
    match *self {
      FuncEvalError::UnknownFunction => "unknown function",
      FuncEvalError::NumberArgs(_) => "wrong number of function arguments",
      FuncEvalError::TooFewArguments => "too few function arguments",
      FuncEvalError::TooManyArguments => "too many function arguments",
    }
  }
}
