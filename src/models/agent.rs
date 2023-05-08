use crate::models::provider::Message;
use crate::llm::request::call_ai_llm;
use crate::llm::prompts::prompt_str;


// Set Stage
#[derive(Debug)]
pub enum Stage {
  Filter,
  TaskStructure,
  TaskFunctions,
  TaskInputChecker,
  TextSummarizer,
  Communicator,
  ProgrammingEvaluatePackages,
  ProgrammingEvaluateAPIs,
  ProgrammingEvaluateAPIKeys,
  ProgrammingJunior,
  ProgrammingSenior
}

// Agent Model
#[derive(Debug)]
pub struct Agent {
  pub name: String,
  pub user_name: String,
  pub objective: String,
  pub stage: Stage,
  pub memory: Vec<Message>
}

impl Agent {

  // Creates new Agent instance
  pub fn new(name: String, user_name: String, objective: String, message: Message) -> Self {
    Self {
      name,
      user_name,
      objective,
      stage: Stage::Filter,
      memory: Vec::from([message])
    }
  }

  // Call LLM with update
  pub async fn get_response(&self, message: Option<Message>) -> Result<String, Box<dyn std::error::Error>> {

    // If message provided, use that, otherwise use message history
    let messages: Vec<Message> = match message {
      Some(msg) => Vec::from([msg]),
      None => self.memory.clone()
    };
    call_ai_llm(messages).await
  }

  // Extract AI Function String
  pub fn get_ai_function(&self, message: &String) -> Message {
    prompt_str(message, &self.stage)
  }
}
