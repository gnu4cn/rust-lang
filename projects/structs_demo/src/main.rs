struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let user1 = User {
        email: String::from("rust@xfoss.com"),
        username: String::from("unisko"),
        active: true,
        sign_in_count: 1
    };
}
