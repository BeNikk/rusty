fn main() {
    let mut str = String::from("Hello,world");
    println!("name is {}", str);
    str.push_str("learn rust");
    let first_str = first_word(str);
    println!("{first_str}");
    let str = String::from("hello bye world");
    let word = first_word_slices(&str);
    println!(" the first word is {word}");
}

// this program has 2 issues -> we take double the amount of
// memory
// 2- Important- when the original str gets cleared, the value of the first_word still remains
// since it is a different variable altogether, this should not be the case and therefore we have
// slices; the first_str should point to the original string instead of creating a new one.
fn first_word(str: String) -> String {
    let mut ans = String::from("");
    for i in str.chars() {
        if i == ' ' {
            break;
        }
        ans.push(i);
    }
    ans
}

//corrected &str -> string slice

fn first_word_slices(str: &String) -> &str {
    let mut index = 0;
    for (_, i) in str.chars().enumerate() {
        if i == ' ' {
            break;
        }
        index += 1;
    }
    &str[0..index]
}
