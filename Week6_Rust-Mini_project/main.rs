// import necessary libraries
use std::collections::HashMap;
// use std::fs::File;
// use std::io::prelude::*;
// use std::io::BufReader;
// use std::path::Path;

fn cosine_similarity(vector1: &HashMap<String, f64>, vector2: &HashMap<String, f64>) -> f64 {
    let mut dot_product = 0.0;
    let mut magnitude1 = 0.0;
    let mut magnitude2 = 0.0;

    for (key, value) in vector1 {
        let value2 = vector2.get(key).unwrap_or(&0.0);
        dot_product += value * value2;
        magnitude1 += value * value;
    }

    for (_, value) in vector2 {
        magnitude2 += value * value;
    }

    dot_product / (magnitude1 * magnitude2).sqrt()
}
fn main() {
    let mut vector1 = HashMap::new();
    vector1.insert("a".to_string(), 1.0);
    vector1.insert("b".to_string(), 2.0);
    vector1.insert("c".to_string(), 3.0);
    vector1.insert("d".to_string(), 4.0);
    vector1.insert("e".to_string(), 5.0);
    let mut vector2 = HashMap::new();
    vector2.insert("a".to_string(), 1.0);
    vector2.insert("b".to_string(), 2.0);
    vector2.insert("c".to_string(), 3.0);
    vector2.insert("d".to_string(), 4.0);
    vector2.insert("e".to_string(), 5.0);
    // get the cosine similarity
    let similarity = cosine_similarity(&vector1, &vector2);
    println!("Cosine similarity: {}", similarity); // should be 1.0
    println!("Hello, world!");
}
