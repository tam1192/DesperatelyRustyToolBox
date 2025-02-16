use std::str::Split;

mod exec;
mod say;

type Args<'a> = Split<'a, &'a str>;

pub fn run(cmd: &str, args: Args) -> bool {
    match cmd {
        "/stop" => {
            true
        }
        "/exec" => {
            exec::run(args);
            false
        },
        "/say" => {
            say::run(args);
            false
        },
        _ if cmd.len() > 0 => {
            eprintln!("{} is bad command", cmd);
            false
        }
        _ => false
    }
}