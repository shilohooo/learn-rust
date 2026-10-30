/// Rust 中的嵌套循环和标签
///
/// 处理嵌套循环的时候，`break` 可以用来跳出当前的循环，回到外层循环
///
/// `continue` 可以跳过本次循环，就近原则（在内层循环使用就是跳过本地内层循环）
///
/// 还可以使用 'label 语法（单引号开头加上自定义标签名称）来控制 `break` 和 `continue` 的位置
fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // 中断内层循环
            // break;

            // 中断外层循环
            break 'outer;
        }

        println!("This point will never reached");
    }

    println!("Exited the outer loop");
}
