fn main() {
    // 单行注释
    /// 文档注释
    /* 块注释，而且可以填套 /*123123*/  */
    println!("Hello World!");
    println!("I'm a Rustacean!");
    let x = 5 + 90 + 5;
    println!("Is `x` 10 or 100? x = {}", x);

    // 31 默认为 i32 类型，添加后缀可以改为它的类型
    println!("{} days", 31i64);

    // region 变量替换字符串占位符

    // 按位置顺序替换
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 使用命名参数
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // 特殊格式 {:b} 代表值的二进制格式
    println!("{} of {:b} people know binary, the other haf don't", 1, 2);

    // 指定宽度对齐文本，下面的语句输出"     1"，五个空格后面跟着数字1，总宽度为6
    println!("{number:>width$}", number = 1, width = 6);

    // 在数字左边补0，下面的语句输入"000001"，五个零后面跟着数字1，总宽度为6
    println!("{number:>0width$}", number = 1, width = 6);

    // println! 会自动检查参数数量是否正确
    println!("My name is {0}, {1}, {0}", "James", "Bond");

    // 创建结构体
    #[allow(dead_code)]
    struct Structure(i32);

    // 输出结构体 - 无法输出，自定义的类型要输出的话，需要更复杂的方式去处理
    // println!("This struct `{}` won't print", Structure(3));

    // endregion

    let pi = 3.1415926;
    // {命名参数:.N} = 精度输出，这里指定了精度为小数点后三位，而且会自动四舍五入
    println!("Pi is roughly {pi:.3}", pi = pi);
}
