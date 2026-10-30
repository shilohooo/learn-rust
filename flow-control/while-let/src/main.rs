/// while let
fn main() {
    // 用 while let 来简化 match
    let mut optional = Some(0);
    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Great than 9, quit");
                    optional = None;
                } else {
                    println!("`i` is `{:}`. Try again.", i);
                    optional = Some(i + 1);
                }
            }
            _ => break,
        }
    }

    // 只需要两层锁进即可
    // 当 let 将 optional 变量解构成 Some(i) 时，就执行循环语句块 {}，否则 break
    while let Some(i) = optional {
        if i > 9 {
            println!("Great than 9, quit");
            // 改为 None 之后就匹配不到 Some(i) 了，然后循环就退出了
            optional = None;
        } else {
            println!("`i` is `{:}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}
