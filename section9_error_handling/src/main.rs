use std::{
    fs::{File, self},
    io::{self, ErrorKind, Read},
};

fn main() {
    // panic!("Crashed"); - this will immediately stop execution
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_err => panic!("Problem opening the file: {:?}", other_err),
        },
    };

    let file = File::open("hello.txt").unwrap(); // unwrap will return the file if Ok and panic otherwise

    // expect lets us choose the panic! error message. we use it in the same way as unwrap
    // this is the preferred way of doing this operation
    let file_expect =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    let username = read_username_from_file().unwrap();
    println!("Hi there {}", username);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// propagating with ?
fn another_read_username() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn even_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn even_more_shorter() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
