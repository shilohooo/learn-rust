#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        // 只接受偶数
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

/// `TryFrom` 和 `TryInto`
///
/// 类似于 `From` 和 `Into`，`TryFrom` 和 `TryInto` 是类型转换的通用 `trait`。
///
/// 不同于 `From`/`Into` 的是，`TryFrom` 和 `TryInto trait` 用于易出错的转换，
///
/// 也正因如此，其返回值是 `Result<T, E>` 型。
fn main() {
    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
