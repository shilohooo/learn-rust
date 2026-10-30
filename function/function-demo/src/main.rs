/// 函数（function）使用 `fn` 关键字声明
///
/// 函数的参数需要标注明确的类型，如果函数有返回值，则返回值的类型必须在 `->` 箭头之后指定
///
/// 函数最后的表达式将作为返回值，可以省略 `return` 关键字。
///
/// 也可以使用 `return` 关键字来提前返回一个值，包括在 循环/if 判断里面
///
/// 函数的定义顺序不影响它的使用，不是非要先定义在前面，再使用～
fn main() {
    fizzbuzz_to(100);
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    };

    // 这是一个表达式，不用 return 关键字也可以
    lhs % rhs == 0
}

/// 如果一个函数没有返回值，它的返回值实际上为单元类型：`()`，也就是 `Unit`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

/// 当函数返回 `()` 时，函数签名可以省略返回类型
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}
