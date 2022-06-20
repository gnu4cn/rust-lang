fn main() {
    let s = "hello";    // 这里 s 的类型为：&s
    println! ("{}", s);

    let mut s = String::from("hello");  // 这里 s 的类型为：String
    println! ("{}", s);

    s.push_str(", world!");
    println! ("{}", s);
}
