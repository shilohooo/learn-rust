trait Person {
    fn name(&self) -> String;
}

/// Person 是 Student 的父 trait
///
/// 实现 Student 需要同时实现 Person
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_lang(&self) -> String;
}

/// CsStudent 是 Student + Programmer 两者的子类
trait CsStudent: Student + Programmer {
    fn git_username(&self) -> String;
}

fn greeting(student: &dyn CsStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_lang(),
        student.git_username()
    )
}

/// # trait - super trait
///
/// Rust 没有继承的概念，但是可以将一个 trait 定义为另一个 trait 的超集（即父 trait）
fn main() {
    println!("Hello, world!");
}
