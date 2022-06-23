fn main() {
    let s1 = String::from("hello");

    let (s2, len): (String, usize) = calculate_length(s1);

    println! ("字符串 {} 的长度为：{}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
