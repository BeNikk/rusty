mod functions;
mod references;
mod slice;
fn main() {
    let x = 10;
    let y = x;
    println!("{x} {y} "); // x and y are variables in the stack that is why they are deeply copied
                          // by default
    println!("the pointers to x and y are {:p} {:p}", &x, &y);
    let s1 = String::from("hello");
    let mut s2 = s1; // these are heap variables, now s1 no longer have ownership of "hello
                     // string"
    println!("{s2}");
    s2 = functions::takes_ownership(s2);
    println!("s2 is valid here as i returned s2, otherwise s2 won't be valid {s2}",);
    functions::takes_stack_ownership(y);
    println!("can still use y because y's copy is moved {y}");
    references::calculate_length(&String::from("nikhil"));
    let mut s3 = String::from("nikhil bhatt");
    references::mutable_references(&mut s3);
    println!("{s3}"); // we can't create other references,mutable or immutable if we have one
                      // mutable reference as it may lead to race conditions
    let s2 = references::dangle();
    println!("{s2}");
    slice::first_word(&String::from("Nikhil Bhatt"));
    slice::first(&String::from("Nikhil Bhatt"));

    let new_str = String::from("hello world");
    let sl = &new_str[0..5];
    println!(" this is the string slice {sl}");

    let mut my_str = String::from("github");
    let my_ref = &my_str;
    //  my_str.push_str(" .com"); i cannot mutate the original string after i have taken the
    //  reference, and then use the referenced variable again. This is against the ownership rule
    println!("{my_ref}");
}
