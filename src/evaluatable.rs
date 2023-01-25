/// Whilst this should be implemented using a trait, it can't be until issue
/// #91611 <https://github.com/rust-lang/rust/issues/91611> is stabilised as the
/// `bind` methods return closures.
///
/// TODO: when #91611 is stabilised, rewrite to use the trait below instead.
#[macro_export]
macro_rules! Evaluatable_Trait {
  ($name:ident $Res:ty) => {
    impl $name {
      /// Evaluates the term.
      pub fn eval(&self) -> Result<$Res, Error> {
        self.eval_with_context(builtin())
      }

      /// Creates a function of one variable based on this term, with default constants and
      /// functions.
      ///
      /// Binds the input of the returned closure to `var`.
      ///
      /// # Failure
      ///
      /// Returns `Err` if there is a variable in the term that is not provided by the default
      /// context or `var`.
      pub fn bind<'a>(self, var: &str) -> Result<impl Fn(f64) -> $Res + 'a, Error> {
        self.bind_with_context(builtin(), var)
      }

      /// Creates a function of one variable based on this term.
      ///
      /// Binds the input of the returned closure to `var`.
      ///
      /// # Failure
      ///
      /// Returns `Err` if there is a variable in the term that is not provided by `ctx` or
      /// `var`.
      pub fn bind_with_context<'a, C>(
        self,
        ctx: C,
        var: &str,
      ) -> Result<impl Fn(f64) -> $Res + 'a, Error>
      where
        C: ContextProvider + 'a,
      {
        self.check_context(((var, 0.), &ctx))?;
        let var = var.to_owned();
        Ok(move |x| {
          self
            .eval_with_context(((&var, x), &ctx))
            .expect("Expr::bind")
        })
      }

      /// Creates a function of two variables based on this term, with default constants and
      /// functions.
      ///
      /// Binds the inputs of the returned closure to `var1` and `var2`.
      ///
      /// # Failure
      ///
      /// Returns `Err` if there is a variable in the term that is not provided by the default
      /// context or `var`.
      pub fn bind2<'a>(
        self,
        var1: &str,
        var2: &str,
      ) -> Result<impl Fn(f64, f64) -> $Res + 'a, Error> {
        self.bind2_with_context(builtin(), var1, var2)
      }

      /// Creates a function of two variables based on this term.
      ///
      /// Binds the inputs of the returned closure to `var1` and `var2`.
      ///
      /// # Failure
      ///
      /// Returns `Err` if there is a variable in the term that is not provided by `ctx` or
      /// `var`.
      pub fn bind2_with_context<'a, C>(
        self,
        ctx: C,
        var1: &str,
        var2: &str,
      ) -> Result<impl Fn(f64, f64) -> $Res + 'a, Error>
      where
        C: ContextProvider + 'a,
      {
        self.check_context(([(var1, 0.), (var2, 0.)], &ctx))?;
        let var1 = var1.to_owned();
        let var2 = var2.to_owned();
        Ok(move |x, y| {
          self
            .eval_with_context(([(&var1, x), (&var2, y)], &ctx))
            .expect("Expr::bind2")
        })
      }

      /// Creates a function of three variables based on this term, with default constants and
      /// functions.
      ///
      /// Binds the inputs of the returned closure to `var1`, `var2` and `var3`.
      ///
      /// # Failure
      ///
      /// Returns `Err` if there is a variable in the term that is not provided by the default
      /// context or `var`.
      pub fn bind3<'a>(
        self,
        var1: &str,
        var2: &str,
        var3: &str,
      ) -> Result<impl Fn(f64, f64, f64) -> $Res + 'a, Error> {
        self.bind3_with_context(builtin(), var1, var2, var3)
      }

      /// Creates a function of three variables based on this term.
      ///
      /// Binds the inputs of the returned closure to `var1`, `var2` and `var3`.
      ///
      /// # Failure
      ///
      /// Returns `Err` if there is a variable in the term that is not provided by `ctx` or
      /// `var`.
      pub fn bind3_with_context<'a, C>(
        self,
        ctx: C,
        var1: &str,
        var2: &str,
        var3: &str,
      ) -> Result<impl Fn(f64, f64, f64) -> $Res + 'a, Error>
      where
        C: ContextProvider + 'a,
      {
        self.check_context(([(var1, 0.), (var2, 0.), (var3, 0.)], &ctx))?;
        let var1 = var1.to_owned();
        let var2 = var2.to_owned();
        let var3 = var3.to_owned();
        Ok(move |x, y, z| {
          self
            .eval_with_context(([(&var1, x), (&var2, y), (&var3, z)], &ctx))
            .expect("Expr::bind3")
        })
      }

      /// Creates a function of four variables based on this term, with default constants and
      /// functions.
      ///
      /// Binds the inputs of the returned closure to `var1`, `var2`, `var3` and `var4`.
      ///
      /// # Failure
      ///
      /// Returns `Err` if there is a variable in the term that is not provided by the default
      /// context or `var`.
      pub fn bind4<'a>(
        self,
        var1: &str,
        var2: &str,
        var3: &str,
        var4: &str,
      ) -> Result<impl Fn(f64, f64, f64, f64) -> $Res + 'a, Error> {
        self.bind4_with_context(builtin(), var1, var2, var3, var4)
      }

      /// Creates a function of four variables based on this term.
      ///
      /// Binds the inputs of the returned closure to `var1`, `var2`, `var3` and `var4`.
      ///
      /// # Failure
      ///
      /// Returns `Err` if there is a variable in the term that is not provided by `ctx` or
      /// `var`.
      pub fn bind4_with_context<'a, C>(
        self,
        ctx: C,
        var1: &str,
        var2: &str,
        var3: &str,
        var4: &str,
      ) -> Result<impl Fn(f64, f64, f64, f64) -> $Res + 'a, Error>
      where
        C: ContextProvider + 'a,
      {
        self.check_context(([(var1, 0.), (var2, 0.), (var3, 0.), (var4, 0.)], &ctx))?;
        let var1 = var1.to_owned();
        let var2 = var2.to_owned();
        let var3 = var3.to_owned();
        let var4 = var4.to_owned();
        Ok(move |x1, x2, x3, x4| {
          self
            .eval_with_context(([(&var1, x1), (&var2, x2), (&var3, x3), (&var4, x4)], &ctx))
            .expect("Expr::bind4")
        })
      }

      /// Creates a function of five variables based on this term, with default constants and
      /// functions.
      ///
      /// Binds the inputs of the returned closure to `var1`, `var2`, `var3`, `var4` and `var5`.
      ///
      /// # Failure
      ///
      /// Returns `Err` if there is a variable in the term that is not provided by the default
      /// context or `var`.
      pub fn bind5<'a>(
        self,
        var1: &str,
        var2: &str,
        var3: &str,
        var4: &str,
        var5: &str,
      ) -> Result<impl Fn(f64, f64, f64, f64, f64) -> $Res + 'a, Error> {
        self.bind5_with_context(builtin(), var1, var2, var3, var4, var5)
      }

      /// Creates a function of five variables based on this term.
      ///
      /// Binds the inputs of the returned closure to `var1`, `var2`, `var3`, `var4` and `var5`.
      ///
      /// # Failure
      ///
      /// Returns `Err` if there is a variable in the term that is not provided by `ctx` or
      /// `var`.
      pub fn bind5_with_context<'a, C>(
        self,
        ctx: C,
        var1: &str,
        var2: &str,
        var3: &str,
        var4: &str,
        var5: &str,
      ) -> Result<impl Fn(f64, f64, f64, f64, f64) -> $Res + 'a, Error>
      where
        C: ContextProvider + 'a,
      {
        self.check_context((
          [(var1, 0.), (var2, 0.), (var3, 0.), (var4, 0.), (var5, 0.)],
          &ctx,
        ))?;
        let var1 = var1.to_owned();
        let var2 = var2.to_owned();
        let var3 = var3.to_owned();
        let var4 = var4.to_owned();
        let var5 = var5.to_owned();
        Ok(move |x1, x2, x3, x4, x5| {
          self
            .eval_with_context((
              [
                (&var1, x1),
                (&var2, x2),
                (&var3, x3),
                (&var4, x4),
                (&var5, x5),
              ],
              &ctx,
            ))
            .expect("Expr::bind5")
        })
      }

      /// Binds the input of the returned closure to elements of `vars`.
      ///
      /// # Failure
      ///
      /// Returns `Err` if there is a variable in the term that is not provided by the default
      /// context or `var`.
      pub fn bindn<'a>(self, vars: &'a [&str]) -> Result<impl Fn(&[f64]) -> $Res + 'a, Error> {
        self.bindn_with_context(builtin(), vars)
      }

      /// Creates a function of N variables based on this term.
      ///
      /// Binds the input of the returned closure to the elements of `vars`.
      ///
      /// # Failure
      ///
      /// Returns `Err` if there is a variable in the term that is not provided by `ctx` or
      /// `var`.
      pub fn bindn_with_context<'a, C>(
        self,
        ctx: C,
        vars: &'a [&str],
      ) -> Result<impl Fn(&[f64]) -> $Res + 'a, Error>
      where
        C: ContextProvider + 'a,
      {
        let n = vars.len();
        self.check_context((
          vars
            .into_iter()
            .zip(vec![0.; n].into_iter())
            .collect::<Vec<_>>(),
          &ctx,
        ))?;
        let vars = vars.iter().map(|v| v.to_owned()).collect::<Vec<_>>();
        Ok(move |x: &[f64]| {
          self
            .eval_with_context((
              vars
                .iter()
                .zip(x.into_iter())
                .map(|(v, x)| (v, *x))
                .collect::<Vec<_>>(),
              &ctx,
            ))
            .expect("Expr::bindn")
        })
      }
    }
  };
}

/*
use super::{builtin, ContextProvider, Error};

pub trait Evaluatable<Res>: Sized {
  /// Evaluates the term with variables given by the argument.
  fn eval_with_context(&self, ctx: impl ContextProvider) -> Result<Res, Error>;

  /// Checks that the value of every variable in the term is specified by the context `ctx`.
  ///
  /// # Failure
  ///
  /// Returns `Err` if a missing variable is detected.
  fn check_context<C: ContextProvider>(&self, ctx: C) -> Result<(), Error>;

  /// Evaluates the term.
  fn eval(&self) -> Result<Res, Error> {
    self.eval_with_context(builtin())
  }

  /// Creates a function of one variable based on this expression, with default constants and
  /// functions.
  ///
  /// Binds the input of the returned closure to `var`.
  ///
  /// # Failure
  ///
  /// Returns `Err` if there is a variable in the expression that is not provided by the default
  /// context or `var`.
  fn bind<'a>(self, var: &str) -> Result<impl Fn(f64) -> Res + 'a, Error> {
    self.bind_with_context(builtin(), var)
  }

  /// Creates a function of one variable based on this expression.
  ///
  /// Binds the input of the returned closure to `var`.
  ///
  /// # Failure
  ///
  /// Returns `Err` if there is a variable in the expression that is not provided by `ctx` or
  /// `var`.
  fn bind_with_context<'a, C, F: Fn(f64) -> Res + 'a>(
    self,
    ctx: C,
    var: &str,
  ) -> Result<impl Fn(f64) -> Res + 'a, Error>
  where
    C: ContextProvider + 'a,
  {
    self.check_context(((var, 0.), &ctx))?;
    let var = var.to_owned();
    Ok(move |x| {
      self
        .eval_with_context(((&var, x), &ctx))
        .expect("Expr::bind")
    })
  }

  /// Creates a function of two variables based on this expression, with default constants and
  /// functions.
  ///
  /// Binds the inputs of the returned closure to `var1` and `var2`.
  ///
  /// # Failure
  ///
  /// Returns `Err` if there is a variable in the expression that is not provided by the default
  /// context or `var`.
  fn bind2<'a>(self, var1: &str, var2: &str) -> Result<impl Fn(f64, f64) -> Res + 'a, Error> {
    self.bind2_with_context(builtin(), var1, var2)
  }

  /// Creates a function of two variables based on this expression.
  ///
  /// Binds the inputs of the returned closure to `var1` and `var2`.
  ///
  /// # Failure
  ///
  /// Returns `Err` if there is a variable in the expression that is not provided by `ctx` or
  /// `var`.
  fn bind2_with_context<'a, C>(
    self,
    ctx: C,
    var1: &str,
    var2: &str,
  ) -> Result<impl Fn(f64, f64) -> Res + 'a, Error>
  where
    C: ContextProvider + 'a,
  {
    self.check_context(([(var1, 0.), (var2, 0.)], &ctx))?;
    let var1 = var1.to_owned();
    let var2 = var2.to_owned();
    Ok(move |x, y| {
      self
        .eval_with_context(([(&var1, x), (&var2, y)], &ctx))
        .expect("Expr::bind2")
    })
  }

  /// Creates a function of three variables based on this expression, with default constants and
  /// functions.
  ///
  /// Binds the inputs of the returned closure to `var1`, `var2` and `var3`.
  ///
  /// # Failure
  ///
  /// Returns `Err` if there is a variable in the expression that is not provided by the default
  /// context or `var`.
  fn bind3<'a>(
    self,
    var1: &str,
    var2: &str,
    var3: &str,
  ) -> Result<impl Fn(f64, f64, f64) -> Res + 'a, Error> {
    self.bind3_with_context(builtin(), var1, var2, var3)
  }

  /// Creates a function of three variables based on this expression.
  ///
  /// Binds the inputs of the returned closure to `var1`, `var2` and `var3`.
  ///
  /// # Failure
  ///
  /// Returns `Err` if there is a variable in the expression that is not provided by `ctx` or
  /// `var`.
  fn bind3_with_context<'a, C>(
    self,
    ctx: C,
    var1: &str,
    var2: &str,
    var3: &str,
  ) -> Result<impl Fn(f64, f64, f64) -> Res + 'a, Error>
  where
    C: ContextProvider + 'a,
  {
    self.check_context(([(var1, 0.), (var2, 0.), (var3, 0.)], &ctx))?;
    let var1 = var1.to_owned();
    let var2 = var2.to_owned();
    let var3 = var3.to_owned();
    Ok(move |x, y, z| {
      self
        .eval_with_context(([(&var1, x), (&var2, y), (&var3, z)], &ctx))
        .expect("Expr::bind3")
    })
  }

  /// Creates a function of four variables based on this expression, with default constants and
  /// functions.
  ///
  /// Binds the inputs of the returned closure to `var1`, `var2`, `var3` and `var4`.
  ///
  /// # Failure
  ///
  /// Returns `Err` if there is a variable in the expression that is not provided by the default
  /// context or `var`.
  fn bind4<'a>(
    self,
    var1: &str,
    var2: &str,
    var3: &str,
    var4: &str,
  ) -> Result<impl Fn(f64, f64, f64, f64) -> Res + 'a, Error> {
    self.bind4_with_context(builtin(), var1, var2, var3, var4)
  }

  /// Creates a function of four variables based on this expression.
  ///
  /// Binds the inputs of the returned closure to `var1`, `var2`, `var3` and `var4`.
  ///
  /// # Failure
  ///
  /// Returns `Err` if there is a variable in the expression that is not provided by `ctx` or
  /// `var`.
  fn bind4_with_context<'a, C>(
    self,
    ctx: C,
    var1: &str,
    var2: &str,
    var3: &str,
    var4: &str,
  ) -> Result<impl Fn(f64, f64, f64, f64) -> Res + 'a, Error>
  where
    C: ContextProvider + 'a,
  {
    self.check_context(([(var1, 0.), (var2, 0.), (var3, 0.), (var4, 0.)], &ctx))?;
    let var1 = var1.to_owned();
    let var2 = var2.to_owned();
    let var3 = var3.to_owned();
    let var4 = var4.to_owned();
    Ok(move |x1, x2, x3, x4| {
      self
        .eval_with_context(([(&var1, x1), (&var2, x2), (&var3, x3), (&var4, x4)], &ctx))
        .expect("Expr::bind4")
    })
  }

  /// Creates a function of five variables based on this expression, with default constants and
  /// functions.
  ///
  /// Binds the inputs of the returned closure to `var1`, `var2`, `var3`, `var4` and `var5`.
  ///
  /// # Failure
  ///
  /// Returns `Err` if there is a variable in the expression that is not provided by the default
  /// context or `var`.
  fn bind5<'a>(
    self,
    var1: &str,
    var2: &str,
    var3: &str,
    var4: &str,
    var5: &str,
  ) -> Result<impl Fn(f64, f64, f64, f64, f64) -> Res + 'a, Error> {
    self.bind5_with_context(builtin(), var1, var2, var3, var4, var5)
  }

  /// Creates a function of five variables based on this expression.
  ///
  /// Binds the inputs of the returned closure to `var1`, `var2`, `var3`, `var4` and `var5`.
  ///
  /// # Failure
  ///
  /// Returns `Err` if there is a variable in the expression that is not provided by `ctx` or
  /// `var`.
  fn bind5_with_context<'a, C>(
    self,
    ctx: C,
    var1: &str,
    var2: &str,
    var3: &str,
    var4: &str,
    var5: &str,
  ) -> Result<impl Fn(f64, f64, f64, f64, f64) -> Res + 'a, Error>
  where
    C: ContextProvider + 'a,
  {
    self.check_context((
      [(var1, 0.), (var2, 0.), (var3, 0.), (var4, 0.), (var5, 0.)],
      &ctx,
    ))?;
    let var1 = var1.to_owned();
    let var2 = var2.to_owned();
    let var3 = var3.to_owned();
    let var4 = var4.to_owned();
    let var5 = var5.to_owned();
    Ok(move |x1, x2, x3, x4, x5| {
      self
        .eval_with_context((
          [
            (&var1, x1),
            (&var2, x2),
            (&var3, x3),
            (&var4, x4),
            (&var5, x5),
          ],
          &ctx,
        ))
        .expect("Expr::bind5")
    })
  }

  /// Binds the input of the returned closure to elements of `vars`.
  ///
  /// # Failure
  ///
  /// Returns `Err` if there is a variable in the expression that is not provided by the default
  /// context or `var`.
  fn bindn<'a>(self, vars: &'a [&str]) -> Result<impl Fn(&[f64]) -> Res + 'a, Error> {
    self.bindn_with_context(builtin(), vars)
  }

  /// Creates a function of N variables based on this expression.
  ///
  /// Binds the input of the returned closure to the elements of `vars`.
  ///
  /// # Failure
  ///
  /// Returns `Err` if there is a variable in the expression that is not provided by `ctx` or
  /// `var`.
  fn bindn_with_context<'a, C>(
    self,
    ctx: C,
    vars: &'a [&str],
  ) -> Result<impl Fn(&[f64]) -> Res + 'a, Error>
  where
    C: ContextProvider + 'a,
  {
    let n = vars.len();
    self.check_context((
      vars
        .into_iter()
        .zip(vec![0.; n].into_iter())
        .collect::<Vec<_>>(),
      &ctx,
    ))?;
    let vars = vars.iter().map(|v| v.to_owned()).collect::<Vec<_>>();
    Ok(move |x: &[f64]| {
      self
        .eval_with_context((
          vars
            .iter()
            .zip(x.into_iter())
            .map(|(v, x)| (v, *x))
            .collect::<Vec<_>>(),
          &ctx,
        ))
        .expect("Expr::bindn")
    })
  }
}
*/
