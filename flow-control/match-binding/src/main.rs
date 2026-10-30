fn age() -> u32 {
    3
}

fn some_num() -> Option<u32> {
    Some(42)
}

/// match 中的变量绑定
///
/// 在 `match` 中，若间接地访问一个变量，则不经过重新绑定就无法在分支中再使用它。
///
/// `match` 提供了 `@` 符号来绑定变量到名称
fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // 注意，这里把 1..=12 这个范围分支的匹配值绑定到了变量 N，后续就可以读取年龄了
        // 如果只是匹配 1..=12 => { ... }, 就无法读取到实际匹配到年龄
        n @ 1..=12 => println!("I' m a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }

    // 使用绑定来解构枚举变体
    match some_num() {
        Some(n @ 42) => println!("The answer: {}!", n),
        Some(n) => println!("Not interesting... {}",n),
        _ => (),
    }
}
