/// Rust 变量作用域与遮蔽
fn main() {
    // 变量绑定有一个作用域，它被限定只在一个代码块中生存。
    // 代码块是一个被大括号 {} 包裹的语句集合。
    // 下面这个变量的作用域是整个 main 函数
    let long_lived_binding = 1;

    // 下面是一个代码块，它的作用域比 main 函数小
    {
        // 这个变量绑定只存在于当前代码块
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
        // 这个变量绑定遮蔽（覆盖）了外面的绑定
        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }
    // 代码块结束了

    // 下面这个打印语句会报错，因为在此作用域上不存在 short_lived_binding 变量
    // println!("outer short: {}", short_lived_binding);

    // 回到 main 函数作用域，long_lived_binding 变量的值为 1
    println!("outer long: {}", long_lived_binding);

    // 这里的绑定同样产生了变量遮蔽
    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);
}
