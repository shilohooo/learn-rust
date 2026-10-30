/// 全局变量 - 在所有其他作用域之外声明
static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

/// 在函数作用域里面访问常量
fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

/// Rust 中的常量可以在任意作用域声明，包括全局作用域。
///
/// - const = 不可改变的值（常用）
/// - static = 具有 `'static` 生命周期的，它可以是可变的变量（需要使用 `static mut` 关键字）
///
/// 有个特例就是 `"string"` 字面量。它可以不经改动就被赋给一个 `static` 变量，
///
/// 因为它的类型标记：`&'static str` 就包含了所要求的生命周期 'static。
///
/// 其他的引用类型都必须特地声明，使之拥有 `'static` 生命周期。
///
/// 这两种引用类型的差异似乎也无关紧要，因为无论如何，`static` 变量都得显式地声明。
fn main() {
    println!("Language: {}", LANGUAGE);
    println!("Threshold: {}", THRESHOLD);
    println!("is big: {}", is_big(20));
    println!("is big: {}", is_big(1));

    // 不能修改一个常量的值！！！
    // THRESHOLD = 5;
}
