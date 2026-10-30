macro_rules! create_fn {
    ($fn_name:ident) => {
        // 创建一个函数，它的名称是 $fn_name
        fn $fn_name() {
            // stringify! 宏把 ident 转换成字符串
            println!("You called {:?}", stringify!($fn_name));
        }
    };
}

// 使用上面的 create_fn 宏来创建函数
create_fn!(foo);
create_fn!(bar);

macro_rules! print_result {
    // expr 指示符表示一个表达式，比如 1 + 1
    ($expr:expr) => {
        println!("{:?} = {:?}", stringify!($expr), $expr)
    };
}

/// # 宏 - 指示符（designators）
///
/// 宏的参数约定以 $ 符号开头，然后使用下面的语法来标注类型：
///
/// ```
/// ($func_name:ident) => ()
/// ```
///
/// ident 是一个指示符，用在变量名或函数名
///
/// 这里列出全部指示符：
///
/// - block
/// - expr 用于表达式
/// - ident 用于变量名或函数名
/// - item
/// - literal 用于字面常量
/// - pat (模式 pattern)
/// - path
/// - stmt (语句 statement)
/// - tt (标记树 token tree)
/// - ty (类型 type)
/// - vis (可见性描述符)
///
/// 完整列表详见 [Rust Reference](https://doc.rust-lang.org/reference/macros-by-example.html)。
fn main() {
    foo();
    bar();

    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    })
}
