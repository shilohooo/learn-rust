use std::fmt::Display;

struct City {
    // 名称
    name: &'static str,
    // 经度
    lat: f32,
    // 纬度
    iat: f32,
}

impl Display for City {
    // f 是一个缓冲区（buffer），此方法必须将格式化后的字符串写入其中
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let iat_c = if self.iat >= 0.0 { 'E' } else { 'W' };
        // write! 和 format! 类似，但它会将格式化后的字符串写入指定的缓冲区
        write!(
            f,
            "{}: {:3}°{} {:3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.iat.abs(),
            iat_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RGB ({}, {}, {}) 0x{:02X}{:02X}{:02X}",
            self.red,
            self.green,
            self.blue,
            self.red,
            self.green,
            self.blue,
        )
    }
}

fn main() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            iat: -6.259772,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            iat: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            iat: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", city)
    }

    for color in [
        Color {
            red: 255,
            green: 255,
            blue: 255,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
        Color {
            red: 128,
            green: 232,
            blue: 90,
        },
    ]
    .iter()
    {
        // println!("{:?}", color);
        println!("{}", color);
    }
}
