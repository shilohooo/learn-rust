trait UsernameWidget {
    /// 获取选定的用户名
    fn get(&self) -> String;
}

trait AgeWidget {
    /// 获取选定的年龄
    fn get(&self) -> u8;
}

struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

/// # trait - 消除重叠 trait
///
/// 一个类型可以实现许多不同的 trait。如果两个 trait 都需要相同的名称怎么办？
///
/// 例如，许多 trait 可能拥有名为 get() 的方法。他们甚至可能有不同的返回类型！
///
/// 有个好消息：由于每个 trait 实现都有自己的 impl 块，因此很清楚您要实现哪个 trait 的 get 方法。
///
/// 何时需要调用这些方法呢？为了消除它们之间的歧义，我们必须使用完全限定语法（Fully Qualified Syntax）。
fn main() {
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };

    // 下面这行代码会报错：Multiple applicable items in scope
    // println!("{}", form.get())

    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}
