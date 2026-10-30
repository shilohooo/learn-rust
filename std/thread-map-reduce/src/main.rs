use std::thread;

static NUM_OF_THREADS: usize = 2;

/// # 测试实例：map-reduce
///
/// Rust 使数据的并行化处理非常简单，在 Rust 中你无需面对并行处理的很多传统难题。
///
/// 标准库提供了开箱即用的线程类型，把它和 Rust 的所有权概念与别名规则结合起来，可以自动地避免数据竞争（data race）。
///
/// 当某状态对某线程是可见的，别名规则（即一个可变引用 XOR 一些只读引用。译注：XOR
/// 是异或的意思，即「二者仅居其一」）就自动地避免了别的线程对它的操作。（当需要同步处理时，请使用 `Mutex` 或 `Channel` 这样的同步类型。）
///
/// 在本例中，我们将会计算一堆数字中每一位的和。我们将把它们分成几块，放入不同的线程。每个线程会把自己那一块数字的每一位加起来，之后我们再把每个线程提供的结果再加起来。
///
/// 注意到，虽然我们在线程之间传递了引用，但 Rust 理解我们是在传递只读的引用，因此不会发生数据竞争等不安全的事情。另外，因为我们把数据块 `move` 到了线程中，Rust
/// 会保证数据存活至线程退出，因此不会产生悬挂指针。
fn main() {
    // 这是我们要处理的数据。
    // 我们会通过线程实现 map-reduce 算法，从而计算每一位的和
    // 每个用空白符隔开的块都会分配给单独的线程来处理
    //
    // 试一试：插入空格，看看输出会怎样变化！
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701608519 07960690014725639
38397966707106094172783238747669219
5238079525788823652 5459303330302837
58495327135744041 048897885734297812
69920216438980873548 808413720956532
1627842463745 2589860345374828574668";

    // 先去除空白字符
    let digits: String = data.chars().filter(|c| !c.is_whitespace()).collect();
    // 计算每一段的长度
    let len = digits.len();
    println!("digits total length: {}", len);
    let chunk_size = (len + NUM_OF_THREADS - 1) / NUM_OF_THREADS;
    println!("Chunk size: {}", chunk_size);

    // 创建一个向量，用于存储将要创建的子线程
    let mut childrens = vec![];

    // 把数据分段处理
    for i in 0..NUM_OF_THREADS {
        let start = i * chunk_size;
        if start >= len {
            break;
        }

        let end = (start + chunk_size).min(len);
        let segment = digits[start..end].to_string();

        childrens.push(thread::spawn(move || -> u32 {
            // 对该分段中字符进行迭代
            let sum = segment
                .chars()
                // 转为数字
                .map(|c| c.to_digit(10).expect("should be a digit"))
                // 求和
                .sum();
            // println! 会锁住标准输出，这样各线程打印的内容不会交错在一起
            println!(
                "[{:?}] - processed segment: {}, result: {}",
                thread::current().id(),
                segment,
                sum
            );

            sum
        }));
    }

    // 收集中间结果得出最终结果
    let mut intermediate_sums = vec![];
    for child in childrens {
        intermediate_sums.push(child.join().unwrap())
    }

    // 把所有结果加起来
    let final_result = intermediate_sums.iter().sum::<u32>();
    println!("Final sum result: {}", final_result);
}
