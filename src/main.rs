#[macro_use]
extern crate serde_json;
use reqwest;
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {
    // Support tweak FILE PROMPT -i
    // Support tweak FILE PROMPT
    // Support tweak -i FILE PROMPT
    
    // Get the command-line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 || args.len() > 4 {
        eprintln!("Usage: tweak FILE PROMPT [-i]");
        std::process::exit(1);
    }

    // Read the file
    let filename = &args[1];
    let mut file = File::open(filename).unwrap_or_else(|_| {
        eprintln!("Error: could not open file '{}'", filename);
        std::process::exit(1);
    });
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap_or_else(|_| {
        eprintln!("Error: could not read file '{}'", filename);
        std::process::exit(1);
    });

    // Concatenate the prompt and the file contents
    let prompt = &args[2];
    let input = format!("{}:\n\n{}", prompt, contents);

    // Send the input to the ChatGPT API and print the response to stdout
    let response = chat_gpt_api(input);

    // If a -i flag was providded, overwrite the file with the new contents
    if args.len() == 4 && args[3] == "-i" {
        let mut file = File::create(filename).unwrap_or_else(|_| {
            eprintln!("Error: could not open file '{}'", filename);
            std::process::exit(1);
        });
        file.write_all(response.as_bytes()).unwrap_or_else(|_| {
            eprintln!("Error: could not write to file '{}'", filename);
            std::process::exit(1);
        });
    }
    // otherwise print the response to stdout
    else {
        println!("{}", response);
    }
}

fn chat_gpt_api(content: String) -> String {
    // Send the input to the ChatGPT API using an HTTP POST request
    let client = reqwest::blocking::Client::new();
    // get the OpenAI API key from the environment
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        eprintln!("Error: OPENAI_API_KEY environment variable not set");
        std::process::exit(1);
    });
    let bearer_token = format!("Bearer {}", api_key);
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", bearer_token)
        .json(&json!({
            "model": "gpt-4",
            "messages": [
                {
                    "role": "user",
                    "content": content
                }
            ]
        }))
        .send()
        .unwrap_or_else(|_| {
            eprintln!("Error: could not send request to ChatGPT API");
            std::process::exit(1);
        });

    // Parse the response as JSON and extract the generated text
    let response_json: serde_json::Value = response.json().unwrap_or_else(|_| {
        eprintln!("Error: could not parse response from ChatGPT API");
        std::process::exit(1);
    });
    let choices = &response_json["choices"];
    let text = choices[0]["message"]["content"].as_str().unwrap_or_else(|| {
        eprintln!("Error: no text found in response from ChatGPT API");
        std::process::exit(1);
    });

    // Return the generated text
    String::from(text.trim())
}
