use std::{io::stdin, collections::HashMap};

use practice_mutable_and_lifetime::department::{validate_words, add_employee, DepartmentList};

fn main() {
    let s = String::from("hoge");

    let s1 = &s;
    let s2 = &s;

    println!("{}, {}", s1, s2);

    let mut store: DepartmentList = HashMap::new();

    loop {
        let mut item = String::new();
        let stdin = stdin();
        if let Err(error) = stdin.read_line(&mut item) {
            println!("Something is going wrong! {}", error);
            println!("{:?}", store);
            continue;
        }

        let words = match validate_words(&item) {
            Ok(list) => list,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        };

        let department_name = match words.get(0) {
            Some(name) => *name,
            None => {
                println!("Something is going wrong!");
                println!("{:?}", store);
                continue;
            }
        };


        let employee_name = match words.get(1) {
            Some(name) => *name,
            None => {
                println!("Something is going wrong!");
                println!("{:?}", store);
                continue;
            }
        };

        add_employee(&mut store, department_name, employee_name);

        println!("{:?}", store);
    }
}
