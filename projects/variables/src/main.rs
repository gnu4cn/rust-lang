fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println! ("内部作用域中 x 的值为：{}", x);
    }

    println! ("x 的值为：{}", x);
    
    let spaces: &str = "    ";
    // let mut spaces = "    ";
    let spaces: usize = spaces.len();

    println! ("spaces 为：{}", spaces);

    let guess: i32 = "42".parse().expect("那不是个数字！");
}
