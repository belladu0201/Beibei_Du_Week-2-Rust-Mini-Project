# Week8_Rust-Mini_project (due 03/22)
## This is a simple Rust-based client for calling the ChatGPT API provided by OpenAI. The code is designed to send a prompt to the API and receive a response with the completion. Please note that for personal privacy, the API key is not included in the source code.

## Code Overview
```
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
```
## Dependencies
- Reqwest: A high-level HTTP client library for making API calls.
- Serde JSON: A library for JSON serialization and deserialization.

## Usage
1. Replace the placeholder value of `api_key` with your personal API key.
```
let api_key = "tobefilled"; // Replace with actual API key
```
2. Update the `prompt`, `temperature`, `max_tokens`, and `stop` values as needed.
```
.json(&json!({
    "prompt": "What is the date of today?",
    "temperature": 0.5,
    "max_tokens": 50,
    "stop": ["\n"]
}))
```
## Output
The program will print the response from the ChatGPT API to the console.
### Example Response
```
{
  "id": "exampleid",
  "object": "text.completion",
  "created": 1677649420,
  "model": "davinci-codex",
  "usage": {
    "prompt_tokens": 6,
    "completion_tokens": 12,
    "total_tokens": 18
  },
  "choices": [
    {
      "text": "The date of today is March 17, 2023.",
      "index": 0,
      "logprobs": null,
      "finish_reason": "stop"
    }
  ]
}
```
