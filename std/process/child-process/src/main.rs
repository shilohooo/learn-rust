use std::process::Command;

/// # 子进程
///
/// `process::Output` 结构体表示已结束的子进程（child process）的输出，
/// 而 `process::Command` 结构体是一个进程创建者（process builder）。
fn main() {
    // 指定一个命令，然后用子进程去执行
    let output = Command::new("rustc")
        // .arg("--help")
        // 测试错误的参数
        .arg("--123")
        .output()
        .unwrap_or_else(|e| {
            panic!("Failed to execute process: {}", e);
        });
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("rust succeeded and stdout was:\n{}", s);
    } else {
        let e = String::from_utf8_lossy(&output.stderr);
        println!("rust failed and stderr was:\n{}", e);
    }
}
