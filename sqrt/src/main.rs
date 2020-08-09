use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num: f64 = args[1].parse().unwrap_or(0.0);

    println!("mysqrt = {}", mysqrt(num));
}

fn mysqrt(num: f64) -> f64 {
    let mut s: f64;
    let mut last: f64 = 0.0;

    if num > 0.0 {
        if num > 1.0 {
            s = num;
        } else {
            s = 1.0;
        }

        loop {
            last = s;
            s = (num / s + s) / 2.0;
            if s >= last {
                break;
            }
        }
    }

    return last;
}
