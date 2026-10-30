use std::fmt::{Debug, Display};

struct S<T: Display>(T);

/// 这里用泛型约束 T 类型必须实现 Display trait
fn printer<T: Display>(t: T) {
    println!("{}", t);
}

trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

/// 泛型 T 必须实现 Debug trait
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

/// 泛型 T 必须实现 HasArea trait
fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

// region 空约束

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

/// 这些函数只对实现了相应的 trait 的类型有效。
///
/// 事实上这些 trait 内部是空的，但这没有关系。
fn red<T: Red>(_: &T) -> &'static str {
    "red"
}

fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

// endregion

/// # 泛型约束
///
/// 在使用泛型时，类型参数常常必须使用 trait 作为约束（bound）来明确规定类型应实现哪些功能。
///
/// 约束的另一个作用是泛型的实例可以访问作为约束的 trait 的方法。
///
/// 约束的工作机制会产生这样的效果：即使一个 trait 不包含任何功能，你仍然可以用它作为约束。
///
/// 标准库中的 Eq 和 Ord 就是这样的 trait。
fn main() {
    // 下面这个语句会报错：`Vec<{integer}>` doesn't implement `std::fmt::Display` [E0277]
    // the trait `std::fmt::Display` is not implemented for `Vec<{integer}>`
    // let s = S(vec![1]);

    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };

    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    // 下面这个语句会报错：Trait `Debug` is not implemented for `Triangle` [E0277]
    // print_debug(&_triangle);
    // 下面这个语句会报错：Trait `HasArea` is not implemented for `Triangle` [E0277]
    // println!("Area: {}", area(&_triangle));

    // region 空约束

    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    // 下面这个语句会报错：Trait `Red` is not implemented for `Turkey` [E0277]
    // println!("A turkey is {}", red(&_turkey));

    // endregion
}
