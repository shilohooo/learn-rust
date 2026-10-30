/// vector 是大小可变的数组。和 slice（切片）类似，它们的大小在编译时是未知的，
/// 但它们可以随时扩大或缩小。一个 vector 使用 3 个词来表示：
///
/// - 一个指向数据的指针
/// - vector 的长度
/// - 它的容量。此容量指明了要为这个 vector 保留多少内存。vector 的长度只要小于该容量，
/// 就可以随意增长；当需要超过这个阈值时，会给 vector 重新分配一段更大的容量。
fn main() {
    // 迭代器可以被收集到 vector 中
    let numbers: Vec<i32> = (0..10).collect();
    println!("Collected: (0..10) into {:?}", numbers);
    // vec! 宏可以用来初始化一个 vector
    let mut numbers2 = vec![1i32, 2, 3, 4, 5, 6];
    println!("Initial vector: {:?}", numbers2);

    // 在 vector 的尾部插入一个元素，注意，必须是可变的 vector 才可以增长
    println!("Push 10 into the numbers2 vector");
    numbers2.push(10);
    println!("Vector: {:?}", numbers2);
    // 下面这行会报错：Cannot borrow immutable local variable `numbers` as mutable
    // numbers.push(0);

    // 获取 vector 当前大小
    println!("Vector size: {}", numbers2.len());

    // 使用下标访问 vector 中的元素
    println!("Second element: {}", numbers2[1]);

    // 移除 vector 最后一个元素并返回
    println!("Pop last element: {:?}", numbers2.pop());

    // 下标越界会导致 panic：index out of bounds
    // println!("Eighth element: {}", numbers2[7]);

    // 迭代 vector
    for num in numbers2.iter() {
        println!("> {}", num);
    }

    // 带下标迭代
    for (i, num) in numbers2.iter().enumerate() {
        println!("In positon {} we have value {}", i, num);
    }

    // 在迭代中修改 vector 里面的元素
    for num in numbers2.iter_mut() {
        *num *= 3;
    }
    println!("Updated vector: {:?}", numbers2);
}
