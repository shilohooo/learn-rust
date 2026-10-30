fn multiply(first: &str, second: &str) -> i32 {
    let left = first.parse::<i32>().unwrap();
    let right = second.parse::<i32>().unwrap();
    left * right
}

/// Result 是 Option 类型的更丰富的版本，描述的是可能的错误而不是可能的不存在。
///
/// 也就是说，Result<T，E> 可以有两个结果的其中一个：
///
/// Ok<T>：找到 T 元素
/// Err<E>：找到 E 元素，E 即表示错误的类型。
/// 按照约定，预期结果是 “Ok”，而意外结果是 “Err”。
///
/// Result 有很多类似 Option 的方法。例如 unwrap()，它要么举出元素 T，要么就 panic。
/// 对于事件的处理，Result 和 Option 有很多相同的组合算子。
fn main() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt);
}
