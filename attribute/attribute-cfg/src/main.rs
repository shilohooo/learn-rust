/// 这个函数仅当目标操作系统是 Linux 的时候才会编译
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running on Linux!");
}

/// 这个函数仅当目标操作系统是 MacOS 的时候才会编译
#[cfg(target_os = "macos")]
fn are_you_on_mac_os() {
    println!("You are running on MacOS!");
}

/// 这个函数仅当目标操作系统不是 Linux 的时候才会编译
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running on Linux!");
}

/// 自定义条件编译
///
/// 自定义条件必须使用 `--cfg` 标记来传给 `rustc`。
#[cfg(some_condition)]
fn condition_fn() {
    println!("condition met!")
}


/// # cfg 条件编译
///
/// 条件编译可能通过两种不同的操作符实现：
///
/// - cfg 属性：在属性位置中使用 #[cfg(...)]
/// - cfg! 宏：在布尔表达式中使用 cfg!(...)
///
/// 两种形式使用的参数语法都相同。
fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    };

    condition_fn();
}
