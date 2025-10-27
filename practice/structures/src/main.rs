mod tuple_struct;
mod user;
fn main() {
    let mut user1 = user::User {
        name: String::from("rust"),
        eligible: true,
    };
    user1.name = String::from("rust and rustlings");
    println!("{} {}", user1.name, user1.eligible);
    let user2 = user::User {
        name: user1.name,
        eligible: false,
    };
    println!("{} {}", user2.name, user2.eligible);
    let white = tuple_struct::Color(255, 255, 255);
    println!("{}, {},  {}", white.0, white.1, white.2);
}
