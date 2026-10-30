use std::error::Error;
use std::fmt::{Display, Formatter};

type Result<T> = std::result::Result<T, DoubleError>;

/// 自定义错误类型
#[derive(Debug, Clone)]
struct DoubleError;

impl Display for DoubleError {
    /// 给自定义错误类型实现它要显示的错误信息
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid first item to double")
    }
}

/// 为 `DoubleError` 实现 `Error` trait，这样其他错误可以包裹这个错误类型
impl Error for DoubleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // 把错误指定为自定义错误类型
        .ok_or(DoubleError)
        .and_then(|f| {
            f.parse::<i32>()
                // 这里也换成自定义类型
                .map_err(|_| DoubleError)
                .map(|num| 2 * num)
        })
}

fn log(result: Result<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

/// # 定义一个错误类型
///
/// 有时候把所有不同的错误都视为一种错误类型会简化代码。我们将用一个自定义错误类型来演示这一点。
///
/// Rust 允许我们定义自己的错误类型。一般来说，一个 “好的” 错误类型应当：
///
/// * 用同一个类型代表了多种错误
/// * 向用户提供了清楚的错误信息
/// * 能够容易地与其他类型比较
///     - 好的例子：`Err(EmptyVec)`
///     - 坏的例子：`Err("Please use a vector with at least one element".to_owned())`
/// * 能够容纳错误的具体信息
///     - 好的例子：`Err(BadChar(c, position))`
///     - 坏的例子：`Err("+ cannot be used here".to_owned())`
/// * 能够与其他错误很好地整合
fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    log(double_first(numbers));
    log(double_first(empty));
    log(double_first(strings));
}
