use std::io;

fn main() {
    println! ("猜出这个数来！");

    println! ("请输入你猜的数。");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("读取行失败......");

    println! ("你猜的数为：{}", guess);
}
