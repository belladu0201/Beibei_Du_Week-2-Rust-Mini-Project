## Week5_Rust-Mini_project - Word Counter


### The Main Code
```
// needed packages
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

```

### Sample of  `test.txt`
![](https://github.com/belladu0201/Beibei_Du_Weekly-Rust-Mini-Project/blob/main/Week5_Rust-Mini_project/Screen%20Shot%202023-02-20%20at%208.13.31%20PM.png)

### Generated Output
![](https://github.com/belladu0201/Beibei_Du_Weekly-Rust-Mini-Project/blob/main/Week5_Rust-Mini_project/Screen%20Shot%202023-02-20%20at%208.13.26%20PM.png)

## Takeaway
1. Important to make sure that the set up is ready. 
2. Put the files in the same directory.
3. Use resources wisely.
4. Use Print Statements as a way to debug.
5. Read documentations and tutorials: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
