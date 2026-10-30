#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

/// # 作用域规则 - ref 模式
///
/// 在通过 let 绑定来进行模式匹配或解构时，ref 关键字可用来创建结构体/元组的字段的引用。
fn main() {
    let c = 'Q';

    // ref 变量名称 = 变量名称 = &variable
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // 解构一个结构体的时候，也可以使用 ref
    let _copy_of_x = {
        // ref_to_x 是一个指向 point 结构体的 x 字段的引用
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;
        ref_to_x
    };

    // point 的可变拷贝
    let mut mutable_point = point;

    {
        // ref 可以与 mut 结合创建可变引用
        let Point {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point;
        // 通过可变引用来修改 mutable_point 的字段 y
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!(
        "mutable point is ({}, {})",
        mutable_point.x, mutable_point.y
    );

    // 创建一个包含一个指针的可变元组
    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        // 解构可变元组
        let (_, ref mut last) = mutable_tuple;
        // 通过可变引用修改元组中的元素
        *last = 12;
    }

    println!("{:?}", mutable_tuple);
}
