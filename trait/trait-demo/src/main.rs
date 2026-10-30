struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    /// 静态方法签名；`Self` 表示实现者类型（implementor type）。
    fn new(name: &'static str) -> Self;

    /// 实例方法签名；这些方法将返回一个字符串。
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    /// trait 可以提供默认的方法定义。
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Self {
        Sheep { name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaaah?"
        } else {
            "baaaaaah!"
        }
    }

    /// 重载 trait 的默认实现的方法
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name(), self.noise());
    }
}

/// # Trait - 特质/特征
///
/// trait 是对未知类型 Self 定义的方法集。该类型也可以访问同一个 trait 中定义的其他方法。
///
/// 对任何数据类型都可以实现 trait。
fn main() {
    // 这种情况需要显式的类型标注
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}
