use std::io;

fn input(prompt: &str) -> f64 {
    let mut value = String::new();
    println!("{}",prompt);
    io::stdin().read_line(&mut value).expect("Failed to read input");
    value.trim().parse::<f64>().expect("Please enter a valid number")
}


fn area_Trapezium() {
    let h = input("Enter the height:");
    let b1 = input("Enter base 1:");
    let b2 = input("Enter base 2:");
    let area = (h / 2.0) * b1 + b2;
    println!("Area of Trapezium = {}", area);
}


fn area_Rhombus() {
    let d1 = input("Enter diagonal 1:");
    let d2 = input("Enter diagonal 2:");
    let area = 0.5 * d1 * d2;
    println!("Area of Rhombus = {}", area);

}

fn area_parallelogram() {
    let b = input("Enter base:");
    let a = input("Enter altitude:");
    let area = a * b;
    println!("Area of parallelogram = {}",area);
}

fn area_cube() {
    let l = input("Enter length of side:");
    let area = 6.0 * l * l;
    println!("Area of cube is {}", area);

}

fn volume_cylinder() {
    let r = input("Enter radius here:");
    let h = input ("Enter height here:");
    let pi = 3.14;
    let volume = pi * r * r * h;
    println!("volume of cylinder is {}", volume)
}

fn main() {
    println!("Select a shape to calculate:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. area of parallelogram");
    println!("4. Area of cube");
    println!("5. Volume of cylinder");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: u32 = choice.trim().parse().expect("Enter a valid number");



    match choice{
        1 => area_Trapezium(),
        2 => area_Rhombus(),
        3 => area_parallelogram(),
        4 => area_cube(),
        5 => volume_cylinder(),
        _ => println!("invalid choice!")
    }
}
