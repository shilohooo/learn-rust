/// 自定义宏
macro_rules! say_hello {
    () => {
        println!("Hello")
    };
}

/// # Rust 宏（macro）
///
/// 元编程，省略大量模版代码
fn main() {
    say_hello!();
}
