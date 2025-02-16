use super::Args;

pub fn run(mut args: Args) {
    let say = args.collect::<Vec<&str>>().join(" ");
    println!("say: {}", say);
}