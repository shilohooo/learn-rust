struct Point {
    x: f64,
    y: f64,
}

/// 为 `Point` 结构体实现的代码块，`Point` 的所有方法都在这里定义
impl Point {
    /// 定义一个静态方法，不需要实例化就能调用
    ///
    /// 静态方法常用作构造器（constructor）
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    /// 实例方法：获取矩形的面积，`&self` 是 `self: &Self` 的语法糖，其中 `Self` 是方法调用者的类型
    ///
    /// 在这个例子中，`Self` 的类型为 `Rectangle`
    ///
    /// 可以把 &self 想象成其他面向对象语言的 this
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs()` 是一个返回值为 `f64` 类型的方法，返回调用者的绝对值
        ((x1 - x2) * (y1 - y2)).abs()
    }

    /// 实例方法：获取矩形的周长
    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2) + (y1 - y2)).abs()
    }

    /// 这个实例方法要求调用者是可变的
    /// `&mut self` 是 `self: &mut Self` 的语法糖
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

/// `Pair` 结构体拥有两个在堆分配的整型参数
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    /// 这个方法会“消耗”调用者的资源，就是离开这个函数作用域之后，就不能再使用了
    /// `self` 是 `self: Self` 的语法糖
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pari({}, {})", first, second);

        // `first` 和 `second` 离开作用域后释放
    }
}

/// 方法（method）是依附于对象的函数。
///
/// 这些方法通过关键字 self 来访问对象中的数据和其他成员。
///
/// 方法在 `impl` 代码块中定义
fn main() {
    let rectangle = Rectangle {
        // 静态方法可以直接通过 类型::方法() 调用
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // 实例方法通过 实例变量.方法() 调用
    // 注意第一个参数：self / &self / &mut self 是隐式传递的
    // rectangle.perimeter() = rectangle.perimeter(&rectangle)
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    // 下面的调用会报错，因为 rectangle 是不可变的
    // rectangle.translate(1.0, 1.0);

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // 这里可以正常运行，因为 square 声明为 mut
    // 可变对象调用可变方法
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

    // 再次调用 destroy 方法将会报错，因为前面的调用“消耗”了 `pair`
    // pair.destroy();
}
