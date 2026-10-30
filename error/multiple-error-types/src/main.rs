// fn double_first(vec: Vec<&str>) -> i32 {
//     // 错误 1
//     let first = vec.first().unwrap();
//     // 错误 2
//     2 * first.parse::<i32>().unwrap()
// }

use std::num::ParseIntError;

/// 处理混合错误类型之互相包含
// fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
//     vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
// }

/// 有时候我们不想再处理错误（比如使用 ? 的时候），
/// 但如果 Option 是 None 则继续处理错误。
/// 一些组合算子可以让我们轻松地交换 Result 和 Option。
fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));
    opt.map_or(Ok(None), |r| r.map(Some))
}

/// 有时 Option 需要和 Result 进行交互，或是 Result<T, Error1> 需要和 Result<T, Error2> 进行交互。
/// 在这类情况下，我们想要以一种方式来管理不同的错误类型，使得它们可组合且易于交互。
fn main() {
    let numbers = vec!["42", "83", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "33", "44"];

    println!("The first doubled is {:?}", double_first(numbers));
    println!("The first doubled is {:?}", double_first(empty));
    println!("The first doubled is {:?}", double_first(strings));
}
