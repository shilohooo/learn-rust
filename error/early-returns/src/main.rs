use std::num::ParseIntError;

type ParseIntResult<T> = Result<T, ParseIntError>;

fn multiply(first: &str, second: &str) -> ParseIntResult<i32> {
    println!("parse first num");
    let left = match first.parse::<i32>() {
        Ok(num) => num,
        // 提前返回，下面的代码不会继续执行
        Err(e) => return Err(e),
    };
    println!("parse second num");
    let right = match second.parse::<i32>() {
        Ok(num) => num,
        Err(e) => return Err(e),
    };

    Ok(left * right)
}

fn log(result: ParseIntResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

/// 提前返回
/// 使用 match 语句和提前返回（early return）的结合。
/// 这也就是说，如果发生错误，可以停止函数的执行然后返回错误。
/// 对有些人来说，这样的代码更好写，更易读。
fn main() {
    log(multiply("10", "2"));
    log(multiply("t", "2"));
}
