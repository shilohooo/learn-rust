use std::error::Error;
use std::fmt::{Display, Formatter};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct EmptyVec;

impl Display for EmptyVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl Error for EmptyVec {}

/// 如果在错误可转换成返回类型的地方使用 ?，它将自动转换成返回类型。
fn double_first(vec: Vec<&str>) -> Result<i32> {
    // 使用 `?` 立即得到内部值，而不是把所有的 Result 和 Option 串起来
    // 要么转换得到一个字符串，要么返回 Error
    let first = vec.first().ok_or(EmptyVec)?;
    // 要么转换得到一个数字，要么返回 Error
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

/// ? 的其他用法
fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
