/// 此函数取得堆分配的内存的所有权
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains: {}", c);
    // 方法执行完之后，c 被销毁且内存得到释放
}

/// # 作用域 - 所有权和移动
///
/// - ownership = 所有权
/// - move = 移动
///
/// 因为变量要负责释放它们拥有的资源，所以资源只能拥有一个所有者。这也防止了资源的重复释放。
///
/// 注意并非所有变量都拥有资源（例如引用）。
///
/// 在进行赋值（let x = y）或通过值来传递函数参数（foo(x)）的时候，
///
/// 资源的所有权（ownership）会发生转移。按照 Rust 的说法，这被称为资源的移动（move）。
///
/// 在移动资源之后，原来的所有者不能再被使用，这可避免悬挂指针（dangling pointer）的产生。
fn main() {
    // 在栈分配内存的整型
    let x = 5u32;
    // 复制 x 的值给 y，不会发生资源移动
    let y = x;

    // a 是一个指向堆分配的整数的指针
    let a = Box::new(5i32);
    println!("a contains: {}", a);

    // 移动 a 到 b
    // 把 a 的指针地址（不是实际的数据）复制到 b
    // 现在 a 和 b 都指向同一个堆分配的数据，但是现在是 b 拥有这个资源
    let b = a;

    // 下面这个语句会报错：Value used after being moved [E0382]
    // 因为 a 不再拥有在堆分配的内存了
    // println!("a contains: {}", a);

    // 下面这个函数会从 b 中取得堆分配的内存的所有权（ownership）
    destroy_box(b);

    // 下面这个语句会报错：Value used after being moved [E0382]
    // 因为在 destroy_box 函数执行结束后，b 指向的堆分配的内存已经被释放了
    // 这个操作会导致解引用已释放的内存，编译器禁止这种操作
    // println!("b contains: {}", b);
}
