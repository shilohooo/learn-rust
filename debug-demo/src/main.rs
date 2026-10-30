// 推到 `Structure` 结构体的 `std:fmt:Debug` 实现，使其可以被打印
#[derive(Debug)]
struct Structure(i32);

// 将 `Structure` 结构体放到 `Deep` 中，然后使 `Deep` 结构体也能够打印
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    // 使用 `{:?}` 打印和使用 `{}` 类似
    println!("{:?} months is a year", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name",
        "Slater",
        "Christian",
        actor = "actor's"
    );
    // 打印结构体，结构体内容输出的是：Structure(342)
    println!("Now {:?} will print!", Structure(342));
    // 使用 derive 到一个问题是不能控制输出的方式
    println!("Now {:?} will print!", Deep(Structure(342)));

    // 使用 {:#?} 美化输出内容
    let name = "shiloh";
    let age = 28;
    let person = Person { name, age };
    println!("{:#?}", person);
}
