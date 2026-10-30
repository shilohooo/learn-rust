enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

enum Something {
    Bar
}


/// `if let`
fn main() {
    // match 必须声明所有可能的匹配
    let optional = Some(7);
    match optional {
        Some(i) => println!("This is a really long string and {:?}", i),
        _ => {}
    }
    // 使用 if let 则会简洁很多
    let number = Some(7);
    let letter: Option<i32> = None;
    let emotion: Option<i32> = None;
    // if let 结构解析：若 let 将 number 解构成 Some(i)，则执行语句块 {}
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }
    // 如果要指明不匹配的情况，可以使用 else
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;
    if let Some(i) = emotion {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emotion :)");
    }

    // 使用 if let 匹配枚举
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // 变量 a 匹配到了 Foo::bar
    if let Foo::Bar = a {
        println!("a is foorbar");
    }

    // 变量 b 没有匹配到 Foo::Bar，因此不会执行 {} 里面的语句
    if let Foo::Bar = b {
        println!("b is foorbar");
    }

    // 变量 c 匹配的时候带有一个值
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // if let 可以匹配枚举非参数化的变量，即枚举未注明 #[derive(PartialEq)]，
    // 我们也没有为其实现 PartialEq。在这种情况下，通常 if Foo::Bar==a 会出错，
    // 因为此类枚举的实例不具有可比性。但是，if let 是可行的。
    let something = Something::Bar;
    // 下面这里会报错：Binary operation `==` cannot be applied to type `Something` [E0369]
    // if Something::Bar == something{
    //     println!("something is foobar");
    // }
    // 但使用 if let 就可以完成匹配
    if let Something::Bar = something {
        println!("something is foobar");
    }
}
