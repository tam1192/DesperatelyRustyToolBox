use super::Args;

pub fn run(mut args: Args) {
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