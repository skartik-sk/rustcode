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
    let message = &response["choices"][0]["message"];
 if let Some(tool_called) = message["tool_calls"].as_array(){
     let mut tool_calls=tool_called;
 
    while tool_calls.len()>0{
        let mut next_msg =vec![];
        for inx in 0..tool_calls.len(){
            
        let tool_call = &tool_calls[inx];
        let tool_call_id = tool_call["id"].as_str().unwrap();
        let name = tool_call["function"]["name"].as_str().unwrap();
        let arguments: Value =
            serde_json::from_str(tool_call["function"]["arguments"].as_str().unwrap())?;

        if name == "Read" {
            let file_path = arguments["file_path"].as_str().unwrap();
            let contents = std::fs::read_to_string(file_path)?;
            //   print!("{}", contents);
        next_msg.push(json!({
          "role": "tool",
          "tool_call_id": tool_call_id,
          "content": contents
        }));
        }
        
        }
        #[allow(unused_variables)]
        let response: Value = client
            .chat()
            .create_byot(json!({
                        "messages": next_msg,
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
        tool_calls = message["tool_calls"].as_array().unwrap();
    } }

    else if let Some(content) = message["content"].as_str() {
        println!("{}", content);
    }
    Ok(())
}
