use std::collections::HashMap;

pub fn create_map() -> HashMap<String, i32> {
    let mut users: HashMap<String, i32> = HashMap::new();
    users.insert(String::from("new_user"), 21);
    users
}

pub fn group_by_key(pairs: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut mpp = HashMap::new();
    for (key, value) in pairs {
        mpp.insert(key, value);
    }
    mpp
}
