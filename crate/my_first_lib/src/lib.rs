/// 公有函数，仅用于函数调用测试
pub fn pub_fn() {
    println!("called Shiloh's `pub_fn()`");
}

/// 私有函数
fn private_fn() {
    println!("called Shiloh's `private_fn()`");
}

/// 公有函数
///
/// 该函数内部会调用模块的私有函数进行测试
pub fn indirect_access() {
    print!("called Shiloh's `indirect_access()`, that\n> ");
    private_fn();
}

/// `#[cfg(test)]` 是 Rust 中的条件编译（`conditional compilation`）
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        pub_fn();

        indirect_access();
    }
}