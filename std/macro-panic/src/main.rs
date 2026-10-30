fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        panic!("division by zero")
    } else {
        dividend / divisor
    }
}

/// # panic!
/// `panic!` 宏可用于产生一个 panic （恐慌），并开始回退（unwind）它的栈。在回退栈的同时，运行时将会释放该线程所拥有的所有资源，这是通过调用线程中所有对象的析构函数完成的。
///
/// 因为我们正在处理的程序只有一个线程，`panic!` 将会引发程序报告 panic 消息并退出。
fn main() {
    let _x = Box::new(0i32);
    division(3, 0);
    println!("This point won't be reached!");
    // _x 变量应当会在此处被销毁
}
