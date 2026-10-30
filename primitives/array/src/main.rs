// 创建一个借用 slice 的函数
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

/// 数组和切片
///
/// 数组是一组拥有相同类型的对象集合，在内存中是连续存储的，使用中括号`[T; length]`来创建
///
/// T 是元素的类型， length 是数组的大小，在编译时就已确定
///
/// 切片（slice）类型和数组类似，但其大小在编译时是不确定的。切片是一个双字对象（tow-word object）
///
/// 第一个字是一个指向数据的指针，第二个字是切片的长度。这个字的宽度和 usize 相同，由处理器架构决定（32位/64位）
///
/// slice 的类型标记为：`&[T]`
fn main() {
    // 定长数组，显式声明类型
    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", nums);
    // 数组里面的元素，值可以是一样的
    let same_val_arr: [i32; 500] = [0; 500];
    // 访问数组的元素，下标从0开始
    println!("first element of the array: {}", nums[0]);
    println!("second element of the array: {}", nums[1]);
    // 获取数组的大小
    println!("array size: {}", nums.len());
    println!("array size: {}", same_val_arr.len());
    // 数组是在栈（Stack）中进行分配的
    println!("array occupies {} bytes", size_of_val(&nums));
    println!("array occupies {} bytes", size_of_val(&same_val_arr));
    // 数组可以自动被借用成为 slice
    println!("borrow a section of the array as a slice");
    // 切片语法 [start(include)..end(exclude)]
    analyze_slice(&same_val_arr[1..4]);
    // 数组下标越界会引发致命错误（panic）
    // println!("{}", nums[5]);
}
