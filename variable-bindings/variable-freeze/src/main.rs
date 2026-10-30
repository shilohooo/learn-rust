/// 变量冻结
fn main() {
    // 当数据被相同的名称不变地绑定时，它还会冻结（freeze）
    // 在不可变绑定超出作用域之前，无法修改已冻结的数据
    let mut  _mutable_int = 7i32;

    {
        // 可变的绑定被不可变绑定遮蔽了
        let _mutable_int = _mutable_int;

        // 下面的赋值语句会报错，因为变量已经变成不可变了
        // _mutable_int = 50;

        // _mutable_int 离开作用域
    }

    // 这里改变变量的值是正常的～
    _mutable_int = 4;

    println!("{}", _mutable_int);
}
