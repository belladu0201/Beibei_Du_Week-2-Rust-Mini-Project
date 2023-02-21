## Week5_Rust-Mini_project - Word Counter


### The Main Code
```
use std::fs::File;
use std::io::prelude::*;

fn word_count(file_name: &str) -> Result<usize, std::io::Error> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.split_whitespace().count())
}

fn main() {
    match word_count("test.txt") {
        Ok(n) => println!("Word count: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    //let result = word_count("test.txt");
    let result = word_count("data/test.txt");
    println!("Word count: {}", result.unwrap());
}

```
