use std::num::ParseIntError;
/// Result 别名，可以省略很多重复的 Err 类型声明
type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply(first: &str, second: &str) -> AliasedResult<i32> {
    first
        .parse::<i32>()
        .and_then(|left| second.parse::<i32>().map(|right| left * right))
}

fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

/// Option 的 map、and_then、以及很多其他组合算子也为 Result 实现了
fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
