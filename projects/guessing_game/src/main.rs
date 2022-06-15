use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println! ("猜出这个数来！");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println! ("随机生成的秘密数字为：{}", secret_number);

    println! ("请输入你猜的数。");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("读取行失败......");

    println! ("你猜的数为：{}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println! ("太小了！"),
        Ordering::Greater => println! ("太大了！"),
        Ordering::Equal => println! ("你赢了！"),
    }
}
