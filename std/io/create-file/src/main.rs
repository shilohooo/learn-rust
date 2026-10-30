use std::fs::File;
use std::io::Write;
use std::path::Path;

/// 文件内容
static LOREM_IPSUM: &'static str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

/// # 创建文件 create
///
/// create 静态方法以只写模式（write-only mode）打开一个文件。
/// 若文件已经存在，则旧内容将被销毁。否则，将创建一个新文件。
fn main() {
    let path = Path::new("out/lorem_ipsum.txt");
    let display = path.display();
    // 以只写模式打开文件
    let mut file = match File::create(&path) {
        Ok(f) => f,
        Err(e) => panic!("Can not create {}: {:?}", display, e),
    };
    // 写入文件内容
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Ok(_) => println!("successfully wrote to {}", display),
        Err(e) => panic!("Can not write to {}: {:?}", display, e),
    }
}
