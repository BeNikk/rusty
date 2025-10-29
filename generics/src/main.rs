fn main() {
    let largest_number = largest(2, 4);
    let largest_str = largest("nik", "bh");
    println!("{} {}", largest_str, largest_number);
}

fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
