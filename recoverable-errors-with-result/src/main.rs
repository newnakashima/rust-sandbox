use std::{fs::{File, self}, io::{ErrorKind, Error, Read}, fmt::Debug};
use std::error::Error as StdError;

fn main() -> Result<(), Box<dyn StdError>> {
    let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // let greeting_fil = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    // let greeting_file = File::open("hello.txt").unwrap();

    // let greeting_file = File::open("hello.txt")
    //     .expect("hello.txt should be included in this project");

    let greeting_file = File::open("hello.txt")?;

    Ok(())

}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn read_username_from_file() -> Result<String, Error> {
    // let username_file_result = File::open("hello.txt");

    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut username = String::new();

    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // =============================================

    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    // =============================================

    // let mut username = String::new();

    // File::open("hello.txt")?.read_to_string(&mut username)?;

    // Ok(username)

    // =============================================

    fs::read_to_string("hello.txt")
}
