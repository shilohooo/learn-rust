/// 字面量
fn main() {
    // 数值字面量：可以直接在数值后面添加类型后缀
    // 下面这个变量绑定的类型就是 i32，有符号32位整型
    let integer = 42i32;
    println!("{}", integer);
    // 没有后缀的数值字面量，整型默认是 i32，浮点数默认是 f64
    let another_integer = 1;
    println!("{}", another_integer);
    let float_number = 3.14;
    println!("{}", float_number);

    // 获取一个变量所占用的内存字节数
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;
    // &符号可以用来获取变量的引用，而不是实际的值
    println!("size of integer in bytes = {}", size_of_val(&integer));
    println!("size of x in bytes = {}", size_of_val(&x));
    println!("size of y in bytes = {}", size_of_val(&y));
    println!("size of z in bytes = {}", size_of_val(&z));
    println!("size of float_number in bytes = {}", size_of_val(&float_number));
}
