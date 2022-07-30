extern crate getopts;
use core::panic;
use getopts::Options;
use std::env;

fn do_work(inp: &str, out: Option<String>) {
    println!("{}", inp);
    match out.unwrap().as_str() {
        "fahrenheit" => println!("華氏"),
        "celsius" => println!("摂氏"),
        _ => panic!("デフォルト値は華氏"),
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("t", "type", "set degrees type of input value. 'fahrenheit' or 'celsius'", "TYPE");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let t = matches.opt_str("t");
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    do_work(&input, t);
}
