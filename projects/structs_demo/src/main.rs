struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let user1 = build_user(
        String::from("rust@xfoss.com"), 
        String::from("rust_xfoss"));

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("java@xfoss.com"),
        sign_in_count: user1.sign_in_count,
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
