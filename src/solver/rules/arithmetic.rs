use super::Rules;

const RULESTR: &str = "
// commutativity
x + y <=> y + x
x * y <=> y * x

// unary operations
x - y <=> x + -y
x / y <=> x * (1/y)

// associativity
(x + y) + z <=> x + (y + z)
(x - y) - z <=> x - (y + z)
x - (y - z) <=> (x - y) + z
(x * y) * z <=> x * (y * z)
(x / y) / z <=> x / (y * z)

// inverses
x + y = z <=> x = z - y
x * y = z <=> x = z / y

// distributivity
x * (y + z) <=> x * y + x * z
(x + y) / z <=> x / z + y / z

// given by distributivity and commutativity
x + x <-> 2 * x

// units
x + 0 <=> x
x - 0 <=> x
x - x <=> 0
x * 1 <=> x
x / 1 <=> x
x / x <=> 1
-0 <=> 0
";

pub fn arithmetic_rules() -> Rules {
  RULESTR.parse().expect("arithmetic rules did not parse")
}
