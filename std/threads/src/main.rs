use std::thread;

static NUM_OF_THREADS: i32 = 10;

/// # 线程
///
/// Rust 通过 `spawn` 函数提供了创建本地操作系统（native OS）线程的机制，
/// 该函数的参数是一个通过值捕获变量的闭包（moving closure）。
fn main() {
    let mut children = vec![];

    for i in 0..NUM_OF_THREADS {
        // 启动另外一个线程，这些线程由操作系统调度（schedule）。
        children.push(thread::spawn(move || {
            let current_thread = thread::current();
            println!(
                "this is thread number: {}, thread name: {:?}, thread id: {:?}",
                i,
                // 通过 thread::spawn 创建的线程是没有名称的，这里会固定输出 None
                current_thread.name(),
                current_thread.id()
            );
        }));
    }

    for child in children {
        // 等待线程执行结束，返回一个结果
        let _ = child.join();
    }
}
