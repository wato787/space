use std::fs::File;
use std::io::{self, Read};

fn main() {
    let file = File::open("hello.txt");
    let _file = match file {
        Ok(f) => f,
        Err(error) => panic!("Problem opening file: {error}"),
    };

    match read_username_from_file() {
        Ok(name) => println!("username: {name}"),
        Err(err) => eprintln!("error: {err}"),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}
