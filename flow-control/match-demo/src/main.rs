/// `match` 模式匹配
///
/// `Rust` 通过 `match` 关键字来提供模式匹配，和 `C` 语言的 `switch` 用法类似。
///
/// 第一个匹配分支会被比对，并且所有可能的值都必须被覆盖。
fn main() {
    let number = 13;
    println!("Tell me about {}", number);

    match number {
        1 => println!("One!"),
        // 匹配多个值，
        // 2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // 匹配一个闭区间范围
        13..=19 => println!("A teen"),
        // 处理剩余的情况
        _ => println!("Ain't special"),
    }

    let bool = true;
    // match 也是一个表达式，可以作为赋值语句
    let binary = match bool {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", bool, binary);
}
