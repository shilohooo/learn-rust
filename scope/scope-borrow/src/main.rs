/// 这个函数会取得一个 box 的所有权并销毁它
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains: {}", boxed_i32)
}

/// 这个函数借用了一个 i32 类型的数据
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

/// # 作用域规则 - 借用
///
/// 多数情况下，我们更希望能访问数据，同时不取得其所有权。为实现这点，Rust 使用了借用（borrowing）机制。
///
/// 对象可以通过引用（&T）来传递，从而取代通过值（T）来传递。
///
/// 编译器（通过借用检查）静态地保证了引用总是指向有效的对象。也就是说，当存在引用指向一个对象时，该对象不能被销毁。
fn main() {
    // boxed_i32 在堆上分配内存
    let boxed_i32 = Box::new(5_i32);
    // stack_i32 在栈上分配内存
    let stack_i32 = 6_i32;

    // 这里借用了 box 的内容，但没有取得所有权，所以 box 的内容在之后可以再次借用
    // 函数本身也是一个作用域，当函数执行结束，在函数中临时创建的引用也就不复存在了
    borrow_i32(&boxed_i32);
    borrow_i32(&stack_i32);

    {
        let ref_to_i32: &i32 = &boxed_i32;

        // 下面的 borrow_i32 方法借用了 boxed_i32
        // 当 boxed_i32 里面的值在作用域中被借用时，它不能被销毁
        // 这里会报错：Cannot move out of `boxed_i32` because it is borrowed
        // eat_box_i32(boxed_i32);

        // 借用 ref_to_i32
        borrow_i32(ref_to_i32);
    }

    // 这里可以移动 boxed_i32，把所有权交给 eat_box_i32 并销毁
    // 因为在这里已经不存在对 boxed_i32 的引用了
    eat_box_i32(boxed_i32);
}
