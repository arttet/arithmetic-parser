use std::io::{self, BufRead};

extern crate arithmetic_parser;

fn main() {
    println!("Enter a valid expression:");
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let s = iterator.next().unwrap().unwrap();
    println!("Result: {}", arithmetic_parser::Parser::calculate(s))
}
