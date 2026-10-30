fn create_box() {
    let _box1 = Box::new(3i32);
    // 方法执行完之后，_box1 会被销毁，内存得到释放～
}

struct ToDrop;

impl Drop for ToDrop {
    /// 结构体被销毁的时候会自动调用这个方法
    fn drop(&mut self) {
        println!("ToDrop is being dropped")
    }
}

/// # 作用域（Scope）
///
/// 作用域在所有权（ownership）、借用（borrow）和生命周期（lifetime）中起着重要作用。
///
/// 也就是说，作用域告诉编译器什么时候借用是合法的、什么时候资源可以释放、
///
/// 以及变量何时被创建或销毁。
///
/// ## RAII
///
/// Rust 的变量不只是在栈中保存数据：它们也占有资源，比如 Box<T> 占有堆（heap）中的内存。
///
/// Rust 强制实行 [RAII](https://en.wikipedia.org/wiki/Resource_Acquisition_Is_Initialization)（Resource Acquisition Is Initialization，资源获取即初始化），
///
/// 所以任何对象在离开作用域时，它的析构函数（destructor）就被调用，
///
/// 然后它占有的资源就被释放。
///
/// 这种行为避免了资源泄漏（resource leak），
///
/// 所以你再也不用手动释放内存或者担心内存泄漏（memory leak）！
///
/// ## 析构函数
///
/// Rust 中的析构函数概念是通过 Drop trait 提供的。当资源离开作用域，就调用析构函数。
///
/// 你无需为每种类型都实现 Drop trait，只要为那些需要自己的析构函数逻辑的类型实现就可以了。
fn main() {
    let _box2 = Box::new(5i32);

    {
        let _box3 = Box::new(4i32);
        // 离开嵌套代码块之后，_box3 被销毁，内存得到释放～
    }

    // 循环调用方法创建在堆内存分配的变量
    for _ in 0u32..1_000 {
        // 不需要手动释放内存～
        create_box()
    }

    let to_drop = ToDrop;
    println!("Made a ToDrop");

    // main 方法执行结束后，_box2 被销毁，内存得到释放～
}

