fn give_commoner(gift: Option<&str>) {
    // 模式匹配
    match gift {
        Some("snake") => println!("Yuck! I'm throwing that snake in a fire."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No gift? Oh well."),
    }
}

fn give_princess(gift: Option<&str>) {
    // unwrap 隐式处理要么返回 Some 要么 panic：called `Option::unwrap()` on a `None` value
    let inside = gift.unwrap();
    if inside == "snake" {
        panic!("AAAaaaaaaa!!!")
    }

    println!("I love {}s", inside);
}

/// Rust Option 和 unwrap
fn main() {
    let food = Some("chicken");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_princess(bird);
    give_princess(nothing);
}
