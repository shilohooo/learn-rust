use std::process::Command;

/// # 进程 - 等待
///
/// 如果你想等待一个 `process::Child` 完成，就必须调用 `Child::wait`，
/// 这会返回一个 `process::ExitStatus`。
fn main() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let status = child.wait().unwrap();
    println!("{}", status);
    println!("reached end of main")
}
