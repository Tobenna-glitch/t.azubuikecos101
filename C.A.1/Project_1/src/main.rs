// Rust program for student grades

use std::io;

fn main() {

let mut input1 = String::new();
let mut input2 = String::new();
let mut input3 = String::new();


 println!("Enter student name");
   let mut name = String::new();
   io::stdin().read_line(&mut name).expect("Failed to read input");
   println!("Your name is: {}", name);

   // input score
   println!("\nEnter first score");
   io::stdin().read_line(&mut input1).expect("Not a valid string");
   let input1:f64 = input1.trim().parse().expect("Not a valid number");

   println!("\nEnter second score");
   io::stdin().read_line(&mut input2).expect("Not a valid string");
   let input2:f64 = input2.trim().parse().expect("Not a valid number");

   println!("\nEnter third score");
   io::stdin().read_line(&mut input3).expect("Not a valid string");
   let input3:f64 = input3.trim().parse().expect("Not a a valid number");

   let a:f64 = (input1 + input2 + input3) / 3.0;
   let average_float = (input1 + input2 + input3) / 3.0; // float type
   let _average = average_float.sqrt();

   println!("The Average of the scores: {}", average_float);


   if average_float >=70.0 && average_float <=100.0
   {
      println!("A");
   }

   if average_float >=60.0 && average_float <=69.0
   {
    println!("B");
   }

   if average_float >=50.0 && average_float <=59.0
   {
    println!("C");
   }

   if average_float >=45.0 && average_float <=49.0
   {
    println!("D");
   }

   if average_float >=0.0 && average_float <=44.0
   {
    println!("F");
   }

   

}