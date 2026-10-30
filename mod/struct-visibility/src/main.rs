mod my {
    /// 一个公有的结构体，带有一个公有的字段（类型为泛型 T）
    pub struct OpenBox<T> {
        pub contents: T,
    }

    /// 一个公有的结构体，带有一个私有的字段（类型为泛型 T）
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        /// 一个公有的构造器
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents }
        }

        /// 通过公有方法访问私有字段
        pub fn get_contents(&self) -> &T {
            &self.contents
        }
    }
}

/// 结构体的可见性
///
/// 结构体的字段也是一个可见性的层次。字段默认拥有私有的可见性，也可以加上 `pub` 修饰语来重载该行为。
///
/// 只有从结构体被定义的模块之外访问其字段时，这个可见性才会起作用，其意义是隐藏信息（即封装，`encapsulation`）。
fn main() {
    // 带有公有字段的公有结构体，实例化跟使用普通的结构体是一样的
    let open_box = my::OpenBox {
        contents: "public information",
    };
    println!("Then open box contains: {}", open_box.contents);

    // 带有私有字段的结构体不能直接用字段赋值的方式来实例化
    // 下面的语句会报错：Field `contents` in struct `my::ClosedBox` is private [E0451]
    // let closed_box = my::ClosedBox { contents: "classified information" };

    // 可以使用它的公有构造器来实例化
    let closed_box = my::ClosedBox::new("classified information");
    // 但是实例化之后也不能访问它的私有字段
    // 下面的语句会报错：// 下面的语句会报错：Field `contents` in struct `my::ClosedBox` is private [E0451]
    // println!("The closed box contains: {}", closed_box.contents);

    // 可以通过公有访问访问私有字段
    println!("The closed box contains: {}", closed_box.get_contents());
}
