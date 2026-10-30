use deeply::nested::function as other_fn;

fn function() {
    println!("called `function()`")
}

mod deeply{
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`")
        }
    }
}

/// `use` 声明
///
/// `use` 声明可以将一个完整的路径绑定到一个新的名字，从而更容易访问。
fn main() {
    other_fn();

    println!("Entering block");

    {
        // 与 `use deeply::nested::function as other_fn;` 相同
        // 但是这里声明的是 function，它会遮蔽外部的同名函数
        use deeply::nested::function;
        function();

        // `use` 声明拥有局部作用域，`function` 函数的遮蔽仅在该代码块中
        println!("Leaving block");
    }

    function();
}

