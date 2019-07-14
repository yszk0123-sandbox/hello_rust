fn main() {
    let mut s = String::from("Hello");

    s.push_str(", world!");

    println!("{}", s);

    let s = get_string();
    println!("{}", s);

    let word = first_word(&s);
    println!("\"{}\"", word);
}

fn get_string() -> String {
    String::from("Hello, world!")
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
