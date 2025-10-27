mod user;

fn main() {
    let user1 = user::User {
        name: String::from("nikhil"),
        eligible: true,
    };
    println!("{} {}", user1.name, user1.eligible);
}
