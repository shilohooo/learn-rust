#[derive(Debug, Copy, Clone)]
struct Nil;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

/// # Clone trait
///
/// 当处理资源时，默认的行为是在赋值或函数调用的同时将它们转移。
///
/// 但是我们有时候也需要把资源复制一份。
fn main() {
    let nil = Nil;

    let copied_nil = nil;

    // 两个 Nil 实例都可以独立使用
    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);

    // 原来的 pair 不能再使用了，因为所有权已经转移到 moved_pair 了
    // println!("original: {:?}", pair);

    let cloned_pair = moved_pair.clone();
    drop(moved_pair);

    // moved_pair 被手动销毁了，也不能再使用了
    // println!("copy: {:?}", moved_pair);

    // 由 .clone() 得来的结果仍可以使用
    println!("clone: {:?}", cloned_pair);
}
