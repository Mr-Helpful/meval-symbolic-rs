use super::Rules;

const RULESTR: &str = "
// basic identities
sin(x)^2 + cos(x)^2 <=> 1
tan(x) <=> sin(x) / cos(x)

// negations
sin(-x) <=> -sin(x)
cos(-x) <=> cos(x)

// addition formula
sin(x + y) <=> sin(x)cos(y) + sin(y)cos(x)
cos(x + y) <=> cos(x)cos(y) - sin(x)sin(y)
tan(x + y) <=> (tan(x) + tan(y)) / (1 + tan(x)tan(y))

// double formula
sin(2 * x) <=> 2*sin(x)*cos(x)
cos(2 * x) <=> cos(x)^2 - sin(x)^2
tan(2 * x) <=> (2*tan(x)) / (1 + tan(x)^2)

// inverses
sin(x) = y <=> x = asin(y)
cos(x) = y <=> x = acos(y)
tan(x) = y <=> x = atan(y)
";

pub fn trigonometry_rules() -> Rules {
  RULESTR.parse().expect("trigonometry rules did not parse")
}
