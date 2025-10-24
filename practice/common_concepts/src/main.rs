mod control_flow;

fn main() {
    //declaring a mutable var;
    let mut x = 10;
    x += 1;
    println!("the value of x is {x}");
    // shadowing -> redeclaring a variable;
    let x = "String";
    println!("the variable that is shadowed is {x}");

    // integer overflow and wrapping
    //let y: u8 = 256; // at compile time this should give error but at run time, it will wrap to
    // 0->256, 1->257 like that.
    //println!("the value of y is {y}");

    // tuples
    let mut y = (2, 4, "String");
    y.0 += 2;
    let (a, b, c) = y;
    println!("{a} {b} {c}");

    let array = [1, 2, 3, 4, 5];
    // functions
    let first = 5;
    let second = 6;
    let result = control_flow::sum(first, second);
    println!("{result}");

    // if else
    let age = 18;
    control_flow::eligible_to_vote(age);
    control_flow::iteration(4);
    control_flow::while_iteration(5);
    control_flow::for_iteration_in_array(&array);
    control_flow::for_loop();
}
