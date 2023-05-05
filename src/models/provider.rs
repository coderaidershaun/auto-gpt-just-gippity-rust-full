use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Clone)]
pub struct Message {
  pub role: String,
  pub content: String,
}


#[derive(Debug, Serialize)]
pub struct ChatCompletion {
  pub model: String,
  pub messages: Vec<Message>,
}


#[derive(Debug, Deserialize)]
pub struct ApiResponse {
  pub choices: Vec<ApiChoice>,
}


#[derive(Debug, Deserialize)]
pub struct ApiChoice {
  pub message: ApiMessage,
}


#[derive(Debug, Deserialize)]
pub struct ApiMessage {
  pub content: String,
}


#[derive(Deserialize, Debug)]
pub struct Task {
  pub task_number: u32,
  pub function_number: u32,
  pub task_description: String,
  pub is_machine: bool,
}
