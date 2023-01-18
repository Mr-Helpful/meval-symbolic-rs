use super::super::Constant;
use std::{fmt::Debug, fmt::Display, hash::Hash, rc::Rc};

#[derive(Clone)]
pub struct TernaryOp<Num: Constant>(String, Rc<dyn Fn(Num, Num, Num) -> Num>);

impl<Num: Constant> TernaryOp<Num> {
  pub(crate) fn new<F: Fn(Num, Num, Num) -> Num + 'static>(name: &str, f: F) -> Self {
    TernaryOp(name.to_owned(), Rc::new(f))
  }
}

impl<Num: Constant> Hash for TernaryOp<Num> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.0.hash(state)
  }
}

impl<Num: Constant> PartialEq for TernaryOp<Num> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<Num: Constant> Eq for TernaryOp<Num> {}

impl<Num: Constant> Debug for TernaryOp<Num> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("TernaryOp({})", self.0))
  }
}

impl<Num: Constant> Display for TernaryOp<Num> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.0.as_str())
  }
}
