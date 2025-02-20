use std::str::Split;

use strum_macros::{Display, EnumString};

mod exec;
mod say;
mod time;
mod ping;
mod counter;
mod help;

type Args<'a> = Split<'a, &'a str>;

#[derive(Debug, EnumString, Display)]
pub enum Command<'a> {
    #[strum(serialize = "/ping")]
    Ping,
    #[strum(serialize = "/stop")]
    Stop,
    #[strum(serialize = "/exec")]
    Exec(&'a str, &'a [&'a str]),
    #[strum(serialize = "/time")]
    Time,
    #[strum(serialize = "/say")]
    Say(&'a str),
    #[strum(serialize = "/counter")]
    Counter(u32),
    #[strum(serialize = "/help")]
    Help,
}

/// コマンドランナー
/// 戻り値はプログラムを終了させるかどうか
pub fn run(cmd: &str, args: Args) -> bool {
    let is_exit = match cmd {
        "/ping" => {
            ping::run(args);
            false
        },
        "/stop" => {
            true
        },
        "/exec" => {
            exec::run(args);
            false
        },
        "/time" => {
            time::run(args);
            false
        }
        "/say" => {
            say::run(args);
            false
        },
        "/counter" => {
            counter::run(args);
            false
        },
        "/help" => {
            help::run(args);
            false
        },
        _ if cmd.len() > 0 => {
            eprintln!("{} is bad command", cmd);
            false
        }
        _ => false
    };
    is_exit
}