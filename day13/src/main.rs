use serde_json::{Value, Error};

fn main() {
    let input: Vec<Vec<&str>> = include_str!("../example.txt").split("\n\n").map(|l| l.lines().collect()).collect();
    println!("{:?}", input);
    let value: Result<Value, Error> = serde_json::from_str(input[0][0]);
    println!("{:?}", value);
}
