/// 给自定义类型定义转换行为
#[derive(Debug)]
struct Number {
    val: i32,
}

impl From<i32> for Number {
    /// 指定的数据类型要如何转换为 `Number` 结构体？
    /// # Arguments
    /// - val 指定数据类型的值
    /// # Returns
    /// `Number` 结构体实例
    fn from(val: i32) -> Self {
        Number { val }
    }
}

/// Rust 类型转换 - `From` 和 `Into`
fn main() {
    let my_str = "Hello, world!";
    println!("{}", my_str);
    // From trait 允许一种类型定义 “怎么根据另一种类型生成自己”，它提供了一种类型转换的简单机制
    let my_string = String::from(my_str);
    println!("{}", my_string);

    let num = Number::from(233);
    println!("My number is: {:?}", num.val);

    let integer = 5;
    // Into trait 就是把 From trait 倒过来而已。
    // 也就是说，如果你为你的类型实现了 From，那么同时你也就免费获得了 Into。
    // 使用 Into trait 通常要求指明要转换到的类型，因为编译器大多数时候不能推断它。
    // 下面这个语句删除类型声明会报错
    let num: Number = integer.into();
    println!("{:?}", num);
}
