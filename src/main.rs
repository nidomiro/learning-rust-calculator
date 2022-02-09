use regex::{Regex};

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

    fn execute(&self) -> Result<i32, String> {
        match self {
            Operation { left, operator: '+', right } => Ok(left + right),
            Operation { left, operator: '-', right } => Ok(left - right),
            Operation { left, operator: '*', right } => Ok(left * right),
            Operation { left, operator: '/', right } => Ok(left / right),
            x => Err(format!("{} is not a valid operator", x.operator)),
        }
    }
}

fn main() {
    println!("Please enter a calculation");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");

    let operation = Operation::from_string(&input).expect("Your input is invalid");

    let result = operation.execute();


    println!("The result is: {:?}", result.unwrap());
}
