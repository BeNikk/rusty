mod maps;
mod vecs;

fn main() {
    let mut vec = Vec::new();
    for i in 1..10 {
        vec.push(i);
    }
    println!("{:?}", vec);
    let new_vec = vecs::return_even(vec);
    println!("{:?}", new_vec);
    let mut users = maps::create_map();
    println!("{:?}", users);
    users.insert(String::from("new_user1"), 32);
    let first_user_age = users.get("new_user1");
    match first_user_age {
        Some(value) => println!("{value}"),
        None => println!("no such user"),
    }
    let new_vec = vec![(String::from("user"), 33), (String::from("user2"), 44)];
    let hash_maps_users = maps::group_by_key(new_vec);
    println!("{:?}", hash_maps_users);
}
