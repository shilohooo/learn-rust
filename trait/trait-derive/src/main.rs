/// 这是一个可以进行比较和排序的元组结构体
#[derive(PartialOrd, PartialEq)]
struct Centimeters(f64);

/// 这是一个可以通过 Debug 格式化打印的元组结构体
///
/// ```
/// let struct_var = Inches(233)
/// println!("{:?}", struct_var)
/// ```
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    /// 英尺转为厘米
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

struct Seconds(i32);

/// # trait - 派生
///
/// 通过 #[derive] 属性，编译器能够提供某些 trait 的基本实现。
///
/// 如果需要更复杂的行为，这些 trait 也可以手动实现。
///
/// 下面是可以自动派生的 trait：
///
/// - 比较 trait: Eq, PartialEq, Ord, PartialOrd
/// - Clone, 用来从 &T 创建副本 T。
/// - Copy，使类型具有 “复制语义”（copy semantics）而非 “移动语义”（move semantics）。
/// - Hash，从 &T 计算哈希值（hash）。
/// - Default, 创建数据类型的一个空实例。
/// - Debug，使用 {:?} formatter 来格式化一个值。
fn main() {
    let one_second = Seconds(1);

    // 下面这行代码回报错：`Seconds` does not implement `Debug` (required by `{:?}`): E0277
    // println!("{:?}", one_second);

    let foot = Inches(12);
    println!("One foot equals: {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };

    println!("One foot is {} than one meter.", cmp);
}
