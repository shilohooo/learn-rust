#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    Box::new(Point { x: 0.0, y: 0.0 })
}

/// 在 Rust 中，所有值默认都是栈分配的。通过创建 Box<T>，可以把值装箱（boxed）来使它在堆上分配。
/// 箱子（box，即 Box<T> 类型的实例）是一个智能指针，指向堆分配的 T 类型的值。
/// 当箱子离开作用域时，它的析构函数会被调用，内部的对象会被销毁，堆上分配的内存也会被释放。
///
/// 被装箱的值可以使用 * 运算符进行解引用；这会移除掉一层装箱。
fn main() {
    // 栈分配
    let point = origin();
    let rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 },
    };
    // 堆分配
    let boxed_rectangle = Box::new(Rectangle {
        p1: origin(),
        p2: origin(),
    });
    // 函数的返回值也可以装箱
    let boxed_point = Box::new(origin());
    // 两层装箱
    let box_in_a_box = Box::new(boxed_origin());

    println!("Point occupies {} bytes in the stack", size_of_val(&point));
    println!(
        "Rectangle occupies {} bytes in the stack",
        size_of_val(&rectangle)
    );

    // box 的宽度就是指针宽度
    println!(
        "Boxed point occupies {} bytes in the stack",
        size_of_val(&boxed_point)
    );
    println!(
        "Boxed rectangle occupies {} bytes in the stack",
        size_of_val(&boxed_rectangle)
    );
    println!(
        "Boxed box occupies {} bytes in the stack",
        size_of_val(&box_in_a_box)
    );

    // 将包含在 `boxed_point` 中的数据复制到 `unboxed_point`
    let unboxed_point = *boxed_point;
    println!(
        "Unboxed point occupies {} bytes in the stack",
        size_of_val(&unboxed_point)
    );
}
