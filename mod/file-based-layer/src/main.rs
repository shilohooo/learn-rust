// 写第三方库的时候，没有 main.rs，只有 lib.rs，这个时候要加上 pub，客户端才可以访问指定的模块
// pub mod my;
mod my;

fn function() {
    println!("called `function()`");
}

/// 基于文件的模块分层
fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}
