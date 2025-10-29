use std::vec;

pub fn sum_of_vec(vector: Vec<i32>) -> i32 {
    let v1_iter = vector.iter();
    let sum = v1_iter.sum();
    sum
    // these consume the iterator
}
pub fn adapter(vector: Vec<i32>) {
    let iter = vector.iter();
    let iter2 = iter.map(|x| x + 1);
    for x in iter2 {
        println!("adapter {}", x);
    }
    let new_iter = vector.iter();
    let iter3 = new_iter.filter(|x| *x % 2 == 0);
    for x in iter3 {
        println!("{}", x);
    }
}
