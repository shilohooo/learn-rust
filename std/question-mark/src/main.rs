pub mod checked;

/// # `?` 运算符
///
/// 把 result 用 match 连接起来会显得很难看；幸运的是，`?` 运算符可以把这种逻辑变得干净漂亮。
///
/// `?` 运算符用在返回值为 `Result` 的表达式后面，它等同于这样一个匹配表达式：
/// 其中 `Err(err)` 分支展开成提前返回的 `return Err(err)`，
/// 而 `Ok(ok)` 分支展开成 `ok` 表达式。
fn main() {
    checked::op(1.0, 10.0f64);
}
