fn main() {
    let v = vec! [1, 2, 3, 4];

    let third: &i32 = &v[2];
    println! ("第三个元素为 {}", third);

    match v.get(2) {
        Some(third) => println! ("第三个元素为 {}", third),
        None => println! ("没有第三个元素。"),
    }

    // dbg! (v);
}
