fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;

    println! ("{}, {} 与 {}", r1, r2, r3);
}
