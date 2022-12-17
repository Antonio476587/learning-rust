use std::fs::File;
use::std::io::{ErrorKind,  Read, self};

fn main() {
    // This enum is in the prelude like Option
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    
    let greeting_file_result = File::open("hello.txt").expect("Waos");

    let username = read_username_from_file();
    
    println!("{}", username.unwrap());
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}