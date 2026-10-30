/// Rust 原生类型
fn main() {
    // 显式类型标注
    let logical: bool = true;
    let num: f64 = 1.0;
    // 数字可以使用后缀去标注类型
    let integer = 5i32;

    // 默认的类型推断
    // 浮点数默认类型为：f64
    let a = 3.1;
    // 整数默认类型为：i32
    let b = 233;

    // 根据上下文来推断类型
    let mut inferred_type = 12;
    inferred_type = 4294967296i64;

    // 可变变量，Rust 默认不可变，加上 mut 关键字才能修改变量的值
    let mut mutable = 12;
    mutable = 2333;

    // 但是不能改变量的类型
    // mutable = false;

    // 重复声明相同名称的变量，会覆盖（遮蔽：shadow）前面的
    let mutable = false;
}
