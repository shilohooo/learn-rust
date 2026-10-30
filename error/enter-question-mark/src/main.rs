use std::num::ParseIntError;

fn multiply(first: &str, second: &str) -> Result<i32, ParseIntError> {
    println!("parse first number");
    let left = first.parse::<i32>()?;
    println!("parse second number");
    let right = second.parse::<i32>()?;
    println!("calc and return");
    Ok(left * right)
}

/// 在 ? 出现以前，相同的功能是使用 try! 宏完成的。现在我们推荐使用 ? 运算符，
/// 但是在老代码中仍然会看到 try!。
/// 如果使用 try! 的话，上一个例子中的 multiply 函数看起来会像是这样：
fn multiply_with_try(first: &str, second: &str) -> Result<i32, ParseIntError> {
    println!("parse first number");
    let left = r#try!(first.parse::<i32>());
    println!("parse second number");
    let right = r#try!(second.parse::<i32>());
    println!("calc and return");
    Ok(left * right)
}

fn log(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

/// 引入 ?
/// 有时我们只是想 unwrap 且避免产生 panic。
/// 到现在为止，对 unwrap 的错误处理都在强迫我们一层层地嵌套，
/// 然而我们只是想把里面的变量拿出来。? 正是为这种情况准备的。
///
/// 当找到一个 Err 时，可以采取两种行动：
///
/// 1. panic!，不过我们已经决定要尽可能避免 panic 了。
/// 2. 返回它，因为 Err 就意味着它已经不能被处理了。
///
/// ? 几乎1 就等于一个会返回 Err 而不是 panic 的 unwrap。
fn main() {
    log(multiply("10", "2"));
    log(multiply("t", "2"));

    println!(">========================<");

    log(multiply_with_try("10", "2"));
    log(multiply_with_try("t", "2"));
}
