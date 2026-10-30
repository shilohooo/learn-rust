use std::fmt::{Display, Formatter};

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 解构元组内的元素
    let (i, b) = pair;
    println!("pair: ({}, {})", i, b);
    (b, i)
}

fn transpose(matrix: Matrix) -> Matrix {
    let (top_left, top_right, bottom_left, bottom_right) = matrix.0;
    Matrix((top_left, bottom_left, top_right, bottom_right))
}

// 结构体加元组
#[derive(Debug)]
struct Matrix((f32, f32, f32, f32));

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let tuple = self.0;
        write!(f, "({}, {})\n({}, {})", tuple.0, tuple.1, tuple.2, tuple.3)
    }
}

/// 元组：一个可以包含多个类型值的组合类型
fn main() {
    let (b, i) = reverse((1, false));
    println!("reverse result: ({}, {})", b, i);

    // 定义一个包含多种类型的元组
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, 5u128, 6usize, -1i8, -2i16, -3i32, -4i64, -5i128, -6isize, 0.1f32,
        0.2f64, 'a', true, "string",
    );
    // 使用下标访问元组中的元素
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // 元组可以嵌套
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, 5u128, 8usize), -1i8);
    // 可以直接打印元组
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // 但很长的元组就不能打印了（超过12个元素）
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
    // println!("too long tuple: {:?}", too_long_tuple);

    // 创建只有一个元素的元组时，结尾需要一个英文逗号来区分放在括号里面的字面量
    println!("one element tuple: {:?}", (233,));
    println!("just a integer: {}", (233));

    // 元组可以被解构
    let tuple = ("hello", 114514, true);
    let (a, b, c) = tuple;
    println!("{:?}, {:?}, {:?}", a, b, c);

    let matrix = Matrix((1.1, 1.2, 2.1, 2.2));
    println!("{:?}", matrix);
    println!("{}", matrix);

    println!("Matrix:\n{}", matrix);
    let transpose = transpose(matrix);
    println!("Transpose:\n{}", transpose);
}
