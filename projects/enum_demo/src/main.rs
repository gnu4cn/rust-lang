fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(n) => Some(n + 1),
    }
}

fn main() {
    let five = Some(5);
    let none = None;
    println! ("{:?}, {:?}", plus_one(five), plus_one(none));
}
