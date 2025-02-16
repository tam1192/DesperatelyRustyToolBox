use super::Args;

pub fn run(args: Args) {
    let say = args.collect::<Vec<&str>>().join(" ");
    println!("say: {}", say);
}