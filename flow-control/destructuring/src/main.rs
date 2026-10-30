#![allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

struct Foo {
    x: (u32, u32),
    y: u32,
}

/// `match` 中的解构
fn main() {
    println!("============================== 解构元组 ==============================");

    let triple = (0, -2, 3);
    println!("Tell me about {:?}", triple);

    match triple {
        // 解构元组的第二个和第三个元素
        (0, y, z) => println!("First is `0`, `y` is {:?} and `z` is {:?}", y, z),
        // `..` 可以用力啊忽略元组的其余部分
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        // _ 表示不将值绑定到变量
        _ => println!("It doesn't matter what that are"),
    }

    println!("============================== 解构枚举 ==============================");

    let color = Color::RGB(122, 17, 40);
    println!("What color is it?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Green => println!("The color is Green!"),
        Color::Blue => println!("The color is Blue!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
        // 不需要其它分支，因为所有的情形都已覆盖
        // _ => {}
    }

    println!("============================== 解构指针和引用 ==============================");

    // 指针和引用
    // 对指针来说，解构（destructure）和解引用（dereference）要区分开，因为这两者的概念是不同的，
    // 和 C 那样的语言用法不一样。
    // 解引用使用 `*` 符号
    // 解构使用 `&`、`ref`、和 `ref mut`
    // 获得一个 `i32` 类型的引用。`&` 符号表示取引用。
    let reference = &4;
    match reference {
        // 如果用 `&val` 这个模式去匹配 `reference`，就相当于做这样的比较：
        // `&i32`（译注：即 `reference` 的类型）
        // `&val`（译注：即用于匹配的模式）
        // ^ 我们看到，如果去掉匹配的 `&`，`i32` 应当赋给 `val`。
        // 因此可用 `val` 表示被 `reference` 引用的值 4。
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // 如果不想用 `&` 符号，需要在匹配前解引用
    match *reference {
        val => println!("Got a value via destructuring: {:?}", val),
    }

    // 变量不是引用类型的 match
    let _num_val = 3;
    // 可以使用 `ref` 关键字对具体的值创建引用
    let ref _num_ref = 3;

    // 还可以在使用到变量的地方，通过 `ref` 或者 `ref mut` 取引用
    let value = 5;
    match value {
        // println、函数参数、方法调用会自动解引用
        ref r => println!("Got a reference to a value: {:?}", r),
        // 匹配成功，处理具体逻辑的时候，可以使用 `*` 符号手动解引用来打印实际的值
        // ref r => println!("Got a reference to a value: {:?}", *r),
    }
    // 现代 Rust 更推荐在 match 之前就取引用
    match &value {
        v => println!("Got a reference to a value: {:?}", v),
    }
    let mut mutable_value = 6;
    // 可变引用
    match &mut mutable_value {
        m => {
            // 这里已经获得了引用，但是如果要修改变量的值，得先解引用
            *m += 10;
            println!("We added 10. `mutable value`: {:?}", m);
        }
    }

    println!("============================== 解构结构体的成员 ==============================");
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y: c } = foo;

    println!("a = {}, b = {}, c = {}", a, b, c);

    // 可以解构结构体并重命名变量，成员顺序并不重要
    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // 还可以忽略某些变量
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // 但是不能直接忽略 x 字段
    // let Foo { y,  } = foo;
}
