use std::num::ParseIntError;

fn multiply(first: &str, second: &str) -> Result<i32, ParseIntError> {
    first
        .parse::<i32>()
        .and_then(|left| second.parse::<i32>().map(|right| left * right))
}

fn print(result: Result<i32, ParseIntError>) {
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
