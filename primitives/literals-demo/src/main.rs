/// 字面量和运算符
fn main() {
    // 通过添加前缀 `0x`、`0o`、`0b`，数字可以用十六进制、八进制、二进制法表示
    // Rust 的运算符优先级和类 C 语言类型，如：Java、PHP 等

    // 整数相加
    println!("1 + 2 = {}", 1 + 2);

    // 整数相减，如果把 1 标注为无符号整数，会导致算术溢出，无符号整数不允许负数
    // println!("1 - 2 = {}", 1u32 - 2);
    println!("1 - 2 = {}", 1 - 2);

    // 布尔运算
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // 位运算
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // 使用下划线改善大数字字面量的可读性
    println!("One million is written as {}", 1_000_000u32);

}
