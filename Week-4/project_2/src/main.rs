use std::io;
  

  fn main() {
    // Ask for experience
    println!("Is the employee experienced? (yes/no):");
    let mut experience_input = String::new();
    io::stdin()
        .read_line(&mut experience_input)
        .expect("Failed to read input");
    let experienced = experience_input.trim().eq_ignore_ascii_case("yes");

    // Ask for age
    println!("Enter the age of the employee:");
    let mut age_input = String::new();
    io::stdin()
        .read_line(&mut age_input)
        .expect("Failed to read input");

    let age: u32 = match age_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid age entered. Please enter a valid number.");
            return;
        }
    };

    // Determine incentive
    let incentive = if experienced {
        if age >= 40 {
            1_560_000
        } else if age >= 30 {
            1_480_000
        } else if age < 28 {
            1_300_000
        } else {
            // Optional case for ages 28–29 not explicitly stated
            1_300_000
        }
    } else {
        100_000
    };

    println!("The annual incentive for this employee is: ₦{}", incentive);
}