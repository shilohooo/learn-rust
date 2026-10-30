struct Fibonacci {
    current: u32,
    next: u32,
}

/// 为 `Fibonacci`（斐波那契）实现 `Iterator`。
///
/// `Iterator` trait 只需定义一个能返回 `next`（下一个）元素的方法。
impl Iterator for Fibonacci {
    type Item = u32;

    /// 我们在这里使用 `.curr` 和 `.next` 来定义数列（sequence）。
    ///
    /// 返回类型为 `Option<T>`：
    ///
    ///     * 当 `Iterator` 结束时，返回 `None`。
    ///     * 其他情况，返回被 `Some` 包裹（wrap）的下一个值。
    fn next(&mut self) -> Option<Self::Item> {
        let nex_next = self.current + self.next;

        self.current = self.next;
        self.next = nex_next;

        // 既然斐波那契数列不存在终点，那么 `Iterator` 将不可能
        // 返回 `None`，而总是返回 `Some`。
        Some(self.current)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci {
        current: 1,
        next: 1,
    }
}

/// # trait - Iterator - 迭代器
///
/// Iterator trait 用来对集合（collection）类型（比如数组）实现迭代器。
///
/// 这个 trait 只需定义一个返回 next（下一个）元素的方法，这可手动在 impl 代码块中定义，
///
/// 或者自动定义（比如在数组或区间中）。
///
/// 为方便起见，for 结构会使用 .into_iter() 方法将一些集合类型转换为迭代器。
fn main() {
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // `for` 遍历 `Iterator` 直到返回 `None`，
    // 并且每个 `Some` 值都被解包（unwrap），然后绑定给一个变量（这里是 `i`）。
    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // `take(n)` 方法提取 `Iterator` 的前 `n` 项。
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // `skip(n)` 方法移除前 `n` 项，从而缩短了 `Iterator` 。
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let numbers = [1u32, 3, 3, 7];
    // `iter` 方法对数组/slice 产生一个 `Iterator`。
    println!("Iterate the following array {:?}", &numbers);
    for i in numbers.iter() {
        println!("> {}", i);
    }
}
