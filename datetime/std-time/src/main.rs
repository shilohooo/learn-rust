use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// # Rust 日期时间处理
///
/// - SystemTime 表示系统时钟
/// - Instant 用于测量经过的时间，适合统计耗时、控制超时时间
fn main() {
    // 获取当前系统时间
    let now = SystemTime::now();
    println!("当前系统时间：{:?}", now);
    // 获取 Unix 时间戳，单位：秒
    let timestamp = now
        .duration_since(UNIX_EPOCH)
        .expect("系统时间早于 Unix 纪元");
    println!("秒级时间戳：{:?}", timestamp.as_secs());
    println!("豪秒级时间戳：{:?}", timestamp.as_millis());
    // 创建时间间隔
    let duration = Duration::from_secs(5);
    println!("时间间隔：{:?}", duration);

    // 测量代码执行耗时
    let start = Instant::now();

    let sum: i64 = (1..=1_000_000).sum();
    println!("计算结果：{}", sum);
    println!("执行耗时：{:?}", start.elapsed());
}
