/// 遍历 Result
fn main() {
    let strings = vec!["tofu", "39", "44"];
    // Result 实现了 FromIter，
    // 因此结果的向量（Vec<Result<T, E>>）可以被转换成结果包裹着向量（Result<Vec<T>, E>）。
    // 一旦找到一个 Result::Err ，遍历就被终止。
    // let numbers: Result<Vec<_>, _> = strings
    //     .into_iter()
    //     .map(|s| s.parse::<i32>())
    //     // 使用 filter_map 忽略处理失败的项
    //     // filter_map 会调用一个函数，过滤掉为 None 的所有结果
    //     // .filter_map(|s| s.parse::<i32>().ok())
    //     .collect();

    // 使用 Partition() 收集所有合法的值与错误
    let (num, err): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    // println!("Result: {:?}", numbers)
    // 上面的数据都包裹在 Result 中，取出它们需要一些模版代码
    let num_unwrap: Vec<_> = num
        .into_iter()
        // 取出数据
        .map(Result::unwrap)
        .collect();
    let err_unwrap: Vec<_> = err
        .into_iter()
        // 取出错误信息
        .map(Result::unwrap_err)
        .collect();
    println!("Number: {:?}", num_unwrap);
    println!("Error: {:?}", err_unwrap);
}
