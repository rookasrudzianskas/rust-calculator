use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let result = operate(operator, first_number, second_number);

    println!("{:?}", result);
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    if operator == '+' {
        first_number + second_number
    } else if  operator == '-' {
        first_number - second_number
    } else if operator == '/' {
        first_number / second_number
    } else if operator == '*' {
        first_number * second_number
    } else {
        // panic!("Unknown operator");
        0.0
    }
}
