fn main() {
    let mut s = String::from("The quick brown fox jumps over the lazy dog.");

    let word = first_word(&s);

    s.clear();

    println! ("首个单词为：{}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
