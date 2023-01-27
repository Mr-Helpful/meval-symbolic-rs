use std::collections::HashSet;

pub use self::heuristics::{Heuristic, OnVar};
use self::{
  heuristics::{Length, MaxNesting, NoOccurences},
  rules::Rules,
};
use super::{Eqtn, Error, Expr, Token};

mod heuristics;
mod rules;

/// A Solver that attempts to isolate a single variable on either the left or
/// right hand side of an equation, i.e. `x = ...`
///
/// The solver operates primarily via a depth limited search, with both a
/// configurable heuristic over equations and a configurable depth limit.
/// The solver also requires a set of rules to apply to equations.
pub struct Solver<
  H: Heuristic<Eqtn> + OnVar = (MaxNesting, NoOccurences, Length),
  const DEPTH: usize = 10,
>(H, Rules);

impl<H: Heuristic<Eqtn> + OnVar> OnVar for Solver<H> {
  /// Defines a solver s.t. it will try to isolate `var` using the heuristic.
  fn on(var: String) -> Self {
    Self(OnVar::on(var), Default::default())
  }
}

impl<H: Heuristic<Eqtn> + OnVar, const DEPTH: usize> Solver<H, DEPTH> {
  pub fn solve(&self, eqtn: Eqtn) -> Result<Eqtn, String> {
    // TODO: this is __very much__ not finished, just use a DLS
    let mut eqtns = vec![eqtn];
    let seen = HashSet::<Eqtn>::new();

    // We'll be using a limited depth first search
    while let Some(eqtn) = eqtns.pop() {
      if eqtns.len() == DEPTH - 1 {
        continue; // we've reached the max depth, go back a step
      }
    }

    Ok(eqtns.pop().unwrap())
  }
}
