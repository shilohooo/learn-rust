struct Container(i32, i32);

/// `A` 和 `B` 在 trait 里面通过 `type` 关键字来定义。
/// （注意：此处的 `type` 不同于为类型取别名时的 `type`）
trait Contains {
    // 使用关联类型
    type A;
    type B;

    /// 在 trait 定义的方法中使用关联类型
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // 在实现中，需要明确指定自定义的关联类型的实际类型是什么
    // 为 `Container(i32, i32)`，那么 `output`（输出）类型
    // 会被确定为 `i32` 和 `i32`。
    type A = i32;
    type B = i32;

    fn contains(&self, num1: &i32, num2: &i32) -> bool {
        &self.0 == num1 && &self.1 == num2
    }

    // `&Self::A` 和 `&Self::B` 在这里也是合法的类型。
    // fn contains(&self, num1: &Self::A, num2: &Self::B) -> bool {
    //     &self.0 == num1 && &self.1 == num2
    // }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

// fn difference<A, B, C>(container: &C) -> i32
// where
//     C: Contains<A, B>,
// {
//     container.last() - container.first()
// }

// 使用关联类型
fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

/// # 泛型关联项
///
/// “关联项”（associated item）指与多种类型的项有关的一组规则。
///
/// 它是 trait 泛型的扩展，允许在 trait 内部定义新的项。
fn main() {
    let num1 = 3;
    let num2 = 10;

    let container = Container(num1, num2);

    println!(
        "Does container contain {} and {}: {}",
        &num1,
        &num2,
        container.contains(&num1, &num2)
    );

    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
