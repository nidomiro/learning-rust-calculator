use regex::{Regex};

#[derive(Debug)]
struct InvalidOperatorError { operator: char }

#[derive(Debug)]
struct Operation {
    left: i32,
    operator: char,
    right: i32,
}

impl Operation {
    fn from_string(input: &String) -> Option<Operation> {
        let regex = Regex::new(r"^\s*(\d+)\s*([+\-*/])\s*(\d+)\s*$").unwrap();

        regex.captures(&input).map(|capture| Operation {
            left: (&capture[1]).parse().unwrap(),
            operator: (&capture[2]).parse().unwrap(),
            right: (&capture[3]).parse().unwrap(),
        })
    }

    fn execute(&self) -> Result<i32, InvalidOperatorError> {
        match self {
            Operation { left, operator: '+', right } => Ok(left + right),
            Operation { left, operator: '-', right } => Ok(left - right),
            Operation { left, operator: '*', right } => Ok(left * right),
            Operation { left, operator: '/', right } => Ok(left / right),
            x => Err(InvalidOperatorError {operator: x.operator}),
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
            break
        }

        let operation = match Operation::from_string(&input) {
            None => {
                println!("Your input was invalid");
                continue
            },
            Some(x) => x
        };

        let result = match operation.execute() {
            Err(InvalidOperatorError { operator: op}) => {
                println!("The operator {} is invalid", op);
                continue
            },
            Ok(x) => x
        };


        println!("The result is: {}", result);
    }
}
