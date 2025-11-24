use std::io;


fn main() {
    
    let lager = vec!("33 export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star");
    let stout = vec!("legend", "Turbo king", "Williams");
    let non_alcoholic = vec!("Maltina", "Amstel Malta", "Malta Gold", "Fayrouz");

    println!("Enter type of Drink");
    println!("1. Lager\n2. Stout\n3. Non-Alcoholic");

    let mut input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("Failed to read line");
    let user_choice:u8 = input_1.trim().parse().expect("Failed to read line");

    if user_choice == 1
    {
        println!("The drinks under Lager are {:?}", lager);
    }
    else if user_choice == 2
    {
        println!("the drinks under Stout are {:?}", stout);
    }
    else if user_choice == 3
    {
        println!("The drinks under Non=Alcholic are {:?}", non_alcoholic)
    }
}
