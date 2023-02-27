## Week7_Rust-Mini_project


Following the completion of last week's Rust mini project, which involved calculating the cosine similarity score between two vectors, I have decided to continue exploring the potential of Rust programming language by defining an algorithm to perform the TF-IDF technique on the Coursera course description text file.

The algorithm for performing the TF-IDF technique on the Coursera course description text file will consist of three functions: `word_count`, `word_frequency`, and `tf_idf`.

The first function, word_count, will be responsible for loading the text file and preprocessing the text data. This involves counting the number of occurrences of each word in the text data. The output of this function will be a dictionary where each key represents a unique word in the text data and the corresponding value is the number of times that word appears in the text data.

The second function, word_frequency, will convert the preprocessed text data into a numerical form using the TF-IDF technique. This function will calculate the frequency of each word in the text data by dividing the number of times the word appears in the text data by the total number of words in the text data. The output of this function will be a dictionary where each key represents a unique word in the text data and the corresponding value is the frequency of that word in the text data.

The final function, tf_idf, will use the TF-IDF and IDF scores to calculate the final similarity score between the text data and a query string. This function will first calculate the TF-IDF score for each term in the text data by multiplying the frequency of the term by the IDF score for that term. The IDF score for each term is calculated by dividing the total number of documents by the number of documents containing that term and taking the logarithm of the result. The output of this function will be a dictionary where each key represents a unique word in the text data and the corresponding value is the TF-IDF score for that word.

#### The mini-Rust project will involve the following steps:

1. Load the Coursera course description text file into the program.
2. Convert the preprocessed text data into a numerical form using the TF-IDF technique.
3. Calculate the TF-IDF score for each term in the text data.

## How to use
```
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
```
The txt file looks like below:<img width="1545" alt="Screen Shot 2023-02-26 at 9 07 31 PM" src="https://user-images.githubusercontent.com/60382493/221455436-f16841c6-0a4b-4f5c-872a-05574f04e81f.png">
#### The output looks like this:
<img width="1541" alt="Screen Shot 2023-02-26 at 9 08 25 PM" src="https://user-images.githubusercontent.com/60382493/221455541-86859760-bf8a-4719-b962-b5faf901607c.png">

### Discussion & Limitation
One of the limitations of the current implementation of the word_count function is that it does not remove stop words and punctuation, and it does not perform stemming or lemmatization on the remaining words. This can lead to inaccurate word counts and frequency calculations, as well as poor similarity scores.

Stop words are common words such as "the", "and", and "is" that do not add much meaning to the text data. By removing stop words, the algorithm can focus on the more important words that are relevant to the similarity calculation. Similarly, removing punctuation can help to ensure that the words are correctly counted and frequency calculations are accurate.

Stemming and lemmatization are techniques for reducing words to their base form, such as converting "running" to "run". This helps to reduce the number of unique words in the text data, which can improve the accuracy of the similarity calculation by reducing the impact of minor variations in the text data.

In future improvements to the algorithm, implementing these text preprocessing techniques can help to improve the accuracy and efficiency of the similarity score calculation. This can be achieved by using external Rust libraries that provide pre-built functions for text preprocessing, or by implementing custom functions within the algorithm itself.

