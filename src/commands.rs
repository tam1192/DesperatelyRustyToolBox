use std::str::Split;

mod exec;
mod say;
mod time;
mod ping;
mod counter;
mod help;

type Args<'a> = Split<'a, &'a str>;

pub fn run(cmd: &str, args: Args) -> bool {
    match cmd {
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
    }
}