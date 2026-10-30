fn call<F:Fn()>(f: F) {
    f();
}

fn my_fn() {
    println!("I am a function!")
}

/// 将函数作为参数
///
/// 如果你声明一个接受闭包作为参数的函数，
///
/// 那么任何满足该闭包的 trait 约束的函数都可以作为其参数。
///
/// `Fn`、`FnMut` 和 `FnOnce` 这些 `trait` 明确了闭包如何从周围的作用域中捕获变量。
fn main() {
    call(my_fn);
    call(|| println!("I am a closure!"));
}
