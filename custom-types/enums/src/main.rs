// 这个属性可以隐藏编译器对未使用的代码的警告 ⚠️
#![allow(dead_code)]

/// 创建一个枚举来对 web 事件进行分类，注意变量名和类型共同指定来 `enum` 取值的种类
///
/// `PageLoad` 和 `PageUnload` 是不一样的
enum WebEvent {
    /// `enum` 的成员可以是单元结构体（称为 `unit-like` 或 `uni`）
    PageLoad,
    PageUnload,
    /// 也可以定义成一个元组结构体
    KeyPress(char),
    Paste(String),
    /// 或者一个普通的结构体
    Click {
        x: i64,
        y: i64,
    },
}

/// 根据枚举类型来匹配不同的处理逻辑
fn inspect(web_event: WebEvent) {
    match web_event {
        WebEvent::PageLoad => println!("page loaded!"),
        WebEvent::PageUnload => println!("page unload!"),
        WebEvent::KeyPress(c) => println!("key '{}' pressed", c),
        WebEvent::Paste(s) => println!("paste \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x = {}, y = {}", x, y);
        }
    }
}

/// 类型别名，可以给一个名称很长的类型进行重命名，然后通过简化的名称去引用它的成员
#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}
/// 使用 `type` 关键字创建一个类型别名
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

/// 常用方式是在 `impl` 块中使用 `Self` 别名
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

/// Rust Enum（枚举）
///
/// enum 关键字允许创建一个从数个不同的值中选其一的枚举类型。
///
/// 任何一个在 struct 中合法的取值在 enum 中也是合法的
fn main() {
    let pressed = WebEvent::KeyPress('x');
    inspect(pressed);
    // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`，通常用于复制数据
    let pasted = WebEvent::Paste("my text".to_owned());
    inspect(pasted);
    let click = WebEvent::Click { x: 20, y: 70 };
    inspect(click);
    let load = WebEvent::PageLoad;
    inspect(load);
    inspect(WebEvent::PageUnload);

    // 使用别名引用枚举成员
    let add = Operations::Add;
    println!("{:?}", add);
    println!("{}", add.run(5, 1));
}
