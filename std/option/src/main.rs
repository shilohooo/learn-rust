fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn try_division(dividend: i32, divisor: i32) {
    match checked_division(dividend, divisor) {
        Some(quotient) => println!("{} / {} = {}", dividend, divisor, quotient),
        None => println!("{} / {} failed", dividend, divisor),
    }
}

/// # 选项 `Option`
///
/// 有时候想要捕捉到程序某部分的失败信息，而不是调用 `panic!`；这可使用 `Option`
/// 枚举类型来实现。
///
/// `Option<T>` 有两个变量：
///
/// * `None`，表明失败或缺少值
/// * `Some(value)`，元组结构体，封装了一个 `T` 类型的值 `value`
fn main() {
    try_division(4, 2);
    try_division(1, 0);

    // 绑定 `None` 到一个变量需要显式类型标注
    let none: Option<i32> = None;
    let _eq_none = None::<i32>;

    let optional_float = Some(0f32);
    println!(
        "{:?} unwraps to {:?}",
        optional_float,
        optional_float.unwrap()
    );

    // 解包 None 会引发 panic
    println!("{:?} unwraps to {:?}", none, none.unwrap());
}
