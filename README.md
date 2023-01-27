[![Build Status](https://travis-ci.org/rekka/meval-rs.svg?branch=master)](https://travis-ci.org/rekka/meval-rs)
[![](http://meritbadge.herokuapp.com/meval)](https://crates.io/crates/meval)
[![meval at docs.rs](https://docs.rs/meval/badge.svg)](https://docs.rs/meval)

# meval-symbolic

This [Rust] crate builds on the work of [meval](https://github.com/rekka/meval-rs) by adding:

- [x] support for common maths operators `+,-,*,/,,%` and methods `abs,sin,...` on expressions
- [x] support for replacing subexpressions, i.e. `replace((x + 2) + x, x + 2, u) = u + x`
- [ ] symbolic solving for variables within expressions, i.e. `solve(x^2 + 2x + 1 = 0, x) = -1`

## Documentation

- [Full API documentation](https://docs.rs/meval)

## Installation

> :heavy_exclamation_mark:__Warning__:heavy_exclamation_mark: : Not currently a package but when I eventually get around to publishing

Simply add the corresponding entry to your `Cargo.toml` dependency list:

```toml
[dependencies]
meval_symbolic = "1"
```

__Requires Rust 1.26.__

## Simple examples

```rust
fn main() {
    let r = meval_symbolic::eval_str("1 + 2").unwrap();

    println!("1 + 2 = {}", r);
}
```

Need to define a Rust function from an expression? No problem, use `Expr`
for this and more:

```rust
fn main() {
    let expr: meval_symbolic::Expr = "sin(pi * x)".parse().unwrap();
    let func = expr.bind("x").unwrap();

    let vs: Vec<_> = (0..100+1).map(|i| func(i as f64 / 100.)).collect();

    println!("sin(pi * x), 0 <= x <= 1: {:?}", vs);
}
```

Custom constants and functions? Define a `Context`!

```rust
use meval_symbolic::{Expr, Context};

let y = 1.;
let expr: Expr = "phi(-2 * zeta + x)".parse().unwrap();

// create a context with function definitions and variables
let mut ctx = Context::new(); // built-ins
ctx.func("phi", |x| x + y)
   .var("zeta", -1.);
// bind function with a custom context
let func = expr.bind_with_context(ctx, "x").unwrap();
assert_eq!(func(2.), -2. * -1. + 2. + 1.);
```

For functions of 2, 3, and N variables use `Context::func2`, `Context::func3` and
`Context::funcn`,
respectively. See `Context` for more options.

If you need a custom function depending on mutable parameters, you will need to use a
[`Cell`](https://doc.rust-lang.org/stable/std/cell/struct.Cell.html):

```rust
use std::cell::Cell;
use meval_symbolic::{Expr, Context};
let y = Cell::new(0.);
let expr: Expr = "phi(x)".parse().unwrap();

let mut ctx = Context::empty(); // no built-ins
ctx.func("phi", |x| x + y.get());

let func = expr.bind_with_context(ctx, "x").unwrap();
assert_eq!(func(2.), 2.);
y.set(3.);
assert_eq!(func(2.), 5.);
```

## Supported expressions

`meval_symbolic` supports basic mathematical operations on floating point numbers:

- binary operators: `+`, `-`, `*`, `/`, `%` (remainder), `^` (power)
- unary operators: `+`, `-`, `!` (factorial)

It supports custom variables and functions like `x`, `weight`, `C_0`, `f(1)`, etc. A variable
or function name must start with `[a-zA-Z_]` and can contain only `[a-zA-Z0-9_]`. Custom
functions with a variable number of arguments are also supported.

Build-ins (given by the context `Context::new()` and when no context provided) currently
supported:

- functions implemented using functions of the same name in [Rust std library][std-float]:

  - `sqrt`, `abs`
  - `exp`, `ln`, `log10`
  - `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`
  - `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh`
  - `floor`, `ceil`, `round`
  - `signum`

- other functions:

  - `max(x, ...)`, `min(x, ...)`: maximum and minimumum of 1 or more numbers

- constants:

  - `pi`
  - `e`

## Deserialization

`Expr` supports deserialization using the [serde] library to make flexible
configuration easy to set up, if the feature `serde` is enabled
(disabled by default).

```rust
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate meval_symbolic;
use meval_symbolic::{Expr, Context};

#[derive(Deserialize)]
struct Ode {
    #[serde(deserialize_with = "meval_symbolic::de::as_f64")]
    x0: f64,
    #[serde(deserialize_with = "meval_symbolic::de::as_f64")]
    t0: f64,
    f: Expr,
}

fn main() {
    let config = r#"
        x0 = "cos(1.)"
        t0 = 2
        f = "sin(x)"
    "#;
    let ode: Ode = toml::from_str(config).unwrap();

    assert_eq!(ode.x0, 1f64.cos());
    assert_eq!(ode.t0, 2f64);
    assert_eq!(ode.f.bind("x").unwrap()(2.), 2f64.sin());
}

```

## Related projects

This is a toy project of mine for learning Rust, and to be hopefully useful when writing
command line scripts. There is no plan to make this anything more than _math expression ->
number_ "converter". For more advanced uses, see:

- [evalexpr] -- A more complete expression evaluator that supports value
  types, boolean operators, strings, assignment operators, ...
- [dyon] -- A rusty dynamically typed scripting language
- [gluon] -- A static, type inferred programming language for application embedding
- [rodolf0/tox](https://github.com/rodolf0/tox) -- another shunting yard expression parser

[Rust]: https://www.rust-lang.org/
[std-float]: http://doc.rust-lang.org/stable/std/primitive.f64.html

[serde]: https://crates.io/crates/serde
[evalexpr]: https://crates.io/crates/evalexpr
[dyon]: https://crates.io/crates/dyon
[gluon]: https://crates.io/crates/gluon

## License

This project is dual-licensed under the Unlicense and MIT licenses.

You may use this code under the terms of either license.
