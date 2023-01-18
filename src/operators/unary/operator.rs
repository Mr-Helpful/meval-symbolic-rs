use super::Constant;
use std::{fmt::Debug, fmt::Display, hash::Hash, rc::Rc};

#[derive(Clone)]
pub struct UnaryOp<Num: Constant>(String, Rc<dyn Fn(Num) -> Num>);

impl<Num: Constant> UnaryOp<Num> {
  pub(crate) fn new<F: Fn(Num) -> Num + 'static>(name: &str, f: F) -> Self {
    UnaryOp(name.to_owned(), Rc::new(f))
  }
}

impl<Num: Constant> Hash for UnaryOp<Num> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.0.hash(state)
  }
}

impl<Num: Constant> PartialEq for UnaryOp<Num> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<Num: Constant> Eq for UnaryOp<Num> {}

impl<Num: Constant> Debug for UnaryOp<Num> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("UnaryOp(\"{}\")", self.0))
  }
}

impl<Num: Constant> Display for UnaryOp<Num> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.0.as_str())
  }
}
