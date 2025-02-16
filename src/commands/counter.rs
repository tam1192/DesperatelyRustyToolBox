use super::Args;

pub fn run(args: Args) {
    if let Some(counter) = args.next() {
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