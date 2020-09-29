use std::env;

fn main() {
    let default = String::from("y");
    let args: Vec<String> = env::args().collect();
    let s = if args.len() > 1 { &args[1] } else { &default };
    loop {
        println!("{}", s);
    }
}
