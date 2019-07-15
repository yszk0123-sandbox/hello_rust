use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    let f = File::open("hello.txt");

    match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Tries to create file but there was a problem: {:?}", e),
        },
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };

    let content = read_content_from_file().expect("Failed to reading the content");
    println!("content: {}", content);
}

fn read_content_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
