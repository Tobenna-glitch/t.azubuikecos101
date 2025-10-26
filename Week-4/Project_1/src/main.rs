// Rust program to calculate the roots of a quadratic equation

use std::io;

fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value of a:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a: f64 = input1.trim().parse().expect("please enter a valid number");
    input1.clear();

    println!("Enter the value of b:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b: f64 = input2.trim().parse().expect("please enter a valid number");
    input2.clear();

    println!("Enter the value of c");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c: f64 = input3.trim().parse().expect("please enter a valid number");

    if a == 0.0 {
        println!("Not a quadratic equation (a must not be zero).");
        return;
    }
     let discrminant = b * b - 4.0 * a * c;

     if discrminant > 0.0 {
        let root1 = (-b + discrminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discrminant.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: {:.2} and {:.2}", root1, root2);
    } else if discrminant == 0.0 {
        let root = -b/ (2.0 * a);
    }
}
