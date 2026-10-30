use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

static NUM_OF_THREADS: i32 = 3;

/// # Channel（通道）
///
/// Rust 为线程之间的通信提供了异步的通道（channel）。
/// 通道允许两个端点之间信息的单向流动：Sender（发送端） 和 Receiver（接收端）。
fn main() {
    // 通道有两个端点：`Sender<T>` 和 `Receiver<T>`，其中 `T` 是要发送的消息的类型
    let (sender, receiver): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    for id in 0..NUM_OF_THREADS {
        // sender 可以被复制
        let thread_sender = sender.clone();
        thread::spawn(move || {
            // 发送数据
            thread_sender.send(id).unwrap();
            println!("thread {} finished", id);
        });
    }
    let mut ids = Vec::with_capacity(NUM_OF_THREADS as usize);
    for _ in 0..NUM_OF_THREADS {
        // recv 方法从通道中拿到一个消息
        // 若无可用消息的话，`recv` 将阻止当前线程
        ids.push(receiver.recv());
    }

    println!("{:?}", ids);
}
