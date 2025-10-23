fn main() {
    let x = 10;
    let y = x;
    println!("{x} {y} "); // x and y are variables in the stack that is why they are deeply copied
                          // by default
    println!("the pointers to x and y are {:p} {:p}", &x, &y);
    let s1 = String::from("hello");
    let s2 = s1; // these are heap variables, now s1 no longer have ownership of "hello
                 // string"
    println!("{s2}");
}
