// slice is a kind of reference, it does not have ownership

pub fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("{item}");
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

pub fn first(s: &String) {
    let mut new_str = String::from("");
    for c in s.chars() {
        println!("{c}");
        if c.to_string() == " " {
            println!("this is where the string has its first word");
            println!("{new_str}");
            break;
        }
        new_str.push(c);
    }
}
