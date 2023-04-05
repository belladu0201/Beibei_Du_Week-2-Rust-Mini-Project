use std::io;

fn main() {
    println!("Welcome to Chatbot!");

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let response = match input.trim() {
            "hello" => "Hi there!",
            "how are you?" => "I'm good, thank you for asking.",
            "what's your name?" => "My name is Chatbot.",
            "hi" => "Hello!",
            "What date is it?" => "It's April 6, 2022",
            "What time is it?" => "It's 11:00 AM",
            "What is the weather like?" => "It's sunny",
            "What is the temperature?" => "It's 70 degrees",
            "What class is this?" => "IDS 721",
            "What is the name of the instructor?" => "Noah Gift",
            "bye" => {
                println!("Goodbye!");
                break;
            }
            _ => "I'm sorry, I don't understand what you're saying.",
        };

        println!("{}", response);
    }
}

