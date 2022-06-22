fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_bake(s2);

    println! ("{}, {}", s1, s3);
}

fn gives_ownership() -> String {
    String::from("归你了")
}

fn takes_and_gives_bake(a_string: String) -> String {
    a_string
}
