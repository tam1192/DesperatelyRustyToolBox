use std::process::Stdio;

use super::Args;

pub fn run(mut args: Args) {
    if let Some(cmd) = args.next() {
        let mut cmd = std::process::Command::new(cmd);
        for args in args {
            cmd.arg(args);
        }
        cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit()).stdin(Stdio::inherit());
        match cmd.spawn() {
            Ok(mut cmd) => {
                cmd.wait().unwrap();
                println!("done")
            },
            Err(e) => eprintln!("exec error: {}", e)
        }
    } else {
        eprintln!("no command given");
    }
}