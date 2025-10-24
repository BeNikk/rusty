pub fn calculate_length(s: &String) {
    let length_of_string = s.len();
    println!("The length of string is {length_of_string}");
}

pub fn mutable_references(s: &mut String) {
    s.push_str("this is added by an external function that mutably borrows string s");
}

pub fn dangle() -> String {
    String::from("new str")
}
