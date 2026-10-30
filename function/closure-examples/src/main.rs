/// # 标准库中闭包的例子
fn main() {
    // Iterator::any
    // 判断集合中是否有满足谓词到元素，有就返回 true，否则返回 false

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec 的 `iter()` 举出 `&i32`。（通过用 `&x` 匹配）把它解构成 `i32`。
    // 注意 `any` 方法会自动地把 `vec.iter()` 举出的迭代器的元素一个个地
    // 传给闭包。因此闭包接收到的参数是 `&i32` 类型的。
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // 对 vec 的 `into_iter()` 举出 `i32` 类型。无需解构。
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));

    // Iterator::find
    // Iterator::find 是一个函数，在传给它一个迭代器时，
    // 将用 Option 类型返回第一个满足谓词的元素。

    let vec3 = vec![1, 2, 3];
    let vec4 = vec![4, 5, 6];

    let mut iter = vec3.iter();
    let mut into_iter = vec4.into_iter();

    // 对迭代器举出的元素的引用是 `&&i32` 类型。解构成 `i32` 类型。
    // 注意 `find` 方法会把迭代器元素的引用传给闭包。
    // 迭代器元素自身是 `&i32` 类型，所以传给闭包的是 `&&i32` 类型。
    // 在闭包或者模式匹配中。`&` 符号的作用是解引用，
    // 这里的 `&&x` 是对 `&&i32` 解了两次引用：`&&i32` -> `&i32` -> `i32`
    // 相当于：|x| **x == 2
    println!("Find 2 in vec3: {:?}", iter.find(|&&x| x == 2));
    // 对迭代器举出的元素的引用是 `&i32` 类型。解构成 `i32` 类型。
    println!("Find 2 in vec4: {:?}", into_iter.find(|&x| x == 2));

    let array3 = [1, 2, 3];
    let array4 = [4, 5, 6];

    println!("Find 2 in vec3: {:?}", array3.iter().find(|&&x| x == 2));
    println!("Find 2 in vec4: {:?}", array4.into_iter().find(|&x| x == 2));
}
