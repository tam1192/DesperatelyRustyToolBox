use chrono::Local;

use super::Args;

pub fn run(args: Args) {
    let _ = args;
    let time = Local::now().format("%H:%M:%S").to_string();
    println!("{}", time);
}