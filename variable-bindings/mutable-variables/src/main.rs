/// Rust 可变变量
fn main() {
    // 变量默认不可变（Immutable），如果需要改变，则要使用 mut 关键字来修饰变量
    let _immutable_var = 1;
    let mut  mutable_var = 1;
    println!("Before mutation: {}", mutable_var);
    // 修改变量
    mutable_var += 1;
    println!("After mutation: {}", mutable_var);

    // 以下语句会导致编译错误
    // _immutable_var += 1;
}
