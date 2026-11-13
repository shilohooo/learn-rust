use chrono::{Datelike, Local, NaiveDate, Utc, Weekday};

/// # Chrono 日期时间处理库
fn main() {
    // 获取 UTC 时间
    let utc = Utc::now();
    println!("UTC 时间：{}", utc);
    // 获取本地时间
    let local = Local::now();
    println!("本地时间：{}", local);
    // 日期格式化，参照 strftime / C 语言
    println!("日期格式化 - 年-月-日 时:分:秒：{}", local.format("%F %T"));
    println!("日期格式化 - 年：{}", local.format("%Y"));
    println!("日期格式化 - 月：{}", local.format("%m"));
    println!("日期格式化 - 日：{}", local.format("%d"));
    println!("日期格式化 - 时：{}", local.format("%H"));
    println!("日期格式化 - 分：{}", local.format("%M"));
    println!("日期格式化 - 秒：{}", local.format("%S"));
    println!("日期格式化 - Unix 时间戳：{}", local.format("%s"));
    // Unix 时间戳
    println!("秒级时间戳：{}", utc.timestamp());
    println!("毫秒级时间戳：{}", utc.timestamp_millis());
    // 创建指定日期
    let date = NaiveDate::from_ymd_opt(2026, 11, 13).expect("Invalid date");
    println!("指定日期：{:?}", date);
    println!("指定日期中的年份：{}", date.year());
    println!("指定日期中的月份：{}", date.month());
    println!("指定日期中的几号：{}", date.day());
    println!("指定日期是一周中的星期几：{}", date.weekday());
    // let week = date.week(Weekday::Mon);
    let week = date.week(Weekday::Sun);
    println!("指定日期所在周的第一天：{:?}", week.first_day());
    println!("指定日期所在周的最后一天：{:?}", week.last_day());
    println!(
        "指定日期所在周的闭区间（RangeInclusive）：{:?}",
        week.days()
    );
}
