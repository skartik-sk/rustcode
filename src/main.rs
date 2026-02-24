use async_openai::{Client, config::OpenAIConfig, error::OpenAIError};
use clap::Parser;
use serde_json::{Value, json};
use std::{env, fs, process, ptr::null};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let base_url = env::var("OPENROUTER_BASE_URL")
        .unwrap_or_else(|_| "https://api.z.ai/api/coding/paas/v4/".to_string());

    let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
        eprintln!("OPENROUTER_API_KEY is not set");
        process::exit(1);
    });

    // print!("key --= {:?}",api_key);
    // let api_key ="abc";

    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);

    let client = Client::with_config(config);
    let is_local = std::env::var("local")
        .map(|local| local == "true")
        .unwrap_or(false);

    let model = if is_local {
        "glm-4.7"
    } else {
        "anthropic/claude-haiku-4.5"
    };

    

    // print!("answer: {:?} ",response);
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // eprintln!("Logs from your program will appear here!");

    let mut messages: Vec<Value> = vec![json!({"role": "user", "content": args.prompt})];
        let mut response = request(&client, &messages).await?;
        let mut response_message = &response["choices"][0]["message"];
        messages.push(response_message.clone());

        while let Some(tool_calls) = response_message["tool_calls"].as_array() {
            let tool_call = &tool_calls[0];
            let name = tool_call["function"]["name"].as_str().unwrap();
            let id = tool_call["id"].as_str().unwrap();
            let args: Value =
                serde_json::from_str(tool_call["function"]["arguments"].as_str().unwrap())?;
            let mut content = "".to_string();

            if name == "Read" {
                let file_path = args["file_path"].as_str().unwrap();
                content = fs::read_to_string(file_path)?;
            }

            messages.push(json!({
                "role": "tool",
                "tool_call_id": id,
                "content": content
            }));

            response = request(&client, &messages).await?;
            response_message = &response["choices"][0]["message"];
            messages.push(response_message.clone());
        }
        if let Some(content) = response_message["content"].as_str() {
            println!("{}", content);
        }

    Ok(())
    
}


pub async fn request (client :&Client<OpenAIConfig>, message:&Vec<Value>)->Result<Value,OpenAIError>{
    
    client
        .chat()
        .create_byot(json!({
                    "messages": message,
                    "model":"anthropic/claude-haiku-4.5",
                    "tools":[{
          "type": "function",
          "function": {
            "name": "Read",
            "description": "Read and return the contents of a file",
            "parameters": {
              "type": "object",
              "properties": {
                "file_path": {
                  "type": "string",
                  "description": "The path to the file to read"
                }
              },
              "required": ["file_path"]
            }
          }
        }]
                }))
        .await
}
