use reqwest::blocking::Client;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let api_key = "tobefilled"; // Replace with actual API key

    let response = client
        .post("https://api.openai.com/v1/engines/davinci-codex/completions")
        .bearer_auth(api_key)
        .json(&json!({
            "prompt": "What is the date of today?",
            "temperature": 0.5,
            "max_tokens": 50,
            "stop": ["\n"]
        }))
        .send()?;

    let response_text = response.text()?;
    println!("{}", response_text);

    Ok(())
}
