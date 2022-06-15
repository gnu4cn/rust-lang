use std::io;

fn main() {
    println! ("猜出这个数来！");

    println! ("请输入你猜的数。");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("读取行失败");

    println! ("你猜的数为：{}
        这个游戏的名字叫做：{}
        是要猜出 {} 以内的数来",
              guess, 
              "猜数游戏", 
              100);
}
