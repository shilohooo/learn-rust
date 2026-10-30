/// # 字符串
///
/// Rust 中有两种字符串类型：`String` 和 `&str`。
///
/// `String` 被存储为由字节组成的 vector（`Vec<u8>`），但保证了它一定是一个有效的
/// UTF-8 序列。`String` 是堆分配的，可增长的，且不是零结尾的（null terminated）。
///
/// `&str` 是一个总是指向有效 UTF-8 序列的切片（`&[u8]`），并可用来查看 `String` 的内容，就如同 `&[T]` 是 `Vec<T>` 的全部或部分引用。
///
/// ## 字面量与转义字符
///
/// 书写含有特殊字符的字符串字面量有很多种方法。它们都会产生类似的 `&str`，所以最好选择最方便的写法。类似地，字节串（byte string）字面量也有多种写法，它们都会产生
/// `&[u8; N]` 类型。
///
/// 通常特殊字符是使用反斜杠字符 `\` 来转义的，这样你就可以在字符串中写入各种各样的字符，甚至是不可打印的字符以及你不知道如何输入的字符。如果你需要反斜杠字符，再用另一个反斜杠来转义它就可以，像这样：`\\`。
///
/// 字面量中出现的字符串或字符定界符必须转义：`"\""`、`'\''`。
fn main() {
    // 这是一个只读内存中分配的字符串的引用
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // 逆序迭代单词，这里不会分配新的字符串
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // 复制字符串到到一个 vector，排序并移除重复值
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();
    println!("{:?}", chars);

    // 创建一个空字符串，可增长
    let mut string = String::new();
    for c in chars {
        // 在字符串的尾部插入一个字符
        string.push(c);
        // 在字符串的尾部插入一个字符串
        string.push_str("，");
    }
    println!("{}", string);

    // 字符串切片，不会产生新的分配操作
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // 堆分配一个字符串
    let alice = String::from("I like dogs");
    // 分配新内存并存储修改过的字符串
    let bob = alice.replace("dog", "cat");
    println!("Alice say: {}", alice);
    println!("Bob say: {}", bob);

    // 通过转移，可以使用十六进制值来表示字符串
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // 也可以使用 Unicode 码位表示字符串
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    let long_string = "String literals\
    can span multiple lines.\
    The linebreak and indentation here->\
    <- can be escaped too!";
    println!("{}", long_string);

    // 原始字符串：写啥就显示啥
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);
    // 在原始字符串中使用引号，需要把原始字符串包含在##里面
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);
    // 如果字符串里面需要写 # 这个符号，则需要在定界符中使用更多的 #
    // # 的数量没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    // 非 UTF-8 字符串，需要使用字节数组，如果内容大部分是文本，可以使用字节串 b'xxxx';
    let bytestring = b"this is a byte string";
    // 但是字节串没有实现 Display trait，打印的时候输出的是每个字符对应的 ASCII 码
    println!("A byte string: {:?}", bytestring);

    // 字节串可以使用单字节的转义字符
    let byte_str_escaped = b"\x52\x75\x73\x74 as bytes";
    // 但是不能使用 Unicode 转义字符
    // 这里会提示错误：unicode escape sequences cannot be used as a byte or in a byte string
    // let byte_str_unicode_escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", byte_str_escaped);

    // 原始字节串，写法和原始字符串一样
    let raw_byte_str = br"\u{211D} is not escaped here";
    println!("{:?}", raw_byte_str);

    // 把字节串转换为 &str 可能会失败，需要用模式匹配去处理
    if let Ok(my_str) = str::from_utf8(raw_byte_str) {
        println!("And the same as text: '{}'", my_str);
    }

    let quotes = br#"You can also use "fancier" formatting,\
    like with normal raw strings"#;

    // 字节串可以不使用 utf-8 编码
    let shift_jis = b"\x82\xe6\x82\xa8\x82\x82";
    // 不过这样就没办法转换为 &str 了
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: {}", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    }
}
