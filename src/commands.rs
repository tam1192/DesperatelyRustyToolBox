use std::str::Split;

mod exec;

type Args<'a> = Split<'a, &'a str>;

fn run(cmd: &str, args: Args) {
    match cmd {
        "/exec" => exec::run(args),
        _ if cmd.len() > 0 => {
            eprintln!("{} is bad command", cmd);
        }
        _ => {}
    }
}