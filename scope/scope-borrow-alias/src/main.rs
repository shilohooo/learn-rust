struct Point {
    x: i32,
    y: i32,
    z: i32,
}

/// # 作用域规则 - 别名
///
/// 数据可以多次不可变借用，但是在不可变借用的同时，原始数据不能使用可变借用。
///
/// 或者说，同一时间内只允许一次可变借用。仅当最后一次使用可变引用之后，原始数据才可以再次借用。
fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    // 多次不可变的借用
    let borrowed_point = &point;
    let another_borrow = &point;

    // 数据可以通过引用或原始类型来访问
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // 下面这个语句会报错：Cannot borrow `point` as mutable because it is also borrowed as immutable
    // 因为在下面的打印语句中，使用了 point 的不可变借用
    // 当还有不可变借用存在时，不能使用可变借用
    // let mutable_borrow = &mut point;

    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // 在这里，不可变的引用不再用于其他代码了，此时可以使用可变的引用再重新借用 point
    let mutable_borrow = &mut point;

    // 下面的语句会报错：Cannot borrow `point.y` as immutable because it is also borrowed as mutable
    // let y = &point.y;
    // 下面的语句会报错：Cannot borrow `point.z` as immutable because it is also borrowed as mutable
    // 因为 point 已被可变引用借用了
    // println!("Point Z coordinates is {}", point.z);

    mutable_borrow.x = 1;
    mutable_borrow.y = 2;
    mutable_borrow.z = 3;

    println!(
        "Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    // 到这里，不可变引用不再使用了，可以重新借用
    let new_borrowed_point = &point;

    println!(
        "Point has coordinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );
}
