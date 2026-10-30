use std::fmt::{Debug, Display};

/// 限制泛型 T 必须实现 Debug 和 Display 两个 trait
fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: {:?}", t);
    println!("Display: {}", t);
}

fn compare_types<T:Debug,U:Debug>(t: &T, u: &U) {
    println!("t: {:?}", t);
    println!("u: {:?}", u);
}

/// # 多重约束
///
/// 多重约束（multiple bounds）可以用 + 连接。和平常一样，类型之间使用 , 隔开。
fn main() {
    let str = "words";
    let numbers = [1, 2, 3];
    let vec = vec![1, 2, 3];
    compare_prints(&str);
    // 下面这个语句会报错：Trait `Display` is not implemented for `[i32; 3]` [E0277]
    // compare_prints(&numbers);

    compare_types(&numbers, &vec);
}
