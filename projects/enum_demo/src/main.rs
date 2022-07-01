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

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}
