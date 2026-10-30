/// 这是一个名称为 `my_mod` 的模块
mod my_mod {
    /// 模块中的成员默认具有私有的可见行（Private）
    fn private_fn() {
        println!("called `my_mod::private_fn()`")
    }

    /// 可以使用 `pub` 修饰符来改变模块中的成员的可见性
    pub fn function() {
        println!("called `my_mod::pub_fn()`")
    }

    /// 同一个模块内的成员可以互相访问，不受可见性影响
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_fn();
    }

    /// 模块支持嵌套
    pub mod my_nested_mod {
        pub fn function() {
            println!("called `my_mod::my_nested_mod::function()`")
        }

        #[allow(dead_code)]
        fn private_fn() {
            println!("called `my_mod::my_nested_mod::private_fn()`")
        }

        /// 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
        /// `path` 必须是父模块（parent module）或祖先模块（ancestor module）
        pub(in crate::my_mod) fn pub_fn_in_my_mod() {
            print!("called `my_mod::nested::pub_fn_in_my_mod()`, that\n> ");
            pub_fn_in_nested();
        }

        /// 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
        pub(self) fn pub_fn_in_nested() {
            println!("called `my_mod::my_nested_mod::pub_fn_in_nested()`")
        }

        /// 使用 `pub(super)` 语法定义的函数只在父模块中可见。
        pub(super) fn pub_fn_in_super_mod() {
            println!("called `my_mod::my_nested_mod::pub_fn_in_super_mod()`")
        }
    }

    pub fn call_pub_fn_in_my_mod() {
        print!("called `my_mod::call_pub_fn_in_my_mod()`, that\n> ");
        my_nested_mod::pub_fn_in_my_mod();
        print!("> ");
        my_nested_mod::pub_fn_in_super_mod();
    }

    /// `pub(crate)` 使得函数只在当前 `crate` 中可见
    pub(crate) fn pub_fn_in_crate() {
        println!("called `my_mod::pub_fn_in_crate()`")
    }

    /// 嵌套模块的可见性遵循相同的规则，默认是 Private
    mod private_nested_mod {
        pub fn function() {
            println!("called `my_mod::private_nested_mod::function()`")
        }
    }
}

fn function() {
    println!("called `function()`")
}


/// # Rust 模块化之可见性（visibility）
///
/// 默认情况下，模块中的项拥有私有的可见性（private visibility），
///
/// 不过可以加上 `pub` 修饰语来重载这一行为。
///
/// 模块中只有公有的（public）项可以从模块外的作用域访问。
fn main() {
    // 不同模块之间，允许函数名称相同
    function();
    my_mod::function();

    my_mod::indirect_access();
    my_mod::my_nested_mod::function();
    my_mod::call_pub_fn_in_my_mod();

    my_mod::pub_fn_in_crate();

    // 下面这个语句会报错：pub_fn_in_my_mod 只能在指定的模块里面访问
    // my_mod::my_nested_mod::pub_fn_in_my_mod();

    // 模块的私有成员不能直接访问，即便是嵌套在公共的模块内部也不行
    // my_mod::private_nested_mod::function();
}
