use super::Constant;
use std::{fmt::Debug, fmt::Display, hash::Hash, rc::Rc};

#[derive(Clone)]
pub struct BinaryOp<Num: Constant>(String, Rc<dyn Fn(Num, Num) -> Num>);

impl<Num: Constant> Debug for BinaryOp<Num> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("BinaryOp(\"{}\")", self.0))
  }
}

impl<Num: Constant> Hash for BinaryOp<Num> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.0.hash(state)
  }
}

impl<Num: Constant> PartialEq for BinaryOp<Num> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<Num: Constant> Eq for BinaryOp<Num> {}

impl<Num: Constant> BinaryOp<Num> {
  pub(crate) fn new<F: Fn(Num, Num) -> Num + 'static>(name: &str, f: F) -> Self {
    BinaryOp(name.to_owned(), Rc::new(f))
  }
}

impl<Num: Constant> Display for BinaryOp<Num> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.0.as_str())
  }
}
