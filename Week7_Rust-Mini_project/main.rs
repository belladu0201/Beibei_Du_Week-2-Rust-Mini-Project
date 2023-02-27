// implement tf-idf
// 1. read in a file
// 2. count the number of words in the file
// 3. count the number of times each word appears in the file
// 4. calculate the tf-idf for each word
// 5. print out the top 10 words with the highest tf-idf
//
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
// use std::path::Path;

// define a function to count the number of words in a file
fn word_count(file_name: &str) -> Result<usize, std::io::Error> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.split_whitespace().count())
}

// define a function to count the number of times each word appears in a file
fn word_frequency(file_name: &str) -> Result<HashMap<String, usize>, std::io::Error> {
    let mut word_freq = HashMap::new();
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        for word in line.split_whitespace() {
            let count = word_freq.entry(word.to_string()).or_insert(0);
            *count += 1;
        }
    }
    Ok(word_freq)
}
// define a function to calculate the tf-idf for each word
fn tf_idf(file_name: &str) -> Result<HashMap<String, f64>, std::io::Error> {
    let mut tf_idf = HashMap::new();
    let word_freq = word_frequency(file_name)?;
    let total_words = word_count(file_name)?;
    for (word, freq) in word_freq {
        let tf = freq as f64 / total_words as f64;
        let idf = 1.0;
        let tf_idf_value = tf * idf;
        tf_idf.insert(word, tf_idf_value);
    }
    Ok(tf_idf)
}

fn main() {
    println!("Hello! Start of the program");
    // test word_count
    let result = word_count("test.txt");
    println!("Word count: {}", result.unwrap());
    // test word_frequency
    let result = word_frequency("test.txt");
    println!("Word frequency: {:?}", result.unwrap());
    // test tf_idf
    let result = tf_idf("test.txt");
    println!("TF-IDF: {:?}", result.unwrap());
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("End of the program.")
}


