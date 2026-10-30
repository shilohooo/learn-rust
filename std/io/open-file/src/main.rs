use std::fs::File;
use std::io::Read;
use std::path::Path;

/// # 打开文件 open
/// `open` 静态方法能够以只读模式（read-only mode）打开一个文件。
///
/// `File` 拥有资源，即文件描述符（file descriptor），它会在自身被 `drop` 时关闭文件。
fn main() {
    let path = Path::new("src/hello.txt");
    let display = path.display();
    // 以只读方式打开路径
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("Can not open file: {}: {:?}", display, e.to_string()),
    };
    // 读取文件内容到一个字符串里面
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => println!("{} contains:\n{}", display, content),
        Err(e) => panic!("Can not read {}: {:?}", display, e),
    }

    // `file` 离开作用域，并且 `src/hello.txt` 文件将被关闭。
}
