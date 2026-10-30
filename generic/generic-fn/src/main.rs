struct A;
struct S(A);
struct GenericStruct<T>(T);

/// 定义一个参数是具体已知的类型的函数
fn reg_fn(_s: S) {

}

/// 定义一个函数 `gen_spec_t`，接受一个 `GenericStruct<A>` 类型的参数 `_s`。
/// `GenericStruct<>` 显式地接受了类型参数 `A`，且在 `gen_spec_t` 中，`A` 没有被用作
/// 泛型类型参数，所以函数不是泛型的。
fn gen_spec_t(_s: GenericStruct<A>) {

}

/// 定义一个函数 `gen_spec_i32`，接受一个 `GenericStruct<i32>` 类型的参数 `_s`。
/// `GenericStruct<>` 显式地接受了类型参量 `i32`，而 `i32` 是一个具体类型。
/// 由于 `i32` 不是一个泛型类型，所以这个函数也不是泛型的。
fn gen_spec_i32(_s: GenericStruct<i32>) {

}

/// 定义一个函数 `generic`，接受一个 `GenericStruct<T>` 类型的参数 `_s`。
/// 因为 `GenericStruct<T>` 之前有 `<T>`，所以这个函数是关于 `T` 的泛型函数。
fn generic_fn<T>(_s: GenericStruct<T>) {
    
}

/// # 泛型在函数中的运用
fn main() {
    println!("Hello, world!");

    // 调用非泛型函数
    reg_fn(S(A));
    gen_spec_t(GenericStruct(A));
    gen_spec_i32(GenericStruct(2));

    // 调用泛型函数，但是显示指定泛型的具体类型
    generic_fn::<char>(GenericStruct('a'));

    // 调用泛型函数，自动推到泛型的具体类型
    generic_fn(GenericStruct("Str"));
}
