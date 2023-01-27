use super::{Eqtn, Error};
use crate::expr::ParseError;
use std::str::FromStr;

impl FromStr for Eqtn {
  type Err = Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    use self::{Error::*, ParseError::*};
    let eqtn = s.split_once('=').ok_or(ParseError(MissingArgument))?;
    Ok(Eqtn(eqtn.0.parse()?, eqtn.1.parse()?))
  }
}
