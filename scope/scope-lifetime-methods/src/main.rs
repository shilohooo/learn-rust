struct Owner(i32);

impl Owner {
    /// 标注生命周期，就像独立的函数一样
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }

    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

/// # 作用域规则 - 生命周期
///
/// 方法的生命周期标注，类似函数的生命周期标注
///
/// 方法一般是不需要标明生命周期的，
///
/// 因为 self 的生命周期会赋给所有的输出生命周期参数，详见 [TRPL](https://rustwiki.org/zh-CN/book/ch10-03-lifetime-syntax.html#%E7%94%9F%E5%91%BD%E5%91%A8%E6%9C%9F%E7%9C%81%E7%95%A5lifetime-elision)。
fn main() {
    let mut owner = Owner(233);
    owner.add_one();
    owner.print();
}
