pub fn return_even(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for i in vec {
        if i % 2 == 0 {
            new_vec.push(i);
        }
    }
    new_vec
}
