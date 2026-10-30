macro_rules! test {
    // 多个参数用分号隔开
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        );
    };
    // 新增一个分支，每个分支都要以分号结束
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        );
    };
}

/// # 宏 - 重载
///
/// 类似方法重载，名称一样，参数列表不一样
fn main() {
    test!(1i32 +1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}
