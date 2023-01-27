use super::Rules;

const RULESTR: &str = "
// shorthands
e^x <=> exp(x)
2^x <=> exp2(x)
e^x - 1 <=> exp_m1(x)
ln(x + 1) <=> ln_1p(x)

// binary operations
x^(y * z) <=> (x^y)^z
x^(y + z) <=> x^y * x^z

x^(-1) <=> 1/x

// inverses
e^x = y <=> x = ln(y)
2^x = y <=> x = log2(y)
10^x = y <=> x = log10(y)
exp_m1(x) = y <=> x = ln_1p(y)
";

pub fn exponential_rules() -> Rules {
  RULESTR.parse().expect("exponential rules did not parse")
}
