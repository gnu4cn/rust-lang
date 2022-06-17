fn main() {
    let tup = (1, 0.5, "元组元素");

    println! ("tup.1：{}", tup.1);

    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    println! ("七月：{}", months[6]);
}
