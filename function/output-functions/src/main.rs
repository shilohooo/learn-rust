fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fn_mut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fn_once() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

/// # 闭包作为输出参数
/// 闭包作为输入参数是可能的，所以返回闭包作为输出参数（output parameter）也应该是可能的。
///
/// 然而返回闭包类型会有问题，因为目前 Rust 只支持返回具体（非泛型）的类型。
///
/// 按照定义，匿名的闭包的类型是未知的，所以只有使用impl Trait才能返回一个闭包。
///
/// 返回闭包的有效特征是：
/// - `Fn`
/// - `FnMut`
/// - `FnOnce`
///
/// 除此之外，还必须使用 move 关键字，它表明所有的捕获都是通过值进行的。
///
/// 这是必须的，因为在函数退出时，任何通过引用的捕获都被丢弃，在闭包中留下无效的引用。
fn main() {
    let plain_fn = create_fn();
    let mut fn_mut = create_fn_mut();
    let fn_once = create_fn_once();

    plain_fn();
    fn_mut();
    fn_once();
}
