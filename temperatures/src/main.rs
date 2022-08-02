extern crate getopts;
use core::panic;
use getopts::Options;
use std::env;

fn convert(input: &str, out: Option<String>) -> f32 {
    let degrees: f32 = input.parse().expect("degrees are invalid");
    match out.unwrap().as_str() {
        "fahrenheit" => fahrenheit2celsius(degrees),
        "celsius" => celsius2fahrenheit(degrees),
        _ => panic!("invalid type"),
    }
}

fn fahrenheit2celsius(input: f32) -> f32 {
    return (input - 32.0) * 5.0 / 9.0;
}

fn celsius2fahrenheit(input: f32) -> f32 {
    return input * 9.0 / 5.0 + 32.0
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
    opts.optflag("m", "minus", "make input value minus");

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

    let minus = matches.opt_present("m");

    let t = matches.opt_str("t");
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    let input = if minus { format!("-{}", input) } else { input };

    let degrees = convert(&input, t);

    println!("{}", degrees);
}
