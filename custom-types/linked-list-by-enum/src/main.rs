use List::*;

/// `enum` 的常见用法：创建链表（linked-list）
enum List {
    // Cons: 一个元组结构体，包含链表的一个元素和一个指向下一个节点的指针
    Cons(u32, Box<List>),
    // Nil: 链表末尾节点，表明链表结束
    Nil,
}

/// 给 `enum` 定义方法
impl List {
    /// 创建一个空的链表
    fn new() -> List {
        Nil
    }

    /// 处理一个链表，在其头部插入新的元素，并返回该 List
    /// # Arguments
    /// - self 当前链表实例
    /// - element 要插入到链表头部的元素
    fn prepend(self, element: u32) -> List {
        Cons(element, Box::new(self))
    }

    /// 获取链表的长度
    fn len(&self) -> u32 {
        // 必须对 `self` 进行匹配（match），因为这个方法的行为取决于 `self` 的
        // 取值种类。
        // `self` 为 `&List` 类型，`*self` 为 `List` 类型，
        // 匹配一个具体的 `T` 类型要好过匹配引用 `&T`。
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    /// 返回列表的字符串表示（该字符串是堆分配的）
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` 和 `print!` 类似，但返回的是一个堆分配的字符串，
                // 而不是打印结果到控制台上
                format!("{}, {}", head, tail.stringify())
            }
            Nil => "Nil".to_string(),
        }
    }
}

fn main() {
    let mut list = List::new();
    // 追加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 显示链表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
