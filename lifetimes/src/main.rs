struct User<'a> {
    name: &'a str,
}
fn main() {
    println!("Lifetimes");

    let a = String::from("hellow");
    let b = String::from("World");
    let longest = longest(&a, &b);
    println!("{}", longest);
    let name = String::from("hello");
    let user = User { name: &name };
    println!("{}", user.name);
}
//since the compiler does not know
//the scope till which the variable would be available
//so it asks us to specify until which part the argument is valid.
//this wierd syntax is just representing that the answer reference is
//valid until the reference of both the argument is valid, like till when the
//intersection of both variable is valid, only till there, the return type is valid
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
