fn main() {
    let principal: f64 = 520_000_000.0;
    let rate: f64 = 10.0;
    let years: u32 = 5;

    // Calculate the amount
    let amount = principal * (1.0 + rate / 100.0).powi(years as i32);

    // Calculate compound interest
    let compound_interest = amount - principal;

    println!("Total Amount after {} years: ₦{:.2}", years, amount);
    println!("Compound Interest: ₦{:.2}", compound_interest);
}
