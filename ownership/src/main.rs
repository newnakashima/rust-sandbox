fn main() {
    let s = "hello";
    println!("{}", s);

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let x = String::from("one");
    let y = x;
    y.to_owned().push_str("two");
    println!("y = {}", y);
}
