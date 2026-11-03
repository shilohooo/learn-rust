/// 第一行是对函数的简短描述
///
/// 接下来数行是详细文档。代码块使用三个反引号 ``` 开启，Rust 会隐式地在其中添加：
///
/// `fn main()` 和 `extern crate <cratename>`。比如测试 `doccomments` crate:
///
/// ```rust
/// let result = doc_testing::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 文档注释通常可能带有以下几个部分，它们都用标题 1 来展示：
///
/// - Examples - 使用示例
/// - Panics - 可能会引发的异常
/// - Failures - 可能会失败的情况
///
/// # Examples
///
/// ```rust
/// let result = doc_testing::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// 如果第二个参数为 0，函数将会 panic
///
/// ```rust,should_panic
/// doc_testing::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide by zero!");
    }

    a / b
}

/// # 文档测试的目的
///
/// 文档测试的主要目的是作为使用函数功能的例子，这是最重要的指导原则之一。
/// 文档测试应当可以作为完整的代码段被直接使用（很多用户会复制文档中的代码来用，所以例子要写得完善）。
/// 但使用 ? 符号会导致编译失败，因为 main 函数会返回 unit 类型。
/// 幸运的是，我们可以在文档中隐藏几行源代码：
/// 你可以写 `fn try_main() -> Result<(), ErrorType>` 这样的函数，把它隐藏起来，
/// 然后在隐藏的 main 中展开它。
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Divide by zero!"));
    }

    Ok(a / b)
}
