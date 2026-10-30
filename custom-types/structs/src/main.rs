/// 创建一个 C 语言风格的结构体
#[derive(Debug)]
struct Person {
    // 结构体可以有多个类型不同的字段
    /// 姓名
    name: String,
    /// 年龄
    age: u8,
}

/// 单元结构体
struct Unit;

/// 元组结构体
struct Pair(i32, f32);

/// 带有两个字段的结构体
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

/// 结构体还可以作为另一个结构体的字段
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

/// 计算长方形的面积，公式：长 * 宽
///
/// 宽 = 右下方 x - 左上方 x
///
/// 长 = 右下方 y - 左上方 y
fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x, y },
        bottom_right: Point { x: x2, y: y2 },
    } = rectangle;
    let width = x2 - x;
    println!("width: {}", width);
    let height = y2 - y;
    println!("height: {}", height);
    width * height
}

/// 根据给定的点位创建长方形
///
/// # Arguments
/// - top_left 左上角点位
/// - f 长
fn square(top_left: Point, f: f32) -> Rectangle {
    let Point { x: tfx, y: tfy } = top_left;
    Rectangle {
        top_left: Point { x: tfx, y: tfy },
        bottom_right: Point {
            x: tfx + f,
            y: tfy + f,
        },
    }
}

/// Rust 自定义类型 - 结构体：struct
///
/// 结构体有3种类型，使用 struct 关键字来创建：
///
/// - 元组结构体（tuple struct），即：命名元组
/// - 经典度 C 语言风格结构体（C struct）
/// - 单元结构体（unit struct），不带字段，通常在泛型中用
fn main() {
    // 使用简单的写法初始化字段，并创建结构体
    let name = String::from("Shiloh");
    let age = 27;
    let person = Person { name, age };
    // 以 debug 的方式打印结构体
    println!("{:?}", person);

    // 实例化结构体
    let point = Point { x: 5.2, y: 1.2 };
    // 访问结构体的字段
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 使用结构体更新语法创建新的 Point 结构体，可以用到之前创建的 point 的数据
    // x 字段指定了值，y 字段的值则用上面的 point 结构体的
    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // 使用 let 绑定来解构 point 结构体
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // 变量名称和字段名称一样，可以省略，参考 js
        bottom_right,
        // 结构体的实例化也是一个表达式，可以用来赋值～
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
    };

    // 实例化一个单元结构体
    let unit = Unit;
    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);
    // 通过下标来访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    println!(
        "rectangle area: {}",
        rect_area(Rectangle {
            top_left: Point { x: 2.1, y: 1.2 },
            bottom_right: Point { x: 11.1, y: 12.2 },
        })
    );

    println!("square: {:?}", square(point, 5.4));
}
