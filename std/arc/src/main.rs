use std::sync::Arc;
use std::thread;

/// # 共享引用计数 Arc
///
/// 当线程之间所有权需要共享时，可以使用`Arc`（共享引用计数，Atomic Reference Counted 缩写）。
/// 这个结构通过 `Clone` 实现可以为内存堆中的值的位置创建一个引用指针，同时增加引用计数器。
/// 由于它在线程之间共享所有权，因此当指向某个值的最后一个引用指针退出作用域时，该变量将被删除。
fn main() {
    let arc = Arc::new("the same apple");
    for _ in 0..10 {
        let apple = Arc::clone(&arc);
        thread::spawn(move || {
            // 由于使用了 Arc，线程可以使用分配在 Arc 变量指针位置的值来生成
            println!("{:?}", apple);
        });
    }
}
