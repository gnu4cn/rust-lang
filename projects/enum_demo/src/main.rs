enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write (String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 方法体将定义在这里
    }
}

fn main() {

    let some_numer = Some(5);
    let some_string = Some("一个字符串");

    let absent_number: Option<i32> = None;
}
