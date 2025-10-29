mod iterator;
fn main() {
    let mut my_vec = vec![1, 2, 3];
    let iterator = my_vec.iter();
    for value in iterator {
        println!("{}", value);
    }
    // iterator borrows the value from vec immutably
    // to have  a mutable borrow
    let mut_iterator = my_vec.iter_mut();
    for val in mut_iterator {
        *val = *val + 1;
    }
    println!("{:?}", my_vec);
    iterator::iterate(&my_vec);

    // into iter takes ownership of the original value
    let v1_iter = my_vec.into_iter();
    for val in v1_iter {
        println!("{}", val);
    }
    // cannot use my_vec here
}
