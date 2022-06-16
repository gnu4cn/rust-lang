fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println! ("内部作用域中 x 的值为：{}", x);
    }

    println! ("x 的值为：{}", x);
    
    let mut spaces = "    ";
    spaces = spaces.len();

    println! ("spaces 为：{}", spaces);
}
