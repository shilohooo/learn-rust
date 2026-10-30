struct S;
struct GenericStruct<T>(T);

impl GenericStruct<f32> {}
impl GenericStruct<S>{}

/// 泛型 Impl
impl <T> GenericStruct<T> {}

struct Val {
    val: f64,
}

struct GenericVal<T> {
    generic_val: T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl <T> GenericVal<T> {
    fn value(&self) -> &T {
        &self.generic_val
    }
}


/// # 带泛型的 `Impl`
fn main() {
    println!("Hello, world!");

    let x = Val { val: 3.0 };
    let y = GenericVal { generic_val: 3i32 };
    println!("{}, {}", x.value(), y.value());
}
