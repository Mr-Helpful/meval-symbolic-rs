use super::Eqtn;

pub struct Rule(Eqtn, Eqtn);

pub struct Rules(Vec<Rule>);

impl Default for Rules {
  fn default() -> Self {
    Self(vec![])
  }
}
