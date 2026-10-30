#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}

/// # 作用域规则 - 生命周期 - trait
///
/// trait 方法中生命期的标注基本上与函数类似。注意，impl 也可能有生命周期的标注。
fn main() {
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}
