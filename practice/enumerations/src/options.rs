pub fn find_first(a: String) -> Option<i32> {
    for (index, char) in a.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }
    None
}
