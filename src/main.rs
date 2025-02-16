use commands::run;
use std::sync::mpsc;
use std::{io, mem, thread, time};

mod commands;

fn main() {
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

        let mut data: std::str::Split<'_, &str> = data.split(" ");
        if let Some(cmd) = data.next() {
            if run(cmd, data) {
                break;
            }
        }
    });

    output.join().unwrap();
}
