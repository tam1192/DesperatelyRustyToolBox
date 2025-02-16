use chrono::Local;

use super::Args;

pub fn run(mut args: Args) {
    let time = Local::now().format("%H:%M:%S").to_string();
    println!("{}", time);
}