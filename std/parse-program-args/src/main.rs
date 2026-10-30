use std::env;

fn increase(num: i32) {
    println!("{}", num + 1);
}

fn decrease(num: i32) {
    println!("{}", num - 1);
}

fn help() {
    println!(
        "usage:\nmatch_args <string>\n\tcheck whether given string is the answer.\nmatch_args {{increase|decrease}} <integer>\n\tIncrease or decrease given integer by one."
    );
}

/// # 参数解析
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("My name is 'match_arg'. Try passing some arguments!")
        }
        2 => match args[1].parse() {
            Ok(42) => println!("This is the answer:)"),
            _ => println!("This is not the answer:("),
        },
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            let number: i32 = match num.parse() {
                Ok(n) => n,
                Err(e) => {
                    println!("error: second argument not an integer");
                    help();
                    return;
                }
            };
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => increase(number),
                _ => {
                    println!("error: invalid command!");
                    help()
                }
            }
        }
        _ => help(),
    }
}
