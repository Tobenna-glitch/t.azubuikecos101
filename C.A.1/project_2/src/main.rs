
use std::io;

fn main() {
let mut input1 = String::new();
let mut input2 = String::new();
let mut input3 = String::new();

println!("Enter p");
io::stdin().read_line(&mut input1).expect("Not a valind string");
let p:u64 = input1.trim().parse().expect("Not a valid number");

println!("Enter r");
io::stdin().read_line(&mut input2).expect("Not a valind string");
let r:u64 = input2.trim().parse().expect("Not a valid number");

println!("Enter t");
io::stdin().read_line(&mut input3).expect("Not a valind string");
let t:u64 = input3.trim().parse().expect("Not a valid number");

let c:u64 = p * (1 + r / 100)^t;
let compound_interest_float = p * (1 + r / 100)^t;
let compound_interest_float = compound_interest_float.isqrt();



println!("compound interest: {}", compound_interest_float);



}