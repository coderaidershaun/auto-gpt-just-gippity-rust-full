use crate::models::provider::{ChatCompletion, ApiResponse, Message};
use reqwest::Client;
use std::env;
use dotenv::dotenv;


// Call Large Language Model (i.e. GPT-4)
pub async fn call_ai_llm(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error>> {
    dotenv().ok();

    // Extract API Key information
    let api_key: String = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY must be set");
    let api_org: String = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG must be set");

    // Confirm API endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // Create headers
    let mut headers: reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
    headers.insert("authorization", reqwest::header::HeaderValue::from_str(&format!("Bearer {}", api_key))?);
    headers.insert("OpenAI-Organization", reqwest::header::HeaderValue::from_str(api_org.as_str())?);
    let client: Client = Client::builder()
        .default_headers(headers)
        .build()?;

    
    // Structure input chat
    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-4".to_string(),
        messages: messages
    };
    
    // // Troubleshooting: Show raw response if issue
    // let res_raw = client
    // .post(url)
    // .json(&chat_completion)
    // .send()
    // .await?;
    // println!("Raw response: {:?}", res_raw.text().await?);

    // Send API Request
    let res: ApiResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await?
        .json()
        .await?;

    // Extract Response
    let response_text: String = res.choices[0].message.content.clone();
    Ok(response_text)
}
