struct Employee {
    ceo:String, 
    company:String,
    age:u32
}
fn main() {

    let emp1 = employee {
        company:String::from("Microsoft corporation"),
        ceo:String::from("satya Nadella"),
        age:56
    };
    let emp2 = employee{
        company:String::from("Google Inc."),
        ceo:String::from("Sundal Pichai"),
        age:51
    };
    display(emp1);
    display(emp2);
}


fn diplay( emp:employee){
    println!("Name i :{} company is {} age is {}",emp.ceo,emp.company,emp.age);
}