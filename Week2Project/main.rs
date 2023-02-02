// The following code is a simple "Hello, world!" program in Rust.
// Path: ids721-proj1/src/main.rs
// fn main() {
//     println!("Hello, world!");
// }

// The following function tells you the length of a string.

fn main() {
    println!("Hello, world!");
    let mut input = String::new();
    println!("Please input a number/string/combinations:");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    let mut count = 0;
    for _ in input.chars() {
        count += 1;
    }
    println!("The number of digits in the input is: {}", count);
}
