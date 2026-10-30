use std::rc::Rc;

/// # 引用计数 `Rc`
///
/// 当需要多个所有权时，可以使用 `Rc`（引用计数，Reference Counting 缩写）。`Rc` 跟踪引用的数量，这相当于包裹在 `Rc` 值的所有者的数量.
///
/// 每当克隆一个 `Rc` 时，`Rc` 的引用计数就会增加 1，而每当克隆得到的 `Rc` 退出作用域时，引用计数就会减少 1。当 `Rc` 的引用计数变为 0 时，这意味着已经没有所有者，`Rc` 和值两者都将被删除。
///
/// 克隆 `Rc` 从不执行深拷贝。克隆只创建另一个指向包裹值的指针，并增加计数。
fn main() {
    let rc_examples = "Rc examples".to_string();
    {
        println!("rc_a is created");
        let rc_a = Rc::new(rc_examples);
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        {
            println!("rc_a is cloned to rc_b");
            let rc_b = Rc::clone(&rc_a);
            println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

            // 如果两个 rc 内部的值是相等的，则两个 rc 也是相等的
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

            // 可以直接通过 rc 调用值的方法
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);

            println!("rc_b is dropped out of scope")
        }

        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
    }

    // 走到这里，rc_examples 已经被销毁了
    // 因为前面 rc_examples 已经移入 rc_a ，然后 rc_a 出了作用域之后就被销毁了
    // 连带着 rc_examples 也被销毁
    // println!("{}", rc_examples);
}
