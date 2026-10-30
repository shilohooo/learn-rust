/// # 作用域规则 - 可变性
///
/// 当所有权转移时，数据的可变性可能发生改变。
fn main() {
    let immutable_box = Box::new(5u32);
    println!("immutable_box contains: {}", immutable_box);

    // immutable_box 是不可变的
    // immutable_box = 4;

    // 移动 box，改变所有权和可变性
    let mut  mutable_box = immutable_box;
    println!("mutable_box contains: {}", mutable_box);

    // 修改 box 的内容，要先用 * 符号解引用
    *mutable_box = 4;
    println!("mutable_box now contains: {}", mutable_box);
}
