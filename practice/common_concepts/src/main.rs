fn main() {
    //declaring a mutable var;
    let mut x = 10;
    x += 1;
    println!("the value of x is {x}");
    // shadowing -> redeclaring a variable;
    let x = "String";
    println!("the variable that is shadowed is {x}");

    // integer overflow and wrapping
    let y: u8 = 256; // at compile time this should give error but at run time, it will wrap to
                     // 0->256, 1->257 like that.
    println!("the value of y is {y}");
}
