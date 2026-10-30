/// 流程控制之 `loop` 循环
///
/// loop 循环表示无限循环，除非手动调用 `break` 退出
///
/// 还可以使用 `continue` 跳过当前循环开始下一轮循环
fn main() {
    let mut  i = 0u32;
    println!("Let's count until infinity!");
    loop {
        i+=1;
        if i==3 {
            println!("three");
            // 跳过本次循环的剩下内容，开始下一轮循环
            continue;
        }
        println!("{}", i);

        if i==5 {
            println!("Ok, that's enough");

            // 退出循环
            break;
        }
    }
}
