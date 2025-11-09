use std::io;

fn main() {
    println!("============== MENU ==============");
    println!("p = Poundo Yam/Edinkaiko soup       -N3,200");
    println!("F = Fried Rice & Chicken            -N3,000 ");
    println!("A = Amala & Ewedu Soup              -N2,500");
    println!("E = Eba & Egusi Soup                -N2,000");
    println!("W = White Rice & Stew               -N2,500");
    println!("======================================");

    let mut food_type = String::new();
    let mut quantity = String::new();

    // Input food type
    println!("Enter the food type (P/F/A/E/W):");
    io::stdin().read_line(&mut food_type).expect("Failed to read input");
    let food_type = food_type.trim().to_uppercase();

    // Input quantity
    println!("Enter the quantity:");
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    let quantity: i32 = quantity.trim().parse().expect("Enter a valid number");

    let price: i32 = match food_type.as_str() {
        "p" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Invalid food type selected!");
            return;
        }
    };

    let total = price * quantity;


    // Apply 5% dicount if total > 10,000
    let final_amount = if total > 10000 {
        total as f32 * 0.95
    } else {
        total as f32
    };

    println!("============================");
    println!("Total Amount (before discount): N{}", total);

    if total > 10000 {
        println!("Discount Applied: 5%");
    }

    println!("Final amount to pay: N{}", final_amount);
    println!("========================================")

    
}