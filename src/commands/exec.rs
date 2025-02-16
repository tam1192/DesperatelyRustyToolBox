use std::process::Stdio;

pub fn run(args: &[&str]) {
    let mut data = args.iter();
    if let Some(cmd) = data.next() {
        let mut cmd = std::process::Command::new(cmd);
        for args in data {
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