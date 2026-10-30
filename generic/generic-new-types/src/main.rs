struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 > 18
}

/// # 泛型之 new type 惯用法
///
/// `newtype` 惯用法（即为不同种类的数据分别定义新的类型）
///
/// 能保证在编译时，提供给程序的都是正确的类型。
fn main() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough: {}", old_enough(&age));
    println!("Old enough: {}", old_enough(&age_days.to_years()));
    // 下面这个语句会报错：Type mismatch [E0308]expected `&Years`, but found `&Days`
    // println!("Old enough: {}", old_enough(&age_days));
}
