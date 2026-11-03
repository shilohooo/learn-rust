#![crate_name = "doc_demo"]

/// 人员信息
pub struct Person {
    /// 姓名
    name: String,
}

impl Person {
    /// 构造器
    ///
    /// # Params
    ///
    /// * `name` - 人员姓名
    ///
    /// # Examples
    ///
    /// ```rust
    /// use doc_demo::Person;
    /// let person = Person::new("your name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: String::from(name),
        }
    }

    /// 友好的问候～
    pub fn hello(&self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    let person = Person::new("Shiloh");
    person.hello();
}
