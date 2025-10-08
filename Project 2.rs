fn main() {
    // Amounts from the sales record
    let amounts = [
        450_000.00,  // Toshiba
        1_500_000.00, // Mac
        750_000.00,   // HP
        2_850_000.00, // Dell
        250_000.00    // Acer
    ];

    // Calculate the sum
    let total: f64 = amounts.iter().sum();

    // Calculate the average
    let average = total / amounts.len() as f64;

    // Print the results
    println!("Total Sales Amount: ₦{:.2}", total);
    println!("Average Sales Amount: ₦{:.2}", average);
}
