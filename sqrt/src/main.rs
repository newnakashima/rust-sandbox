use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num: f64 = args[1].parse().unwrap_or(0.0);

    let mut count = 100;
    let mut larger_number = num;
    let mut smaller_number = 0.0;
    let mut root = 0.0;

    while count != 0 {
        let avg: f64 = (larger_number + smaller_number) / 2.0;
        let powered: f64 = avg * avg;
        if powered > num {
            larger_number = avg;
            root = avg;
        } else if powered < num {
            smaller_number = (smaller_number + avg) / 2.0;
            root = avg;
        } else {
            root = avg;
        }
        count = count - 1;
    }

    let rounded = root.round();
    if rounded * rounded == num {
        root = rounded;
    }

    println!("root = {}", root);
}
