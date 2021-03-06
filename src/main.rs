use std::env;

fn main() {
    const BUF_SIZE:usize = 1024*8;

    let default = String::from("y");
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { &default };
    let input_size = input.len() + 1;

    let mut output = input.clone();
    output.push('\n');

    for _x in 0..((BUF_SIZE-input_size)/input_size) {
        output.push_str(input);
        output.push('\n');
    }

    loop {
        print!("{}", output);
    }
}
