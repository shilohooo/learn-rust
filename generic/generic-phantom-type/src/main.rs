use std::marker::PhantomData;
use std::ops::Add;

/// 这个虚元组结构体对 `A` 是泛型的，并且带有隐藏参数 `B`。
///
/// 允许这种类型进行相等测试（equality test）
///
/// **注意：** 对于泛型 `A` 会分配存储空间，但 `B` 不会。因此，`B` 不能参与运算
#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

#[derive(PartialEq)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

// region test

#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

/// `Length` 是一个带有虚类型参数 `Unit` 的类型，
///
/// 而且对于表示长度的类型（即 `f64`）而言，`Length` 不是泛型的。
///
/// `f64` 已经实现了 `Clone` 和 `Copy` trait.
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

/// `Add` trait 定义了 `+` 运算符的行为。
impl<Unit> Add for Length<Unit> {
    // 两个 Length 相加，它的结果还是 Length 类型
    type Output = Length<Unit>;

    /// add() 返回一个含有和的新的 `Length` 结构体。
    ///
    /// ## Examples:
    /// ```
    /// // length1 = self, length2 = rhs
    /// length1 + length2
    /// ```
    fn add(self, rhs: Length<Unit>) -> Self::Output {
        Length(self.0 + rhs.0, PhantomData)
    }
}

// endregion

/// # 泛型：虚类型参数（phantom type）
///
/// 虚类型（phantom type）参数是一种在运行时不出现，
///
/// 而在（且仅在）编译时进行静态检查的类型参数。
///
/// 可以用额外的泛型类型参数指定数据类型，该类型可以充当标记，也可以供编译时类型检查使用。
///
/// 这些额外的参数没有存储值，也没有运行时行为。
fn main() {
    // f32 是一个隐藏参数
    let tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // f64 是一个隐藏参数
    let tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // 下面这个语句会报错：Type mismatch [E0308]expected `PhantomTuple<char, f32>`, but found `PhantomTuple<char, f64>`
    // 因为它们都类型不匹配
    // 把虚类型设置为一样，才能通过编译
    // println!("tuple1 == tuple2 yields: {}", tuple1 == tuple2);
    // println!("struct1 == struct2 yields: {}", struct1 == struct2);

    // region test

    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let tow_meters = one_meter + one_meter;

    println!("One foot + One foot = {:?} inch", two_feet.0);
    println!("One meter + One meter = {:?} mm", tow_meters.0);

    // 下面这个语句会报错：Type mismatch [E0308]expected `Length<Inch>`, but found `Length<Mm>`
    // 因为泛型类型（单位）不同，不能相加，这是通过泛型实现的限制
    // Length<Inch> + Length<Inch> / Length<Mm> + Length<Mm>
    // one_foot + one_meter;

    // endregion
}
