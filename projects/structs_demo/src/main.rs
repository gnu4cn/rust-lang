struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let user1 = build_user(
        String::from("rust@xfoss.com"), 
        String::from("rust_xfoss")
    );

    let user2 = User {
        email: String::from("java@xfoss.com"),
        username: String::from("java_xfoss"),
        ..user1
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
