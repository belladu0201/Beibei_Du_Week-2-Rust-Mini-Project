## Week6_Rust-Mini_project

For this week's Rust mini project, I have decided to implement a similarity score calculation algorithm. The motivation for this project is based on my second project which involves building a recommendation system. In order to determine the similarity between two text messages in the recommendation system, I plan to use some text manipulation techniques, such as Term Frequency-Inverse Document Frequency (TF-IDF). This technique will help me to clean the text data and represent it in a numerical form, after which I will use the numerical data to calculate the similarity score.

The main objective of this project is to build a robust and efficient algorithm that can be used to compute similarity scores between two text messages. To achieve this objective, I will be implementing the following steps in my project 2. 

1. Load the text data into the program and preprocess it by removing stop words and punctuation, and stemming or lemmatizing the remaining words.
2. Convert the preprocessed text data into a numerical form using the TF-IDF technique.
3. Calculate the similarity score between the two text messages using a suitable algorithm, such as cosine similarity or Jaccard similarity.
4. Output the similarity score to the user.

However, I am doing a simpler version in Rust language to calculate the similarity score between two vectors.
The formula is displayed below:

![alt](https://github.com/belladu0201/Beibei_Du_Weekly-Rust-Mini-Project/blob/main/similarity_score_formula.png)

## How to use
```
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
}
```
In the main function, define the two vectors and call the function `cosine_similarity()`. The case that I used here is create a two exactly same vectors and the expected output should be 1. The output that I got matches this result.


<img width="759" alt="Screen Shot 2023-02-26 at 8 21 11 PM" src="https://user-images.githubusercontent.com/60382493/221450861-b72ca1be-a9c1-418e-94a9-7c19c51078aa.png">
