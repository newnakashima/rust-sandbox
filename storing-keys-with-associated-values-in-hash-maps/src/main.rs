use std::{collections::HashMap, io, process::exit};

// use storing_keys_with_associated_values_in_hash_maps::{median::median, text_interface::service};
use storing_keys_with_associated_values_in_hash_maps::text_interface::*;

fn main() {
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // println!("{:?}", scores);

    // let team_name = String::from("Blue");
    // // let score = scores.get(&team_name).copied().unwrap_or(0);
    // let score = scores.get(&team_name);
    // if let Some(s) = score {
    //     println!("{}", s);
    // }

    // println!("{:?}", scores);

    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);

    // // println!("{}", field_name);
    // // println!("{}", field_value);

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 25);
    // println!("{:?}", scores);


    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // println!("{:?}", scores);

    // let text = "hello world wonderful world";

    // let mut map = HashMap::new();

    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }
    func();
}
fn func() {
    // println!("{:?}", map);
    let mut map: HashMap<String, &mut Vec<String>>  = HashMap::new();
    loop {
        let mut default: Vec<String> = Vec::new();
        let mut buffer = String::new();
        let stdin = io::stdin();
        let _ = stdin.read_line(&mut buffer);

        let mut words = parse_words(&buffer);
        let mut response = String::from("");
        if words.len() < 2 {
            println!("1 arguments are invalid");
            continue;
        }
        let operation = parse_operation(words.get(0), words.get(1));

        response = match operation {
            Operation::Add => {
                if words.len() < 4 {
                    String::from("2 arguments are invalid")
                // } else {
                    // add_employee(&mut map, &mut words, &mut default)
                } else {
                    "hoge".to_string()
                }
            },
            Operation::ListCompany => list_company(),
            Operation::ListDepartment => list_department(),
            Operation::Invalid => String::from("3 arguments are invalid")
        };

        println!("{}", response);
    }
}
