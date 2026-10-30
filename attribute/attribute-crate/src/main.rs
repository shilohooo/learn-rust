// 作用于整个 crate 的属性，语法为 #![xxx]
// 使用了 `#![crate_type]` 属性后，编译时无需加 --crate-type=xxx 命令
#![crate_type = "bin"]
#![crate_name = "attribute_crate"]

/// `crate_type` 属性可以告知编译器 `crate` 是一个二进制的可执行文件还是一个库（甚至是哪种类型的库），
///
/// `crate_name` 属性可以设定 `crate` 的名称。
///
/// 不过，一定要注意在使用 `cargo` 时，这两种类型时都没有作用。
///
/// 由于大多数 Rust 工程都使用 `cargo`，这意味着 `crate_type` 和 `crate_name` 的作用事实上很有限。
fn main() {
    println!("Hello, world!");
}
