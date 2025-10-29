pub fn iterate(vector: &Vec<i32>) {
    let mut iter = vector.iter();
    while let Some(val) = iter.next() {
        println!("{}", val);
    }
}
