use std::fmt;
use std::fmt::Formatter;

// 这个 extern 代码块链接到 libm 库
// 新版的（2024 Edition）rust 要求 extern 要加 unsafe
//
// 它告诉链接器：把名为 m 的库链接进来。在 Linux / macOS 上，这个 m 就是 libm（math library），属于 C 标准库的一部分，通常位于：
//
// Linux：/usr/lib/x86_64-linux-gnu/libm.so（或类似路径）
//
// macOS：/usr/lib/libm.dylib
//
// Windows：MSVC 工具链下没有独立的 libm，csqrtf / ccosf 一般由 ucrt 提供（有时需要换写法，这也是为什么这段示例在 Linux/macOS 上最顺）
//
// csqrtf 和 ccosf 就是 libm 里早已编译好的 C 函数，extern "C" 块只是告诉 Rust 编译器：“这些符号存在于外部，别管实现，按 C 调用约定去调就行。”
#[link(name = "m")]
unsafe extern "C" {
    // 这个外部函数用于计算单精度复数的平方根
    fn csqrtf(z: Complex) -> Complex;
    // 这个用来计算单精度复数的复变余弦
    fn ccosf(z: Complex) -> Complex;
}

// 由于调用外部语言函数被认为是不安全的，通常都会包一层
fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

/// # 外部语言函数接口
///
/// Rust 提供了到 C 语言库的外部语言函数接口（Foreign Function Interface，FFI）。
///
/// 外部语言函数必须在一个 `extern` 代码块中声明，且该代码块要带有一个包含库名称的 #[link] 属性。
fn main() {
    let z = Complex { re: -1., im: 0. };
    // 调用外部语言函数是不安全操作，需要放 unsafe 里面
    let z_sqrt = unsafe { csqrtf(z) };

    println!("the square root of {:?} is {:?}", z, z_sqrt);

    // 调用封装了不安全操作的安全接口
    println!("cos({:?}) = {:?}", z, cos(z));
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}
