fn main() {
    let mut sentence = String::from("hello world");
    let word = first_word(&sentence);
    // sentence.clear(); - protection from this by compiler rule - cannot have mutable reference when there is/are immutable references..
    println!("{}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
