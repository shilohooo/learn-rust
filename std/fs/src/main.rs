use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix;
use std::path::Path;
use std::{fs, io};

/// 实现一个简单的 `cat path`
fn cat(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// 实现一个 `echo s > path`
fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(s.as_bytes())
}

/// 实现一个 `touch path`，忽略已经存在的文件
fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

/// 文件系统操作（fs）
///
/// `std::io::fs` 模块包含几个处理文件系统的函数
fn main() {
    // 创建一个目录
    match fs::create_dir("a") {
        Ok(_) => {}
        Err(e) => println!("目录创建失败：{:?}", e.kind()),
    }

    // 递归创建一个目录
    fs::create_dir_all("a/b/c/d").unwrap_or_else(|e| {
        println!("递归创建一个目录失败：{}", e);
    });

    // 创建一个符号链接
    if cfg!(target_family = "unix") {
        unix::fs::symlink("src/Hello.txt", "src/HelloLink.txt").unwrap_or_else(|e| {
            println!("创建符号链接失败：{:?}", e.kind());
        })
    }

    // 读取目录内容
    match fs::read_dir("src") {
        Ok(paths) => {
            for path in paths {
                println!("> {:?}", path.unwrap().path());
            }
        }
        Err(e) => println!("读取目录内容失败：{:?}", e.kind()),
    }

    match cat(Path::new("src/Hello.txt")) {
        Ok(content) => println!("File content:\n{}", content),
        Err(e) => panic!("Can't read file content: {:?}", e),
    }

    match echo("Hi", Path::new("src/test.txt")) {
        Ok(_) => println!("success"),
        Err(e) => panic!("Can't write to file: {:?}", e),
    }

    match touch(Path::new("src/new.txt")) {
        Ok(_) => println!("文件创建成功"),
        Err(e) => panic!("文件创建失败：{:?}", e),
    }

    // 删除一个文件
    fs::remove_file("a/b/c/happy.log").unwrap_or_else(|e| {
        println!("文件删除失败：{:?}", e.kind());
    });
    // 删除一个目录，目录下面有子文件/文件夹，则会删除失败
    fs::remove_dir("a/b/c/d").unwrap_or_else(|e| {
        println!("删除一个目录失败：{:?}", e.kind());
    });
}
