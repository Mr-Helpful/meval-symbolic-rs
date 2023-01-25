use crate::Evaluatable_Trait;

use super::expr::*;
use super::Expr;
use Error;

/// A relation between two expressions, e.g. `x = 5`
pub struct Eqtn(pub(crate) Expr, pub(crate) Expr);

/// We can implement all of the eval methods in terms of the left and right
/// hand sides of an equation
impl Eqtn {
  /// Evaluates the equation with variables given by the argument.
  pub fn eval_with_context<C: ContextProvider + Clone>(&self, ctx: C) -> Result<bool, Error> {
    let lhs = self.0.eval_with_context(ctx.clone())?;
    let rhs = self.1.eval_with_context(ctx)?;
    Ok(lhs == rhs)
  }

  /// Checks that the value of every variable in the equation is specified by
  /// the context `ctx`.
  ///
  /// # Failure
  ///
  /// Returns `Err` if a missing variable is detected.
  fn check_context<C: ContextProvider + Clone>(&self, ctx: C) -> Result<(), Error> {
    self.0.check_context(ctx.clone())?;
    self.1.check_context(ctx)
  }
}

Evaluatable_Trait!(Eqtn bool);
