use std::fmt::{Display, Formatter};
use std::str::FromStr;

struct Circle {
    radius: i32,
}

/// 要把任何类型转换成 `String`，只需要实现那个类型的 `ToString trait`。
///
/// 然而不要直接这么做，应该实现 `fmt::Display trait`，它会自动提供 ToString，
///
/// 并且还可以用来打印类型
impl Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle of radius = {}", self.radius)
    }
}

impl FromStr for Circle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let radius = s.parse::<i32>();
        match radius {
            Ok(r) => Ok(Circle { radius: r }),
            Err(_) => Err(()),
        }
    }
}

// 编译器会产生一个警告提示不要直接实现 ToString，应改为实现 Display
// impl ToString for Circle{
//     fn to_string(&self) -> String {
//         format!("Circle of radius = {}", self.radius)
//     }
// }

/// `ToString` 和 `FromStr`
fn main() {
    let circle = Circle { radius: 3 };
    println!("{}", circle);
    // println!("{}", circle.to_string());

    // 解析字符串转换为数字 - 显式指定转换的变量类型
    let parsed: i32 = "5".parse().unwrap();
    println!("{}", parsed);
    // 通过泛型指定要转换的类型
    let turbo_parsed = "10".parse::<i32>().unwrap();
    println!("{}", turbo_parsed);

    let sum = parsed + turbo_parsed;
    println!("{} + {} = {}", parsed, turbo_parsed, sum);

    // 如果要转换到用户定义类型，只要手动实现 FromStr 就行
    let parsed_circle = "123".parse::<Circle>();
    match parsed_circle {
        Ok(circle) => println!("{}", circle),
        Err(_) => println!("invalid radius"),
    }
}
