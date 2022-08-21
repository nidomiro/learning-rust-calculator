use regex::{Captures, Regex};

#[derive(Debug, PartialEq)]
enum MathStringParseError {
    NotParsable,
    InvalidOperatorError { operator: char },
}

#[derive(Debug, PartialEq)]
enum Operation {
    Number(f64),
    Addition {
        left_addend: Box<Operation>,
        right_addend: Box<Operation>,
    },
    Subtraction {
        minuend: Box<Operation>,
        subtrahend: Box<Operation>,
    },
    Multiplication {
        multiplicand: Box<Operation>,
        multiplier: Box<Operation>,
    },
    Division {
        dividend: Box<Operation>,
        divisor: Box<Operation>,
    },
}

impl Operation {
    /// Constructs an Operation from an String
    /// By now only one operator is supported
    fn from_string(input: &str) -> Result<Operation, MathStringParseError> {
        let regex = Regex::new(r"^\s*(\d+)\s*(\S)\s*(\d+)\s*$").unwrap();

        fn from_capture(capture: Captures) -> Result<Operation, MathStringParseError> {
            let operator: char = (&capture[2]).parse().unwrap();
            let left_side: f64 = (&capture[1]).parse().unwrap();
            let right_side: f64 = (&capture[3]).parse().unwrap();
            match operator {
                '+' => Ok(Operation::Addition {
                    left_addend: Operation::Number(left_side).into(),
                    right_addend: Operation::Number(right_side).into(),
                }),
                '-' => Ok(Operation::Subtraction {
                    minuend: Operation::Number(left_side).into(),
                    subtrahend: Operation::Number(right_side).into(),
                }),
                '*' => Ok(Operation::Multiplication {
                    multiplicand: Operation::Number(left_side).into(),
                    multiplier: Operation::Number(right_side).into(),
                }),
                '/' => Ok(Operation::Division {
                    dividend: Operation::Number(left_side).into(),
                    divisor: Operation::Number(right_side).into(),
                }),
                x => Err(MathStringParseError::InvalidOperatorError { operator: x }),
            }
        }

        match regex.captures(&input) {
            None => Err(MathStringParseError::NotParsable),
            Some(capture) => from_capture(capture),
        }
    }

    fn execute(&self) -> f64 {
        match self {
            Operation::Number(x) => *x,
            Operation::Addition {
                left_addend,
                right_addend,
            } => left_addend.execute() + right_addend.execute(),
            Operation::Subtraction {
                minuend,
                subtrahend,
            } => minuend.execute() - subtrahend.execute(),
            Operation::Multiplication {
                multiplicand,
                multiplier,
            } => multiplicand.execute() * multiplier.execute(),
            Operation::Division { dividend, divisor } => dividend.execute() / divisor.execute(),
        }
    }
}

fn main() {
    loop {
        println!("Please enter a calculation (or quit to exit)");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");

        if input.trim().eq("quit") {
            println!("exiting...");
            break;
        }

        let operation = match Operation::from_string(&input) {
            Err(MathStringParseError::NotParsable) => {
                println!("Your input was invalid");
                continue;
            }
            Err(MathStringParseError::InvalidOperatorError { operator }) => {
                println!("{} is an invalid operator", operator);
                continue;
            }
            Ok(x) => x,
        };

        let result = operation.execute();

        println!("The result is: {}", result);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // copied from https://stackoverflow.com/a/34666891/1469540
    macro_rules! operation_from_string_test {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    assert_eq!(Operation::from_string(input), expected);
                }
            )*
            }
    }

    operation_from_string_test! {
        simple_addition: ("1+2", Ok(Operation::Addition {
            left_addend: Number(1.0).into(),
            right_addend: Number(2.0).into()
        })),
        simple_subtraction: ("1-2", Ok(Operation::Subtraction {
            minuend: Number(1.0).into(),
            subtrahend: Number(2.0).into()
        })),
        simple_multiplication: ("1*2", Ok(Operation::Multiplication {
            multiplicand: Number(1.0).into(),
            multiplier: Number(2.0).into()
        })),
        simple_division: ("1/2", Ok(Operation::Division {
            dividend: Number(1.0).into(),
            divisor: Number(2.0).into()
        })),
    }

    #[test]
    fn exec_simple_addition() {
        assert_eq!(
            Operation::Addition {
                left_addend: Operation::Number(1.0).into(),
                right_addend: Operation::Number(2.0).into()
            }
            .execute(),
            3.0
        )
    }

    #[test]
    fn exec_simple_subtraction() {
        assert_eq!(
            Operation::Subtraction {
                minuend: Operation::Number(1.0).into(),
                subtrahend: Operation::Number(2.0).into()
            }
            .execute(),
            -1.0
        )
    }

    #[test]
    fn exec_simple_multiplication() {
        assert_eq!(
            Operation::Multiplication {
                multiplicand: Operation::Number(1.0).into(),
                multiplier: Operation::Number(2.0).into()
            }
            .execute(),
            2.0
        )
    }

    #[test]
    fn exec_simple_division() {
        assert_eq!(
            Operation::Division {
                dividend: Operation::Number(1.0).into(),
                divisor: Operation::Number(2.0).into()
            }
            .execute(),
            0.5
        )
    }
}
