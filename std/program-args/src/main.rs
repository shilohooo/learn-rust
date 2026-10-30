use std::env;

/// # 程序参数
///
/// ## 标准库
///
/// 命令行参数可使用 std::env::args 进行接收，这将返回一个迭代器，
/// 该迭代器会对每个参数举出一个字符串。
///
/// ## Crate
///
/// `clap`: 最流行的命令行参数解析库
fn main() {
    let args: Vec<String> = env::args().collect();
    // 第一个参数为调用本程序的相对路径
    println!("My path is: {}", args[0]);
    // 其余的参数都是被传递给程序的命令行参数
    println!("I got {:?} arguments: {:?}", args.len() - 1, &args[1..]);
}
