#![allow(dead_code)]

/// 拥有隐式辨别值（implicit discriminator，从 0 开始）的枚举
enum Number {
    Zero,
    One,
    Tow
}

/// 拥有隐式辨别值（explicit discriminator）的枚举
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff
}

/// `enum` 也可以像 C 语言风格的枚举类型那样使用
fn main() {
    // `enum` 可以转为整型
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
