use std::{fmt, vec};

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        // 迭代 vec，并记录迭代次数
        for (count, v) in vec.iter().enumerate() {
            // 对每个元素加上逗号（除第一个元素之外）
            if count != 0 {
                // 使用 try! 或者结尾的 ? 来返回错误
                write!(f, ", ")?;
            }
            write!(f, "{}:{}", count, v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let vec = List(vec![1, 2, 3, 4, 5, 6]);
    println!("{}", vec);
}
