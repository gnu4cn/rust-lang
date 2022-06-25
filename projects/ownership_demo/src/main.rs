fn main() {
    let s = String::from("The quick brown fox jumps over the lazy dog.");

    let word = first_word(&s);

    println! ("{}", word);
    println! ("{}", &s[4..9]);


    let s = String::from("这是一个示例。");

    println! ("{}", first_word(&s));
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
