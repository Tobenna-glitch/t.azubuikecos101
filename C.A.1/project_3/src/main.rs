fn main() {
    // Items, Item code and quantity
    let sales = vec![
    ("Laptop", 550_000.00),
    ("Monitor", 120_000.00),
    ("Keyboard", 15_000.00),
    ("Headset", 25_000.00),
    ];

    // Ask user to enter an item code and quantity
    println!("Enter an an Item code:");
    println!("Enter the quantity:");
    
    // Calculate the total cost
    let mut total_cost = 0.0
    let count = sales.len() as f64;
    println!("Total sales: (:.2}", total_cost);

     // Introduction of if else statements
     if total_cost >500_000.00
     {
    println!("sales increased by 7 percent: ", sales * 7 / 100);
     }
  else 
  {
  println!("No increased sales")
  }

}