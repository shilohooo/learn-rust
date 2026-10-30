fn give_princess(gift: &str) {
    // 报错没处理的话会导致程序退出
    if gift == "snake" { panic!("AAAaaaa!!!"); }
    println!("I love {}s", gift);
}

/// 错误处理 - panic，报错/抛出异常
fn main() {
    give_princess("teddy bear");
    give_princess("snake");
    give_princess("gift");
}
