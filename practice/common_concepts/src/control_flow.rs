pub fn eligible_to_vote(age: u32) {
    if age > 18 {
        println!("you are eligible to vote");
    } else if age < 18 {
        println!("you are not eligible to vote");
    } else {
        println!("you are eligble to vote this year, now make voting card if not already");
    }
}
pub fn sum(a: u32, b: u32) -> u32 {
    a + b
}

pub fn iteration(x: u32) {
    let mut y = 0;
    loop {
        if y >= x {
            break;
        }
        println!("Looping inside the iteration function {}", y);
        y += 1;
    }
}

pub fn while_iteration(x: u32) {
    let mut y = 0;
    while x > y {
        println!("the value of iteration number is {}", y);
        y += 1;
    }
}

pub fn for_iteration_in_array(a: &[i32]) {
    for element in a {
        println!("{}", element);
    }
}

pub fn for_loop() {
    for i in 0..10 {
        println!("{i}");
    }
}
