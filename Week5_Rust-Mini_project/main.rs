use std::fs::File;
use std::io::prelude::*;

fn word_count(file_name: &str) -> Result<usize, std::io::Error> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.split_whitespace().count())
}

fn main() {
    println!("Hello! Start of the program");
    let result = word_count("test.txt");
    println!("Word count: {}", result.unwrap());
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("End of the program.")
}


