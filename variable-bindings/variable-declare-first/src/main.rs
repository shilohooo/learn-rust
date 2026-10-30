/// 变量先声明再赋值
fn main() {
    let a_var;

    {
        let x = 2;

        // 初始化变量
        a_var = x * x;
    }

    println!("{}", a_var);

    let another_var;
    // 下面的打印语句将会报错，因为变量必须先初始化再使用
    // println!("{:?}", another_var);

    another_var = 1;
    println!("{}", another_var);
}
