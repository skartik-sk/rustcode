use async_openai::{Client, config::OpenAIConfig};
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
        "z-ai/glm-4.5-air:free"
    } else {
        "anthropic/claude-haiku-4.5"
    };
    //print!(model.to_string());
    #[allow(unused_variables)]
    let response: Value = client
        .chat()
        .create_byot(json!({
                    "messages": [
                        {
                            "role": "user",
                            "content":args.prompt
                        }
                    ],
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
        .await?;

    // print!("answer: {:?} ",response);
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // eprintln!("Logs from your program will appear here!");

    // TODO: Uncomment the lines below to pass the first stage

        if let Some(arguments) =
            response["choices"][0]["message"]["tool_calls"][0]["function"]["arguments"].as_str()
        {
            if let Some(name) =
                response["choices"][0]["message"]["tool_calls"][0]["function"]["name"].as_str()
            {
                // print!("{:?}{:?}", arguments, name);
                if name == "Read" {
                    //"{\"file_path\": \"/path/to/file.txt\"}"
                    if let Some(start) = arguments.find(r#""file_path": ""#) {
                        let start_of_path = start + r#""file_path": ""#.len();
                        if let Some(end) = arguments[start_of_path..].find('"') {
                            let file_path = &arguments[start_of_path..start_of_path + end];
                            // println!("File path: {}", file_path); // Outputs: /path/to/file.txt
                            let file_content = fs::read_to_string(file_path).unwrap();
                            print!("{}", file_content);
                        }
                    }
                }
            }
        }
        else if  let Some(content) = response["choices"][0]["message"]["content"].as_str() {print!("{}", content);
     
    }

    Ok(())
}
