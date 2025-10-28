use chrono::{Local, Utc};
fn main() {
    let now = Utc::now();
    println!("Current date and time in UTC:{}", now);
    let local = Local::now();
    println!("current date and time in local is {}", local);
}
