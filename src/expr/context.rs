use fnv::FnvHashMap;
use std::error;
use std::fmt::{self, Display, Formatter};
use std::rc::Rc;

type ContextHashMap<K, V> = FnvHashMap<K, V>;

/// A trait of a source of variables (and constants) and functions for substitution into an
/// evaluated expression.
///
/// A simplest way to create a custom context provider is to use [`Context`](struct.Context.html).
///
/// ## Advanced usage
///
/// Alternatively, values of variables/constants can be specified by tuples `(name, value)`,
/// `std::collections::HashMap` or `std::collections::BTreeMap`.
///
/// ```rust
/// use meval::{ContextProvider, Context};
///
/// let mut ctx = Context::new(); // built-ins
/// ctx.var("x", 2.); // insert a new variable
/// assert_eq!(ctx.get_var("pi"), Some(std::f64::consts::PI));
///
/// let myvars = ("x", 2.); // tuple as a ContextProvider
/// assert_eq!(myvars.get_var("x"), Some(2f64));
///
/// // HashMap as a ContextProvider
/// let mut varmap = std::collections::HashMap::new();
/// varmap.insert("x", 2.);
/// varmap.insert("y", 3.);
/// assert_eq!(varmap.get_var("x"), Some(2f64));
/// assert_eq!(varmap.get_var("z"), None);
/// ```
///
/// Custom functions can be also defined.
///
/// ```rust
/// use meval::{ContextProvider, Context};
///
/// let mut ctx = Context::new(); // built-ins
/// ctx.func2("phi", |x, y| x / (y * y));
///
/// assert_eq!(ctx.eval_func("phi", &[2., 3.]), Ok(2. / (3. * 3.)));
/// ```
///
/// A `ContextProvider` can be built by combining other contexts:
///
/// ```rust
/// use meval::Context;
///
/// let bins = Context::new(); // built-ins
/// let mut funcs = Context::empty(); // empty context
/// funcs.func2("phi", |x, y| x / (y * y));
/// let myvars = ("x", 2.);
///
/// // contexts can be combined using tuples
/// let ctx = ((myvars, bins), funcs); // first context has preference if there's duplicity
///
/// assert_eq!(meval::eval_str_with_context("x * pi + phi(1., 2.)", ctx).unwrap(), 2. *
///             std::f64::consts::PI + 1. / (2. * 2.));
/// ```
///
pub trait ContextProvider {
  fn get_var(&self, _: &str) -> Option<f64> {
    None
  }
  fn eval_func(&self, _: &str, _: &[f64]) -> Result<f64, FuncEvalError> {
    Err(FuncEvalError::UnknownFunction)
  }
}

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

#[doc(hidden)]
pub fn max_array(xs: &[f64]) -> f64 {
  xs.iter().fold(::std::f64::NEG_INFINITY, |m, &x| m.max(x))
}

#[doc(hidden)]
pub fn min_array(xs: &[f64]) -> f64 {
  xs.iter().fold(::std::f64::INFINITY, |m, &x| m.min(x))
}

/// Returns the built-in constants and functions in a form that can be used as a `ContextProvider`.
#[doc(hidden)]
pub fn builtin<'a>() -> Context<'a> {
  // TODO: cache this (lazy_static)
  Context::new()
}

impl<'a, T: ContextProvider> ContextProvider for &'a T {
  fn get_var(&self, name: &str) -> Option<f64> {
    (&**self).get_var(name)
  }

  fn eval_func(&self, name: &str, args: &[f64]) -> Result<f64, FuncEvalError> {
    (&**self).eval_func(name, args)
  }
}

impl<'a, T: ContextProvider> ContextProvider for &'a mut T {
  fn get_var(&self, name: &str) -> Option<f64> {
    (&**self).get_var(name)
  }

  fn eval_func(&self, name: &str, args: &[f64]) -> Result<f64, FuncEvalError> {
    (&**self).eval_func(name, args)
  }
}

impl<T: ContextProvider, S: ContextProvider> ContextProvider for (T, S) {
  fn get_var(&self, name: &str) -> Option<f64> {
    self.0.get_var(name).or_else(|| self.1.get_var(name))
  }
  fn eval_func(&self, name: &str, args: &[f64]) -> Result<f64, FuncEvalError> {
    match self.0.eval_func(name, args) {
      Err(FuncEvalError::UnknownFunction) => self.1.eval_func(name, args),
      e => e,
    }
  }
}

impl<S: AsRef<str>> ContextProvider for (S, f64) {
  fn get_var(&self, name: &str) -> Option<f64> {
    if self.0.as_ref() == name {
      Some(self.1)
    } else {
      None
    }
  }
}

/// `std::collections::HashMap` of variables.
impl<S> ContextProvider for std::collections::HashMap<S, f64>
where
  S: std::hash::Hash + std::cmp::Eq + std::borrow::Borrow<str>,
{
  fn get_var(&self, name: &str) -> Option<f64> {
    self.get(name).cloned()
  }
}

/// `std::collections::BTreeMap` of variables.
impl<S> ContextProvider for std::collections::BTreeMap<S, f64>
where
  S: std::cmp::Ord + std::borrow::Borrow<str>,
{
  fn get_var(&self, name: &str) -> Option<f64> {
    self.get(name).cloned()
  }
}

impl<C: ContextProvider> ContextProvider for Vec<C> {
  fn get_var(&self, name: &str) -> Option<f64> {
    self.iter().find_map(|ctx| ctx.get_var(name))
  }
  fn eval_func(&self, name: &str, args: &[f64]) -> Result<f64, FuncEvalError> {
    self
      .iter()
      .fold(Err(FuncEvalError::UnknownFunction), |res, ctx| {
        res.or(ctx.eval_func(name, args))
      })
  }
}

impl<C: ContextProvider, const N: usize> ContextProvider for [C; N] {
  fn get_var(&self, name: &str) -> Option<f64> {
    self.iter().find_map(|ctx| ctx.get_var(name))
  }
  fn eval_func(&self, name: &str, args: &[f64]) -> Result<f64, FuncEvalError> {
    self
      .iter()
      .fold(Err(FuncEvalError::UnknownFunction), |res, ctx| {
        res.or(ctx.eval_func(name, args))
      })
  }
}

/// A structure for storing variables/constants and functions to be used in an expression.
///
/// # Example
///
/// ```rust
/// use meval::{eval_str_with_context, Context};
///
/// let mut ctx = Context::new(); // builtins
/// ctx.var("x", 3.)
///    .func("f", |x| 2. * x)
///    .funcn("sum", |xs| xs.iter().sum(), ..);
///
/// assert_eq!(eval_str_with_context("pi + sum(1., 2.) + f(x)", &ctx),
///            Ok(std::f64::consts::PI + 1. + 2. + 2. * 3.));
/// ```
#[derive(Clone)]
pub struct Context<'a> {
  pub(crate) vars: ContextHashMap<String, f64>,
  pub(crate) funcs: ContextHashMap<String, GuardedFunc<'a>>,
}

impl<'a> Context<'a> {
  /// Creates a context with built-in constants and functions.
  pub fn new() -> Context<'a> {
    thread_local!(static DEFAULT_CONTEXT: Context<'static> = {
        let mut ctx = Context::empty();
        ctx.var("pi", std::f64::consts::PI);
        ctx.var("e", std::f64::consts::E);

        ctx.func("sqrt", f64::sqrt);
        ctx.func("exp", f64::exp);
        ctx.func("ln", f64::ln);
        ctx.func("log10", f64::log10);
        ctx.func("abs", f64::abs);
        ctx.func("sin", f64::sin);
        ctx.func("cos", f64::cos);
        ctx.func("tan", f64::tan);
        ctx.func("asin", f64::asin);
        ctx.func("acos", f64::acos);
        ctx.func("atan", f64::atan);
        ctx.func("sinh", f64::sinh);
        ctx.func("cosh", f64::cosh);
        ctx.func("tanh", f64::tanh);
        ctx.func("asinh", f64::asinh);
        ctx.func("acosh", f64::acosh);
        ctx.func("atanh", f64::atanh);
        ctx.func("floor", f64::floor);
        ctx.func("ceil", f64::ceil);
        ctx.func("round", f64::round);
        ctx.func("signum", f64::signum);
        ctx.func2("atan2", f64::atan2);
        ctx.funcn("max", max_array, 1..);
        ctx.funcn("min", min_array, 1..);

        ctx.func("cbrt", f64::cbrt);
        ctx.func("exp_m1", f64::exp_m1);
        ctx.func("exp2", f64::exp2);
        ctx.func("fract", f64::fract);
        ctx.func("ln_1p", f64::ln_1p);
        ctx.func("log2", f64::log2);
        ctx.func("recip", f64::recip);
        ctx.func("trunc", f64::trunc);
        ctx.func2("hypot", f64::hypot);
        ctx.func2("log", f64::log);
        ctx.func3("mul_add", f64::mul_add);

        ctx
    });

    DEFAULT_CONTEXT.with(|ctx| ctx.clone())
  }

  /// Creates an empty contexts.
  pub fn empty() -> Context<'a> {
    Context {
      vars: ContextHashMap::default(),
      funcs: ContextHashMap::default(),
    }
  }

  /// Adds a new variable/constant.
  pub fn var<S: Into<String>>(&mut self, var: S, value: f64) -> &mut Self {
    self.vars.insert(var.into(), value);
    self
  }

  /// Adds a new function of one argument.
  pub fn func<S, F>(&mut self, name: S, func: F) -> &mut Self
  where
    S: Into<String>,
    F: Fn(f64) -> f64 + 'a,
  {
    self.funcs.insert(
      name.into(),
      Rc::new(move |args: &[f64]| {
        if args.len() == 1 {
          Ok(func(args[0]))
        } else {
          Err(FuncEvalError::NumberArgs(1))
        }
      }),
    );
    self
  }

  /// Adds a new function of two arguments.
  pub fn func2<S, F>(&mut self, name: S, func: F) -> &mut Self
  where
    S: Into<String>,
    F: Fn(f64, f64) -> f64 + 'a,
  {
    self.funcs.insert(
      name.into(),
      Rc::new(move |args: &[f64]| {
        if args.len() == 2 {
          Ok(func(args[0], args[1]))
        } else {
          Err(FuncEvalError::NumberArgs(2))
        }
      }),
    );
    self
  }

  /// Adds a new function of three arguments.
  pub fn func3<S, F>(&mut self, name: S, func: F) -> &mut Self
  where
    S: Into<String>,
    F: Fn(f64, f64, f64) -> f64 + 'a,
  {
    self.funcs.insert(
      name.into(),
      Rc::new(move |args: &[f64]| {
        if args.len() == 3 {
          Ok(func(args[0], args[1], args[2]))
        } else {
          Err(FuncEvalError::NumberArgs(3))
        }
      }),
    );
    self
  }

  /// Adds a new function of a variable number of arguments.
  ///
  /// `n_args` specifies the allowed number of variables by giving an exact number `n` or a range
  /// `n..m`, `..`, `n..`, `..m`. The range is half-open, exclusive on the right, as is common in
  /// Rust standard library.
  ///
  /// # Example
  ///
  /// ```rust
  /// let mut ctx = meval::Context::empty();
  ///
  /// // require exactly 2 arguments
  /// ctx.funcn("sum_two", |xs| xs[0] + xs[1], 2);
  ///
  /// // allow an arbitrary number of arguments
  /// ctx.funcn("sum", |xs| xs.iter().sum(), ..);
  /// ```
  pub fn funcn<S, F, N>(&mut self, name: S, func: F, n_args: N) -> &mut Self
  where
    S: Into<String>,
    F: Fn(&[f64]) -> f64 + 'a,
    N: ArgGuard,
  {
    self.funcs.insert(name.into(), n_args.to_arg_guard(func));
    self
  }
}

impl<'a> Default for Context<'a> {
  fn default() -> Self {
    Context::new()
  }
}

pub(crate) type GuardedFunc<'a> = Rc<dyn Fn(&[f64]) -> Result<f64, FuncEvalError> + 'a>;

/// Trait for types that can specify the number of required arguments for a function with a
/// variable number of arguments.
///
/// # Example
///
/// ```rust
/// let mut ctx = meval::Context::empty();
///
/// // require exactly 2 arguments
/// ctx.funcn("sum_two", |xs| xs[0] + xs[1], 2);
///
/// // allow an arbitrary number of arguments
/// ctx.funcn("sum", |xs| xs.iter().sum(), ..);
/// ```
pub trait ArgGuard {
  fn to_arg_guard<'a, F: Fn(&[f64]) -> f64 + 'a>(self, func: F) -> GuardedFunc<'a>;
}

impl ArgGuard for usize {
  fn to_arg_guard<'a, F: Fn(&[f64]) -> f64 + 'a>(self, func: F) -> GuardedFunc<'a> {
    Rc::new(move |args: &[f64]| {
      if args.len() == self {
        Ok(func(args))
      } else {
        Err(FuncEvalError::NumberArgs(1))
      }
    })
  }
}

impl ArgGuard for std::ops::RangeFrom<usize> {
  fn to_arg_guard<'a, F: Fn(&[f64]) -> f64 + 'a>(self, func: F) -> GuardedFunc<'a> {
    Rc::new(move |args: &[f64]| {
      if args.len() >= self.start {
        Ok(func(args))
      } else {
        Err(FuncEvalError::TooFewArguments)
      }
    })
  }
}

impl ArgGuard for std::ops::RangeTo<usize> {
  fn to_arg_guard<'a, F: Fn(&[f64]) -> f64 + 'a>(self, func: F) -> GuardedFunc<'a> {
    Rc::new(move |args: &[f64]| {
      if args.len() < self.end {
        Ok(func(args))
      } else {
        Err(FuncEvalError::TooManyArguments)
      }
    })
  }
}

impl ArgGuard for std::ops::Range<usize> {
  fn to_arg_guard<'a, F: Fn(&[f64]) -> f64 + 'a>(self, func: F) -> GuardedFunc<'a> {
    Rc::new(move |args: &[f64]| {
      if args.len() >= self.start && args.len() < self.end {
        Ok(func(args))
      } else if args.len() < self.start {
        Err(FuncEvalError::TooFewArguments)
      } else {
        Err(FuncEvalError::TooManyArguments)
      }
    })
  }
}

impl ArgGuard for std::ops::RangeFull {
  fn to_arg_guard<'a, F: Fn(&[f64]) -> f64 + 'a>(self, func: F) -> GuardedFunc<'a> {
    Rc::new(move |args: &[f64]| Ok(func(args)))
  }
}

impl<'a> ContextProvider for Context<'a> {
  fn get_var(&self, name: &str) -> Option<f64> {
    self.vars.get(name).cloned()
  }
  fn eval_func(&self, name: &str, args: &[f64]) -> Result<f64, FuncEvalError> {
    self
      .funcs
      .get(name)
      .map_or(Err(FuncEvalError::UnknownFunction), |f| f(args))
  }
}
