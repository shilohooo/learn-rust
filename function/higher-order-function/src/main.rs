fn is_odd(n:u32) -> bool {
    n % 2 != 1
}

/// # Rust 高阶函数
///
/// Rust 提供了高阶函数（Higher Order Function, HOF），
///
/// 指那些输入一个或多个函数，并且/或者产生一个更有用的函数的函数。
///
/// HOF 和惰性迭代器（lazy iterator）给 Rust 带来了函数式（functional）编程的风格。
fn main() {
    println!("Find the sum of all the squared odd numbers under 1000");

    let upper = 1000;
    // 命令式写法
    let mut acc = 0;
    // 迭代：0, 1, 2 ... 到无穷大
    for n in 0.. {
        let n_squared = n * n;
        if n_squared>=upper {
            break
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }

    println!("imperative style: {}", acc);

    // 函数式写法
    let sum: u32 =
        (0..)
            // 所有自然数取平方
            .map(|x| x * x)
            // 取小于上限的
            .take_while(|&x| x < upper)
            // 取奇数
            .filter(|&x| is_odd(x))
            // 累加
            .fold(0, |sum, i| sum + i);
    println!("functional style: {}", sum);
}
