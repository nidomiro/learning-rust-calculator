use regex::{Captures, Regex};

#[derive(Debug)]
enum MathStringParseError {
    NotParsable,
    InvalidOperatorError {
        operator: char,
    }
}

#[derive(Debug)]
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
    fn from_string(input: &String) -> Result<Operation, MathStringParseError> {
        let regex = Regex::new(r"^\s*(\d+)\s*(\S)\s*(\d+)\s*$").unwrap();

        fn from_capture(capture: Captures) -> Result<Operation, MathStringParseError> {
            let operator: char = (&capture[2]).parse().unwrap();
            let left_side: f64 = (&capture[1]).parse().unwrap();
            let right_side: f64 = (&capture[3]).parse().unwrap();
            match operator {
                '+' => Ok(Operation::Addition {
                    left_addend: Box::new(Operation::Number(left_side)),
                    right_addend: Box::new(Operation::Number(right_side)),
                }),
                '-' => Ok(Operation::Subtraction {
                    minuend: Box::new(Operation::Number(left_side)),
                    subtrahend: Box::new(Operation::Number(right_side)),
                }),
                '*' => Ok(Operation::Multiplication {
                    multiplicand: Box::new(Operation::Number(left_side)),
                    multiplier: Box::new(Operation::Number(right_side)),
                }),
                '/' => Ok(Operation::Division {
                    dividend: Box::new(Operation::Number(left_side)),
                    divisor: Box::new(Operation::Number(right_side)),
                }),
                x => Err(MathStringParseError::InvalidOperatorError {
                    operator: x,
                }),
            }
        }

        match regex.captures(&input) {
            None => Err(MathStringParseError::NotParsable),
            Some(capture) => from_capture(capture)
        }
    }

    fn execute(&self) -> f64 {
        match self {
            Operation::Number(x) => *x,
            Operation::Addition {left_addend, right_addend} => left_addend.execute() + right_addend.execute(),
            Operation::Subtraction {minuend, subtrahend} => minuend.execute() - subtrahend.execute(),
            Operation::Multiplication {multiplicand, multiplier} => multiplicand.execute() * multiplier.execute(),
            Operation::Division {dividend, divisor} => dividend.execute() * divisor.execute(),
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
            },
            Err(MathStringParseError::InvalidOperatorError {operator}) => {
                println!("{} is an invalid operator", operator);
                continue;
            }
            Ok(x) => x,
        };

        let result = operation.execute();

        println!("The result is: {}", result);
    }
}
