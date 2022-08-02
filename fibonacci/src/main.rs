extern crate getopts;
use core::panic;
use getopts::Options;
use std::env;

fn fibonacci(n: u32) -> u32 {
    return match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 2) + fibonacci(n - 1),
    };
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} NUMBER", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
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

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let n: u32 = input.parse().expect("argument must be 32 bit unsigned integer");

    println!("{}", fibonacci(n));
}
