pub fn takes_ownership(s: String) -> String {
    println!("this is taking ownership");
    println!("{s}");
    s
}

pub fn takes_stack_ownership(number: u32) -> () {
    println!("i can use the number here as it makes a copy because it is faster to do so");
    println!("{number}");
}
