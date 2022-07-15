fn main() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println! ("极大值被配置为了 {}", max),
        _ => (),
    }
}
