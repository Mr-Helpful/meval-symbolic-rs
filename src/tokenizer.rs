//! Tokenizer that converts a mathematical expression in a string form into a series of `Token`s.
//!
//! The underlying parser is build using the [nom] parser combinator crate.
//!
//! The parser should tokenize only well-formed expressions.
//!
//! [nom]: https://crates.io/crates/nom
use nom::{
  branch::alt,
  bytes::complete::{is_not, tag},
  character::complete::{alpha1, alphanumeric1, multispace0},
  combinator::{complete, map, map_res, recognize, value},
  multi::many0,
  number::complete::recognize_float,
  sequence::{delimited, pair, preceded, terminated},
  IResult, Needed,
};

use std::{
  self, fmt,
  num::NonZeroUsize,
  str::{from_utf8, FromStr},
};

/// An error reported by the parser.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
  /// A token that is not allowed at the given location (contains the location of the offending
  /// character in the source string).
  UnexpectedToken(usize),
  /// Missing right parentheses at the end of the source string (contains the number of missing
  /// parens).
  MissingRParen(i32),
  /// Missing operator or function argument at the end of the expression.
  MissingArgument,
}

impl From<(&[u8], nom::Err<nom::error::Error<&[u8]>>)> for ParseError {
  /// Converts from the initial input and a nom parse error into our parse error
  fn from((bytes, err): (&[u8], nom::Err<nom::error::Error<&[u8]>>)) -> Self {
    use nom::Err::*;
    use ParseError::*;

    match err {
      Incomplete(Needed::Unknown) => MissingArgument,
      Incomplete(Needed::Size(n)) => MissingRParen(n.get() as i32),
      Error(e) => UnexpectedToken(bytes.len() - e.input.len()),
      Failure(e) => panic!(
        "Unexpected parse result when parsing `{}` at `{}`: {:?}",
        String::from_utf8_lossy(bytes),
        String::from_utf8_lossy(e.input),
        Failure(e)
      ),
    }
  }
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ParseError::UnexpectedToken(i) => write!(f, "Unexpected token at byte {}.", i),
      ParseError::MissingRParen(i) => write!(
        f,
        "Missing {} right parenthes{}.",
        i,
        if i == 1 { "is" } else { "es" }
      ),
      ParseError::MissingArgument => write!(f, "Missing argument at the end of expression."),
    }
  }
}

impl std::error::Error for ParseError {
  fn description(&self) -> &str {
    match *self {
      ParseError::UnexpectedToken(_) => "unexpected token",
      ParseError::MissingRParen(_) => "missing right parenthesis",
      ParseError::MissingArgument => "missing argument",
    }
  }
}

/// Mathematical operations.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operation {
  Plus,
  Minus,
  Times,
  Div,
  Rem,
  Pow,
  Fact,
}

/// Expression tokens.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  /// Binary operation.
  Binary(Operation),
  /// Unary operation.
  Unary(Operation),

  /// Left parenthesis.
  LParen,
  /// Right parenthesis.
  RParen,
  /// Comma: function argument separator
  Comma,

  /// A number.
  Number(f64),
  /// A variable.
  Var(String),
  /// A function with name and number of arguments.
  Func(String, Option<usize>),
}

fn binop(input: &[u8]) -> IResult<&[u8], Token> {
  alt((
    value(Token::Binary(Operation::Plus), tag("+")),
    value(Token::Binary(Operation::Minus), tag("-")),
    value(Token::Binary(Operation::Times), tag("*")),
    value(Token::Binary(Operation::Div), tag("/")),
    value(Token::Binary(Operation::Rem), tag("%")),
    value(Token::Binary(Operation::Pow), tag("^")),
  ))(input)
}

fn negpos(input: &[u8]) -> IResult<&[u8], Token> {
  alt((
    value(Token::Unary(Operation::Plus), tag("+")),
    value(Token::Unary(Operation::Minus), tag("-")),
  ))(input)
}

fn fact(input: &[u8]) -> IResult<&[u8], Token> {
  value(Token::Unary(Operation::Fact), tag("!"))(input)
}
fn lparen(input: &[u8]) -> IResult<&[u8], Token> {
  value(Token::LParen, tag("("))(input)
}
fn rparen(input: &[u8]) -> IResult<&[u8], Token> {
  value(Token::RParen, tag(")"))(input)
}
fn comma(input: &[u8]) -> IResult<&[u8], Token> {
  value(Token::Comma, tag(","))(input)
}

/// Parse an identifier:
///
/// Must start with a letter or an underscore, can be followed by letters, digits or underscores.
fn ident(input: &[u8]) -> IResult<&[u8], &[u8]> {
  recognize(pair(
    alt((tag("_"), alpha1)),
    many0(alt((tag("_"), alphanumeric1))),
  ))(input)
}

fn var(input: &[u8]) -> IResult<&[u8], Token> {
  map(map_res(complete(ident), |s| from_utf8(s)), |s| {
    Token::Var(s.into())
  })(input)
}

/// Parse `func(`, returns `func`.
fn func(input: &[u8]) -> IResult<&[u8], Token> {
  map(
    map_res(
      terminated(complete(ident), preceded(multispace0, complete(tag("(")))),
      from_utf8,
    ),
    |s| Token::Func(s.into(), None),
  )(input)
}

/// Matches a float according to the specification given in [f64::from_str()](https://doc.rust-lang.org/std/primitive.f64.html#method.from_str).
fn number(input: &[u8]) -> IResult<&[u8], Token> {
  // don't match `-` or `+` as these should be treated as unary operators
  is_not("-+")(input)?;
  map(
    map_res(map_res(recognize_float, from_utf8), f64::from_str),
    Token::Number,
  )(input)
}

fn lexpr(input: &[u8]) -> IResult<&[u8], Token> {
  delimited(
    multispace0,
    alt((number, func, var, negpos, lparen)),
    multispace0,
  )(input)
}
fn after_rexpr(input: &[u8]) -> IResult<&[u8], Token> {
  delimited(multispace0, alt((fact, binop, rparen)), multispace0)(input)
}
fn after_rexpr_no_paren(input: &[u8]) -> IResult<&[u8], Token> {
  delimited(multispace0, alt((fact, binop)), multispace0)(input)
}
fn after_rexpr_comma(input: &[u8]) -> IResult<&[u8], Token> {
  delimited(multispace0, alt((fact, binop, rparen, comma)), multispace0)(input)
}

#[derive(Debug, Clone, Copy)]
enum TokenizerState {
  // accept any token that is an expression from the left: var, num, (, negpos
  LExpr,
  // accept any token that needs an expression on the left: fact, binop, ), comma
  AfterRExpr,
}

#[derive(Debug, Clone, Copy)]
enum ParenState {
  SubExpr,
  FuncArgs,
}

/// Parse a given mathematical expression
///
/// I've split out + cleaned up this parser
/// so it can be reused for equations and rules
///
/// Also this breaks the usual way that parser combinators are written for
/// considerably better performance as it uses much less backtracking
pub(crate) fn expression(mut input: &[u8]) -> IResult<&[u8], Vec<Token>> {
  use self::ParenState::*;
  use self::Token::*;
  use self::TokenizerState::*;

  // current level of nesting, including whether the level represents arguments.
  let mut paren_stack = vec![];
  let mut state = LExpr;
  let mut res = vec![];

  while !input.is_empty() {
    let t;
    (input, t) = match (state, paren_stack.last()) {
      (LExpr, _) => lexpr(input),
      (AfterRExpr, None) => after_rexpr_no_paren(input),
      (AfterRExpr, Some(&SubExpr)) => after_rexpr(input),
      (AfterRExpr, Some(&FuncArgs)) => after_rexpr_comma(input),
    }?;

    match t {
      LParen => {
        paren_stack.push(SubExpr);
      }
      Func(..) => {
        paren_stack.push(FuncArgs);
      }
      RParen => {
        paren_stack.pop().expect("The paren_stack is empty!");
      }
      Var(_) | Number(_) => {
        state = AfterRExpr;
      }
      Binary(_) | Comma => {
        state = LExpr;
      }
      Unary(_) => {}
    }
    res.push(t);
  }

  match state {
    LExpr => Err(nom::Err::Incomplete(Needed::Unknown)),
    _ if !paren_stack.is_empty() => Err(nom::Err::Incomplete(Needed::Size(
      NonZeroUsize::new(paren_stack.len())
        .expect("The stack was non empty but the stack length was 0???"),
    ))),
    _ => Ok((input, res)),
  }
}

/// Tokenize a given mathematical expression.
///
/// The parser should return `Ok` only if the expression is well-formed.
///
/// # Failure
///
/// Returns `Err` if the expression is not well-formed.
pub fn tokenize<S: AsRef<str>>(input: S) -> Result<Vec<Token>, ParseError> {
  let bytes = input.as_ref().as_bytes();
  expression(bytes)
    .map(|(_, tkns)| tkns)
    .map_err(|err| ParseError::from((bytes, err)))
}

#[cfg(test)]
mod tests {
  use super::*;
  use nom::error::ErrorKind;

  fn error<O>(input: &[u8], code: ErrorKind) -> IResult<&[u8], O> {
    Err(nom::Err::Error(nom::error::Error { input, code }))
  }

  fn failure<O>(input: &[u8], code: ErrorKind) -> IResult<&[u8], O> {
    Err(nom::Err::Failure(nom::error::Error { input, code }))
  }

  #[test]
  fn it_works() {
    assert_eq!(binop(b"+"), Ok((&b""[..], Token::Binary(Operation::Plus))));
    assert_eq!(number(b"32143"), Ok((&b""[..], Token::Number(32143f64))));
    assert_eq!(var(b"abc"), Ok((&b""[..], Token::Var("abc".into()))));
    assert_eq!(
      func(b"abc("),
      Ok((&b""[..], Token::Func("abc".into(), None)))
    );
    assert_eq!(
      func(b"abc ("),
      Ok((&b""[..], Token::Func("abc".into(), None)))
    );
  }

  #[test]
  fn test_var() {
    for &s in ["abc", "U0", "_034", "a_be45EA", "aAzZ_"].iter() {
      assert_eq!(var(s.as_bytes()), Ok((&b""[..], Token::Var(s.into()))));
    }

    assert_eq!(var(b""), error(&b""[..], ErrorKind::Alpha));
    assert_eq!(var(b"0"), error(&b"0"[..], ErrorKind::Alpha));
  }

  #[test]
  fn test_func() {
    for &s in ["abc(", "u0(", "_034 (", "A_be45EA  ("].iter() {
      assert_eq!(
        func(s.as_bytes()),
        Ok((
          &b""[..],
          Token::Func((&s[0..s.len() - 1]).trim().into(), None)
        ))
      );
    }

    assert_eq!(func(b""), error(&b""[..], ErrorKind::Alpha));
    assert_eq!(func(b"("), error(&b"("[..], ErrorKind::Alpha));
    assert_eq!(func(b"0("), error(&b"0("[..], ErrorKind::Alpha));
  }

  #[test]
  fn test_number() {
    assert_eq!(number(b"32143"), Ok((&b""[..], Token::Number(32143f64))));
    assert_eq!(number(b"2."), Ok((&b""[..], Token::Number(2.0f64))));
    assert_eq!(
      number(b"32143.25"),
      Ok((&b""[..], Token::Number(32143.25f64)))
    );
    assert_eq!(
      number(b"0.125e9"),
      Ok((&b""[..], Token::Number(0.125e9f64)))
    );
    assert_eq!(
      number(b"20.5E-3"),
      Ok((&b""[..], Token::Number(20.5E-3f64)))
    );
    assert_eq!(
      number(b"123423e+50"),
      Ok((&b""[..], Token::Number(123423e+50f64)))
    );

    assert_eq!(number(b""), error(&b""[..], ErrorKind::IsNot));
    assert_eq!(number(b"+"), error(&b"+"[..], ErrorKind::IsNot));
    assert_eq!(number(b"e"), error(&b"e"[..], ErrorKind::Char));
    assert_eq!(number(b"1E"), failure(&b""[..], ErrorKind::Digit));
    assert_eq!(number(b"1e+"), failure(&b""[..], ErrorKind::Digit));
  }

  #[test]
  fn test_tokenize() {
    use super::Operation::*;
    use super::Token::*;

    assert_eq!(tokenize("a"), Ok(vec![Var("a".into())]));

    assert_eq!(
      tokenize("2 +(3--2) "),
      Ok(vec![
        Number(2f64),
        Binary(Plus),
        LParen,
        Number(3f64),
        Binary(Minus),
        Unary(Minus),
        Number(2f64),
        RParen
      ])
    );

    assert_eq!(
      tokenize("-2^ ab0 *12 - C_0"),
      Ok(vec![
        Unary(Minus),
        Number(2f64),
        Binary(Pow),
        Var("ab0".into()),
        Binary(Times),
        Number(12f64),
        Binary(Minus),
        Var("C_0".into()),
      ])
    );

    assert_eq!(
      tokenize("-sin(pi * 3)^ cos(2) / Func2(x, f(y), z) * _buildIN(y)"),
      Ok(vec![
        Unary(Minus),
        Func("sin".into(), None),
        Var("pi".into()),
        Binary(Times),
        Number(3f64),
        RParen,
        Binary(Pow),
        Func("cos".into(), None),
        Number(2f64),
        RParen,
        Binary(Div),
        Func("Func2".into(), None),
        Var("x".into()),
        Comma,
        Func("f".into(), None),
        Var("y".into()),
        RParen,
        Comma,
        Var("z".into()),
        RParen,
        Binary(Times),
        Func("_buildIN".into(), None),
        Var("y".into()),
        RParen,
      ])
    );

    assert_eq!(
      tokenize("2 % 3"),
      Ok(vec![Number(2f64), Binary(Rem), Number(3f64)])
    );

    assert_eq!(
      tokenize("1 + 3! + 1"),
      Ok(vec![
        Number(1f64),
        Binary(Plus),
        Number(3f64),
        Unary(Fact),
        Binary(Plus),
        Number(1f64)
      ])
    );

    assert_eq!(tokenize("!3"), Err(ParseError::UnexpectedToken(0)));

    assert_eq!(tokenize("()"), Err(ParseError::UnexpectedToken(1)));

    assert_eq!(tokenize(""), Err(ParseError::MissingArgument));
    assert_eq!(tokenize("2)"), Err(ParseError::UnexpectedToken(1)));
    assert_eq!(tokenize("2^"), Err(ParseError::MissingArgument));
    assert_eq!(tokenize("(((2)"), Err(ParseError::MissingRParen(2)));
    assert_eq!(tokenize("f(2,)"), Err(ParseError::UnexpectedToken(4)));
    assert_eq!(tokenize("f(,2)"), Err(ParseError::UnexpectedToken(2)));
  }
}
