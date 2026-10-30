#[allow(dead_code)]
#[derive(Copy, Clone)]
struct Book {
    // &'static str 是一个对分配在只读内存区的字符串的引用
    author: &'static str,
    title: &'static str,
    year: u32,
}

/// 此函数的参数是一个对 Book 类型的引用
fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

/// 此函数的参数是一个可变的 Book 类型的引用，函数内部可以修改 Book 数据
fn new_edition(book: &mut Book) {
    book.year = 2024;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

/// # 作用域规则 - 借用 - 可变性
///
/// 可变数据可以使用 &mut T 进行可变借用。这叫做可变引用（mutable reference），
///
/// 它使借用者可以读/写数据。相反，&T 通过不可变引用（immutable reference）来借用数据，
///
/// 借用者可以读数据而不能更改数据
fn main() {
    let immutable_book = Book {
        // 字符串字面量的类型为：&'static str
        author: "吴承恩",
        title: "西游记",
        year: 1542,
    };

    // 创建一个 immutable_book 的可变拷贝
    let mut mutable_ref = immutable_book;

    borrow_book(&immutable_book);

    borrow_book(&mutable_ref);

    // 可变地借用一个可变对象
    new_edition(&mut mutable_ref);

    // 下面这个调用会报错：Cannot borrow immutable local variable `immutable_book` as mutable
    // 因为 immutable_book 是不可变的
    // new_edition(&mut immutable_book);
}
