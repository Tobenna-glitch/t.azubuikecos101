fn main() {
    
    let v = vec![10,20,30];


    let v2 = v;

    diplay(v2);


    println!("In main {:?}",v2);
}

fn display(v:Vec<i32>){
    println!("inside display {:?}",v);
}
