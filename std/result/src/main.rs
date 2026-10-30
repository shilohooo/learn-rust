pub mod checked;

fn op(x: f64, y: f64) -> f64 {
    match checked::div(x, y) {
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            },
        },
        Err(why) => panic!("{:?}", why),
    }
}

/// # 结果 `Result`
///
/// 我们已经看到 `Option` 枚举类型可以用作可能失败的函数的返回值，其中返回 `None` 可以表明失败。但是有时要强调**为什么**一个操作会失败。为做到这点，我们提供了 `Result` 枚举类型。
///
/// `Result<T, E>` 类型拥有两个取值：
///
/// * `Ok(value)` 表示操作成功，并包装操作返回的 `value`（`value` 拥有 `T` 类型）。
/// * `Err(why)`，表示操作失败，并包装 `why`，它（但愿）能够解释失败的原因（`why` 拥有 `E` 类型）。
fn main() {
    println!("{}", op(1.0, 10.0));
}
