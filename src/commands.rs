use std::str::Split;

mod exec;

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
        _ if cmd.len() > 0 => {
            eprintln!("{} is bad command", cmd);
            false
        }
        _ => false
    }
}