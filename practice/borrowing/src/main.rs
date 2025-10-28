fn main() {
    let mut s1 = String::from("hello world");
    do_something(&mut s1);
}
fn do_something(s: &mut String) {
    s.push_str("This is rust references");
    println!("{}", s);
}
