use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

impl <T> PrintInOption for T where Option<T>: Debug {
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

// #[derive(Debug)]
struct A;

/// # 泛型 where 分局
///
/// 约束也可以使用 `where` 分句来表达，它放在 { 的前面，而不需写在类型第一次出现之前。
///
/// 另外 `where` 从句可以用于任意类型的限定，而不局限于类型参数本身。
///
/// `where` 在下面一些情况下很有用：
/// - 当分别指定泛型的类型和约束会更清晰时：
///
/// ```
/// impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}
///
/// // 使用 `where` 从句来表达约束
/// impl <A, D> MyTrait<A, D> for YourType where
///     A: TraitB + TraitC,
///     D: TraitE + TraitF {}
/// ```
fn main() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();


    let a = A;
    // 下面这个语句会报错：The method `print_in_option` exists for struct `A`,
    // but its trait bounds were not satisfied
    // 因为自定义的结构 A，它没有实现 Debug trait
    // a.print_in_option()
}
