/// Rust 变量绑定
fn main() {
    // 使用 `let` 关键字将值绑定到变量，大多数情况可以不显式指定数据类型，Rust 会自动推导
    let a_int = 1u32;
    println!("{}", a_int);
    let a_bool = true;
    println!("{}", a_bool);
    let a_unit = ();
    println!("{:?}", a_unit);

    // 复制变量
    let copied_int = a_int;
    println!("{}", copied_int);

    // 没有使用过的变量，编译器会产生一个 Variable xxx is never used 警告，可以通过使用下划线开头的变量名称来消除该警告
    let _unused_int = 3u32;
}
