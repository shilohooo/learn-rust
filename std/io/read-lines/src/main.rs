use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

/// 输出包裹在 Result 中以允许匹配错误，
///
/// 将迭代器返回给文件行的读取器（Reader）
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    // 文件必须存在
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// # 读取行
///
/// 方法 `lines()` 在文件的行上返回一个迭代器。
///
/// `File::open` 需要一个泛型 `AsRef<Path>`。这正是 `read_lines()` 期望的输入。
fn main() {
    if let Ok(lines) = read_lines("src/ip-whitelist.txt") {
        // 使用迭代器，返回一个（可选）字符串
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}
