fn main() {
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println! ("极大值被设置为了 {}", max);
    }
}
