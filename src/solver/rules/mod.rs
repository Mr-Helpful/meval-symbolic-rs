use self::{
  arithmetic::arithmetic_rules, exponential::exponential_rules, trigonometry::trigonometry_rules,
};
use super::{Eqtn, Error};

mod arithmetic;
mod exponential;
mod parser;
mod trigonometry;

/// Represents an axiom of the system.
///
/// We're only going to be using iff. rules here
/// as they comprise the majority of transformations on equations
#[derive(Clone, PartialEq)]
pub struct Rule(Eqtn, Eqtn);

impl Rule {
  /// As we're using iff. rules, it's safe to flip the implication
  pub(crate) fn flipped(self) -> Self {
    Self(self.1, self.0)
  }

  // pub fn apply(&self, eqtn: Eqtn) -> Eqtn {}
}

pub struct Rules(Vec<Rule>);

impl Default for Rules {
  fn default() -> Self {
    let mut res = vec![];
    res.extend(arithmetic_rules().0);
    res.extend(trigonometry_rules().0);
    res.extend(exponential_rules().0);
    Self(res)
  }
}

impl Rules {
  pub fn new() -> Self {
    Self(vec![])
  }

  /// Computes the symmetric closure of rules
  /// i.e. if `e1 <-> e2` exists, include `e2 <-> e1`
  pub(crate) fn symmetric_close(mut self) -> Self {
    self.0.extend(self.0.clone().into_iter().map(Rule::flipped));
    self
  }
}
