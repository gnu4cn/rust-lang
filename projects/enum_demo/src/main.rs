fn main() {
    let config_max = Some(3u8);
    let config_none: Option<u8> = None;

    if let None =  config_none {
        println! ("极大值未被设置");
    }

    if let Some(max) =  config_max {
        println! ("极大值被设置为了 {}", max);
    }
}
