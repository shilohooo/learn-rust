#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier
}

/// use `enum`
fn main() {
    // 显式地 `use` 枚举的各个成员，后续就不要 `Enum::Xxx` 了
    use Status::{Poor, Rich};
    // `use` 枚举中的全部成员
    use Work::*;
    // `Poor` 等价于 Status::Poor
    let status = Poor;
    // `Civilian` 等价于 `Work::Civilian`
    let work = Civilian;

    match status {
        // 注意：这里没有使用完整的路径：Status::Rich，因为上面显式地使用了 `use`
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers work!"),
    }
}
