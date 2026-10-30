fn next_birthday(current_age: Option<u8>) -> Option<String> {
    let next_age = current_age?;
    Some(format!("Next year I will be {}", next_age))
}

struct Person {
    job: Option<Job>,
}

#[derive(Copy, Clone)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Copy, Clone)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        // 不使用 ? 链接 option 的话要嵌套 match
        // return match self.job {
        //     Some(job) => match job.phone_number {
        //         Some(phone_number) => match phone_number.area_code
        //         None => None,
        //     },
        //     None => None,
        // };
        // 可以将多个 ? 链接在一起，以使代码更具可读性。
        self.job?.phone_number?.area_code
    }
}

/// 可以使用 match 语句来解开 Option，但使用 ? 运算符通常会更容易。
/// 如果 x 是 Option，那么若 x 是 Some ，对x?表达式求值将返回底层值，
/// 否则无论函数是否正在执行都将终止且返回 None。
fn main() {
    println!("{:?}", next_birthday(Some(12)));
    let person = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 123892183,
            }),
        }),
    };
    assert_eq!(person.work_phone_area_code(), Some(61));
}
