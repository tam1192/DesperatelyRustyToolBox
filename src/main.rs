use chrono::Local;
use std::process::Stdio;
use std::sync::mpsc;
use std::thread::sleep;
use std::{io, mem, thread, time};

mod commands;

fn main() {
    let start_time = time::Instant::now();

    let (tx, rx) = mpsc::channel();

    let _ = thread::spawn(move || {
        let stdin = io::stdin();
        let mut buffer = String::new();
        loop {
            stdin.read_line(&mut buffer).unwrap();
            let mut buf = String::new();
            mem::swap(&mut buffer, &mut buf);
            let _ = tx.send(buf);
        }
    });

    let output = thread::spawn(move || loop {
        // 入力を受け取る
        let data = rx.recv().unwrap();
        let data = if let Some(len) = data.find('\n') {
            &data[..len]
        } else {
            data.as_str()
        };

        let mut data = data.split(" ");
        match data.next() {
            Some("/help") => {
                let help = r#"
/stop
/ping
/say <str>
/time
/exec <cmd> <args...>
/counter <count>
                "#;
                println!("{}", help);
            }
            Some("/stop") => {
                println!("good bye");
                println!("{:?}", start_time.elapsed());
                break;
            }
            Some("/ping") => {
                println!("pong!!");
                println!("{:?}", start_time.elapsed());
            }
            Some("/say") => {
                let say = data.collect::<Vec<&str>>().join(" ");
                println!("say: {}", say);

            }
            Some("/time") => {
                let time = Local::now().format("%H:%M:%S").to_string();
                println!("{}", time)
            }
            Some("/counter") => {
                if let Some(counter) = data.next() {
                    if let Ok(counter) = counter.parse::<u64>(){
                        for count in 0..counter {
                            sleep(time::Duration::from_secs(1));
                            let count = counter - count;
                            println!("{}", count);
                        }
                        println!("done!");
                    } else {
                        eprintln!("counter should be an integer");
                    }
                } else {
                    eprintln!("counter should be an integer");
                }
            }
            Some("/exec") => {
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
            Some(data) if data.len() > 0 => {
                eprintln!("{} is bad command", data);
            }
            _ => {}
        }
    });

    output.join().unwrap();
}
