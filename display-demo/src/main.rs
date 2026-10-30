// 导入模块
use std::fmt::{self};

// 定义结构体，实现 fmt::Display
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // self 指向结构体，self.0表示访问第一个字段
        write!(f, "{}", self.0)
    }
}

// 实现 Debug 的结构体，用来和实现了 fmt::Display 的结构体做比较
#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

// 定义一个有具名字段的结构体
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    let min_max = MinMax(0, 14);
    println!("比较结构体的输出格式：");
    // 这里输出的是：(0, 14)
    println!("Display: {}", min_max);
    // 这里输出的是：MinMax(0, 14)
    println!("Debug: {:?}", min_max);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!(
        "The big range is {big} and the small is {small}",
        big = big_range,
        small = small_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Point2D 结构体输出比较：");
    // 输出：x: 3.3, y: 7.2
    println!("Display: {}", point);
    // 输出：Point2D { x: 3.3, y: 7.2 }
    println!("Debug: {:?}", point);

    // 这里会报错，因为输出二进制实现需要实现 fmt::Binary
    // println!("What does Point2D look like in binary: {:b}?", point);

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Complex 结构体输出比较：");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}
