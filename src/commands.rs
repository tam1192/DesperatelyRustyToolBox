mod exec;

fn run(cmd: &str, args: &[&str]) {
    match cmd {
        "/exec" => exec::run(args),
        _ if cmd.len() > 0 => {
            eprintln!("{} is bad command", cmd);
        }
        _ => {}
    }
}