/// 卫语句
fn main() {
    let pair = (2, -2);
    // 在模式匹配里面可以加上卫语句进行过滤
    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}
