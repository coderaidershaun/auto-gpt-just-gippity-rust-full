use crate::plugins::plugins::{
  search_youtube_for_video_metadata, 
  search_google, 
  action_from_human, 
  scrape_and_sumamrize_webpage_content, 
  llm_summarise_content,
  python_code_programmer,
  execute_prewritten_python_script
};
use crate::models::provider::Message;
use crate::models::plugins::{TaskArgs};
use crate::models::agent::Agent;
use std::fmt::Debug;


// Structure task message
pub fn convert_to_message<T>(query: T, message_type: &str) -> Message
where T: Debug {
  let res: String = format!("{:?}", query);
  Message {
    role: message_type.to_string(),
    content: format!("{:?}", res)
  }
}

// Handle function response - push response to agent memory
async fn handle_function_response<T>(
  func_resp: T,
  agent: &mut Agent,
  role: &str,
) -> Result<(), Box<dyn std::error::Error>>
where
  T: std::future::Future<Output = Result<String, Box<dyn std::error::Error>>> {
  let response: String = func_resp.await?;
  let response_message: Message = convert_to_message(response, role);
  agent.memory.push(response_message);
  Ok(())
}


// Handle task funciton
pub async fn handle_task_function(function_num: u32, agent: &mut Agent, task_args: Vec<TaskArgs>) -> Result<(), Box<dyn std::error::Error>> {
  match function_num {
      1 => handle_function_response(search_youtube_for_video_metadata(&task_args[0].argument_input), agent, "user").await?,
      2 => handle_function_response(search_google(&task_args[0].argument_input), agent, "user").await?,
      3 => handle_function_response(action_from_human(&task_args[0].argument_input), agent, "user").await?,
      4 => handle_function_response(scrape_and_sumamrize_webpage_content(&task_args[0].argument_input), agent, "user").await?,
      5 => handle_function_response(llm_summarise_content(task_args[0].argument_input.clone()), agent, "user").await?,
      6 => handle_function_response(python_code_programmer(&task_args[0].argument_input), agent, "user").await?,
      7 => handle_function_response(execute_prewritten_python_script(&task_args[0].argument_input), agent, "user").await?,
      _ => {
          eprintln!("Missing function number {}", function_num);
          panic!("No such function listed")
      }
  }

  Ok(())
}

