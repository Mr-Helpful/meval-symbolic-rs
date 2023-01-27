use super::{Error, Rule, Rules};
use crate::{expr::Token, Eqtn, Expr, ParseError};
use std::str::FromStr;

fn lhs_eqtn(lhs: Expr) -> Eqtn {
  use self::Token::Var;
  // rhs is a variable that cannot be represented as an expression
  // this ensures that there can be no collisions in variable identifiers
  Eqtn(lhs, Expr(vec![Var("".into())]))
}

impl FromStr for Rule {
  type Err = Error;

  /// A parse from a string to a rule
  ///
  /// This supports two forms of parsing:
  /// - equation -> equation, i.e. `x + y = z -> x = z - y`
  /// - expression -> expression, i.e. `(x + y) + z -> x + (y + z)`
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    use self::{Error::*, ParseError::*};
    let rule = s.split_once("<=>").ok_or(ParseError(MissingArgument))?;

    ({
      let lhs = rule.0.parse::<Eqtn>()?;
      let rhs = rule.1.parse::<Eqtn>()?;
      Ok::<Rule, Self::Err>(Rule(lhs, rhs))
    })
    .or({
      let lhs = rule.0.parse::<Expr>()?;
      let rhs = rule.1.parse::<Expr>()?;
      Ok(Rule(lhs_eqtn(lhs), lhs_eqtn(rhs)))
    })
  }
}

impl FromStr for Rules {
  type Err = Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut res = vec![];
    for line in s
      .split('\n')
      .filter(|line| (line != &"") & (!line.starts_with("//")))
    {
      res.push(line.parse()?)
    }
    Ok(Rules(res).symmetric_close())
  }
}
