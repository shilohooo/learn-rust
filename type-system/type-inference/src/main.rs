/// 类型推导 - 大部份情况都不需要显式地去声明类型～
fn main() {
    // 显式的声明了类型，编译器无需推导
    let element = 5u8;
    // 目前，编译器还不知道 vec 的具体类型是啥，只能推导为 Vec<?>
    // 因为还没有插入元素
    let mut vector = Vec::new();
    // 当插入一个元素之后，编译器就知道 vec 的类型是 Vec<u8> 了
    vector.push(element);
    println!("{:?}", vector);
}
