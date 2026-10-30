type NanoSecond = u64;
type Inch = u64;

/// 加上这个属性可以消除命名不规范的警告
#[allow(non_camel_case_types)]
type u64_alias = u64;

/// 类型别名
///
/// 使用 `type` 语句给已有的类型取一个新的名字
///
/// 类型命名规范：大驼峰 - CamelCase，但不包括原生类型
///
/// 别名的主要用途是避免写出冗长的模板化代码（boilerplate code）。
///
/// 如 `IoResult<T>` 是 `Result<T, IoError>` 类型的别名。
fn main() {
    let nano_seconds: NanoSecond = 5 as u64_alias;
    let inches: Inch = 2 as u64_alias;
    // 需要注意的是：类型别名无法提供额外的类型安全，它只是一个名称
    println!(
        "{} nana seconds + {} inches = {} unit?",
        nano_seconds,
        inches,
        nano_seconds + inches
    );
}
