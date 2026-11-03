pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 用于测试断言失败的情况
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

pub fn divide_non_zero_result(a: i32, b: i32) -> i32 {
    if a == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}

/// # 单元测试
///
/// 测试（test）是这样一种 Rust 函数：它保证其他部分的代码按照所希望的行为正常运行。测试函数的函数体通常会进行一些配置，运行我们想要测试的代码，然后断言（assert）结果是不是我们所期望的。
///
/// 大多数单元测试都会被放到一个叫 `tests` 的、带有 `#[cfg(test)]` 属性的模块中，测试函数要加上 `#[test]` 属性。
///
/// 当测试函数中有什么东西 panic 了，测试就失败。有一些这方面的辅助宏：
///
/// - assert!(expression) - 如果表达式的值是 false 则 panic。
/// - assert_eq!(left, right) 和 assert_ne!(left, right) - 检验左右两边是否 相等/不等。
///
/// ## 测试 panic
///
/// 一些函数应当在特定条件下 panic。为测试这种行为，请使用 `#[should_panic]` 属性。
/// 这个属性接受可选参数 expected = 以指定 panic 时的消息。如果你的函数能以多种方式 panic，
/// 这个属性就保证了你在测试的确实是所指定的 panic。
#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    #[ignore] // 忽略测试
    fn test_bad_add() {
        assert_eq!(bad_add(1, 2), 3)
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(20, 10), 2);
    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "Divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }
}
