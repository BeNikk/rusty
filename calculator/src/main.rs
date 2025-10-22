use std::io;
fn main() {
    println!("Basic Calculator");
    let mut x = String::new();
    println!("Enter the first number");
    io::stdin()
        .read_line(&mut x)
        .expect("failed to read the user input");
    let mut y = String::new();
    println!("Enter the second number");
    io::stdin()
        .read_line(&mut y)
        .expect("failed to read the user input");
    let mut operation = String::new();
    println!("enter the operation (+,-,*,/)");
    io::stdin()
        .read_line(&mut operation)
        .expect("failed to read the user input");
    let x: u32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("not a number");
            return;
        }
    };
    let y: u32 = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("not a number");
            return;
        }
    };
    let operation = operation.trim();
    let result = match operation {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => {
            if y == 0 {
                println!("cannot divide by 0");
                return;
            }
            x / y
        }
        _ => {
            println!("invalid operation");
            return;
        }
    };
    println!("result {result}");
}
