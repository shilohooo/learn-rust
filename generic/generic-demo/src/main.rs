struct A;

struct Single(A);

struct GenericStruct<T>(T);

fn foo<T>(arg: T) {
    // do something...
}

/// # 泛型（Generic）
///
/// 泛型（generic）是关于泛化类型和函数功能，以扩大其适用范围的话题。
///
/// 泛型极大地减少了代码的重复，但它自身的语法很要求细心。
///
/// 也就是说，采用泛型意味着仔细地指定泛型类型具体化时，什么样的具体类型是合法的。
///
/// 泛型最简单和常用的用法是用于类型参数。
///
/// > 定义泛型类型或泛型函数之类的东西时，我们会用 <A> 或者 <T> 这类标记 作为类型的代号，
/// > 就像函数的形参一样。在使用时，为把 <A>、<T> 具体化，我们会把类型说明像实参一样使用，
/// > 像是 <i32> 这样。这两种把（泛型的或具体的）类型 当作参数的用法就是类型参数。
fn main() {
    let single = Single(A);

    // 显示指定泛型参数
    let generic_struct: GenericStruct<char> = GenericStruct('a');

    // 自动推到泛型参数
    let _t = GenericStruct(1);
    let _i32 = GenericStruct(A);
    let _str = GenericStruct("sss");
}
