fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}
