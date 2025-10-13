fn main() {
    let p: f64 = 510_000.0; // Initial cost of the TV
    let r: f64 = 5.0;       // Depreciation rate in percent
    let n: u32 = 3;         // Number of years

    // Calculate depreciation factor
    let depreciation_factor = 1.0 - (r / 100.0);

    // Calculate depreciated value after n years
    let a = p * depreciation_factor.powi(n as i32);

    // Output the result
    println!("Value of the TV after {} years: ₦{:.2}", n, a);
}
