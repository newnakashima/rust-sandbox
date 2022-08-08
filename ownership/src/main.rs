fn main() {
    let s = "hello";
    println!("{}", s);

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");

    let s = takes_ownership(s);

    println!("{}", s);

    let x = 5;

    makes_copy(x);

    println!("{}", x);

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);

    println!("{}", s);

    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);

    some_string
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
