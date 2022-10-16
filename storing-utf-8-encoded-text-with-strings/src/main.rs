use std::ops::Add;

fn main() {
    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    let mut v = vec!["hoge", "fuga"];

    let mut vp = vec!["piyo"];

    vp.append(&mut v);

    vp.iter().enumerate().for_each(|(_, s)| println!("{}", s));

    let mut s = String::from("foo");
    s.push_str("bar");

    println!("{}", s);

    let mut s = String::from("lo");
    s.push('l');

    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    println!("{}", s2);

    println!("{}", s3);

    let s4 = s3.add("piyo");
    println!("{}", s4);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let s1 = String::from("Здравствуйте");
    let h = s1.as_bytes()[0];
    println!("{}", h);

    let s = &s1[0..4];
    println!("{}", s);

    for c in s1.chars() {
        println!("{}", c);
    }
}
