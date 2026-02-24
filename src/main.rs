use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use serde_json::{Value, json};
use std::{env, process};

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
            "model":"anthropic/claude-haiku-4.5"
        }))
        .await?;


    // print!("answer: {:?} ",response);
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // eprintln!("Logs from your program will appear here!");

    // TODO: Uncomment the lines below to pass the first stage
    if let Some(content) = response["choices"][0]["message"]["content"].as_str() {
        println!("{}", content);
    }

    Ok(())
}
