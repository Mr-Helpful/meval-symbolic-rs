use std::collections::HashSet;

pub use self::heuristics::{Heuristic, MaxNesting, OnVar};
use self::{
  heuristics::{Length, NoOccurences},
  rules::{Rule, Rules},
};
use super::{Eqtn, Expr, Token};

mod heuristics;
mod rules;

pub struct Solver<H: Heuristic<Eqtn> + OnVar = (MaxNesting, NoOccurences, Length)>(H, Rules);

impl<H: Heuristic<Eqtn> + OnVar> OnVar for Solver<H> {
  fn on(var: String) -> Self {
    Self(OnVar::on(var), Default::default())
  }
}

impl<H: Heuristic<Eqtn> + OnVar> Solver<H> {
  pub fn solve(&self, eqtn: Eqtn) -> Result<Eqtn, String> {
    const DEPTH: usize = 10;
    let mut eqtns = vec![eqtn];
    let seen = HashSet::<Eqtn>::new();

    // We'll be using a limited depth first search
    while let Some(eqtn) = eqtns.pop() {
      if eqtns.len() == DEPTH - 1 {
        continue; // we've reached the max depth, go back a step
      }
    }

    Ok(eqtns.pop().unwrap()) // TODO
  }
}
