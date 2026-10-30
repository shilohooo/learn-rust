use std::io::{Read, Write};
use std::process::{Command, Stdio};

static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";

/// 管道（pipe）
///
/// `std::Child` 结构体代表了一个正在运行的子进程，它暴露了 `stdin`（标准输入），
/// `stdout`（标准输出）和 `stderr`（标准错误）句柄，从而可以通过管道与所代表的进程交互。
fn main() {
    // 启动一个命令
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(process) => process,
        Err(e) => panic!("Couldn't spawn wc: {:?}", e),
    };
    // 写入内容
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Ok(_) => println!("sent pangram to wc"),
        Err(e) => panic!("Couldn't write to wc stdin: {:?}", e),
    }
    // 读取内容，stdin 在上面调用后会被销毁，管道也随之关闭
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        // 这里会输出读取到的内容有多少行、有多少个单词、有多少字节
        Ok(_) => println!("wc responded with:\n{}", s),
        Err(e) => panic!("Couldn't read wc stdout: {:?}", e),
    }
}
