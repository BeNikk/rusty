mod vecs;
fn main() {
    let mut vec = Vec::new();
    for i in 1..10 {
        vec.push(i);
    }
    println!("{:?}", vec);
    let new_vec = vecs::return_even(vec);
    println!("{:?}", new_vec);
}
