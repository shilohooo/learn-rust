#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

/// # 作用域规则 - 部分移动
///
/// 在单个变量的解构内，可以同时使用 by-move 和 by-reference 模式绑定。
///
/// 这样做将导致变量的部分移动（partial move），这意味着变量的某些部分将被移动，而其他部分将保留。
///
/// 在这种情况下，后面不能整体使用父级变量，但是仍然可以使用只引用（而不移动）的部分。
fn main() {
    let person = Person {
        name: String::from("Shiloh"),
        age: 18,
    };

    // name 从 person 结构体中移走了，但 age 只是引用
    let Person { name, ref age } = person;

    println!("The person's age is: {}", age);
    println!("The person's name is: {}", name);

    // 下面这个语句会报错：Value used after being moved [E0382]
    // 因为 person 的 name 被移走了，不能直接使用整个结构体了
    // println!("The person struct is: {:?}", person);

    // 但是可以使用 person 的 age 字段，因为它没有被移动（ref）
    println!("The person's age from person struct is: {}", person.age);
}
