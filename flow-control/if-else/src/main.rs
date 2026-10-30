/// 流程控制之 if else
/// `if-else` 条件选择是一个表达式，并且所有分支都必须返回相同的类型
fn main() {
    let num = 5;
    if num < 0 {
        print!("{} is negative", num);
    } else if num > 0 {
        print!("{} is positive", num);
    } else {
        print!("{} is zero", num);
    }

    // 表达式赋值
    let big_num = if num < 10 && num > -10 {
        println!(", and is a small number, increase ten-fold");
        num * 10
    } else {
        println!(", and is a big number, half the number");
        num / 2
    };

    println!("{} -> {}", num, big_num);
}
