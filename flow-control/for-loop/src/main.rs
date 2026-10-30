/// `for` 循环
///
/// `for in` 结果可以遍历一个 `Iterator`（迭代器）
///
/// 创建迭代器的一个最简单的方法是使用区间标记：`a..b`
///
/// 这会生成从 `a`（包含此起始值）到 `b` （不包含此结束值）的，步长为1的一系列值
fn main() {
    // `n` 将在每次迭代中分别取1, 2 , 3, 4, 5 ..., 100
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // 可以使用a..=b表示两端都包含在内的范围。上面的代码可以写成：
    // `n` 将在每次迭代中分别取1, 2 , 3, 4, 5 ..., 100
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // iter 迭代器 - 每次迭代都是借用集合中的一个元素，不会改变集合本身
    // 循环结束之后，集合中的元素还可以使用
    let names = vec!["Shiloh", "Bruce", "Tom", "Rose"];

    for name in names.iter() {
        match name {
            &"Shiloh" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("===================================");

    // into_iter - 会消耗集合。在每次迭代中，集合中的数据本身会被提供。
    // 一旦集合被消耗了，之后就无法再使用了，因为它已经在循环中被 “移除”（move）了。
    for name in names.into_iter() {
        match name {
            "Shiloh" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    let mut names = vec!["Shiloh", "Bruce", "Tom", "Rose"];
    // iter_mut - 可变地（mutably）借用集合中的每个元素，从而允许集合被就地修改。
    for name in names.iter_mut() {
        *name = match name {
            &mut "Shiloh" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
}
