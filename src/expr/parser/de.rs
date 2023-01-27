//! Deserialization utilities.
use super::Expr;
use serde::de::Deserializer;
use serde::de::Error;
use serde::Deserialize;

/// Deserialize into [`Expr`](../struct.Expr.html) and then evaluate using `Expr::eval`.
///
/// # Example
///
/// ```rust
/// #[macro_use]
/// extern crate serde_derive;
/// extern crate toml;
/// extern crate meval;
/// use meval::{Expr, Context};
///
/// #[derive(Deserialize)]
/// struct Foo {
///     #[serde(deserialize_with = "meval::de::as_f64")]
///     x: f64,
/// }
///
/// fn main() {
///     let foo: Foo = toml::from_str(r#" x = "cos(1.)" "#).unwrap();
///     assert_eq!(foo.x, 1f64.cos());
///
///     let foo: Result<Foo, _> = toml::from_str(r#" x = "cos(x)" "#);
///     assert!(foo.is_err());
/// }
/// ```
///
/// See [crate root](../index.html#deserialization) for another example.
pub fn as_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
  Expr::deserialize(deserializer)?
    .eval()
    .map_err(D::Error::custom)
}

use std::fmt;
use std::str::FromStr;
use tokenizer::Token;

impl<'de> Deserialize<'de> for Expr {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct ExprVisitor;

    impl<'de> serde::de::Visitor<'de> for ExprVisitor {
      type Value = Expr;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a math expression")
      }

      fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Expr::from_str(v).map_err(serde::de::Error::custom)
      }

      fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Expr(vec![Token::Number(v)]))
      }

      fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Expr(vec![Token::Number(v as f64)]))
      }

      fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Expr(vec![Token::Number(v as f64)]))
      }
    }

    deserializer.deserialize_any(ExprVisitor)
  }
}

#[cfg(test)]
mod tests {
  use self::as_f64;
  use super::*;
  use serde_json;
  use serde_test;
  #[test]
  fn test_deserialization() {
    use serde_test::Token;
    let expr = Expr::from_str("sin(x)").unwrap();

    serde_test::assert_de_tokens(&expr, &[Token::Str("sin(x)")]);
    serde_test::assert_de_tokens(&expr, &[Token::String("sin(x)")]);

    let expr = Expr::from_str("5").unwrap();

    serde_test::assert_de_tokens(&expr, &[Token::F64(5.)]);
    serde_test::assert_de_tokens(&expr, &[Token::U8(5)]);
    serde_test::assert_de_tokens(&expr, &[Token::I8(5)]);
  }

  #[test]
  fn test_json_deserialization() {
    #[derive(Deserialize)]
    struct Ode {
      #[serde(deserialize_with = "as_f64")]
      x0: f64,
      #[serde(deserialize_with = "as_f64")]
      t0: f64,
      f: Expr,
      g: Expr,
      h: Expr,
    }

    let config = r#"
            {
                "x0": "cos(1.)",
                "t0": 2,
                "f": "sin(x)",
                "g": 2.5,
                "h": 5
            }
            "#;
    let ode: Ode = serde_json::from_str(config).unwrap();

    assert_eq!(ode.x0, 1f64.cos());
    assert_eq!(ode.t0, 2f64);
    assert_eq!(ode.f.bind("x").unwrap()(2.), 2f64.sin());
    assert_eq!(ode.g.eval().unwrap(), 2.5f64);
    assert_eq!(ode.h.eval().unwrap(), 5f64);
  }
}
