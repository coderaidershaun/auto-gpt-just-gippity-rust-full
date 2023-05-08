use crate::helpers::helpers::{save_script_to_python_file, execute_python_script, trim_string_to_length};
use crate::models::plugins::{YTSearchResponse, APIEndpointKey};
use crate::models::provider::Message;
use crate::models::agent::Stage;
use crate::llm::request::call_ai_llm;
use crate::llm::prompts::prompt_str;
use reqwest::Client;
use scraper::{Html, Selector};
use serde_json::Value;
use tokio::process::Command as AsyncCommand;
use std::env;
use dotenv::dotenv;

/// YouTube: API Instructions: https://developers.google.com/youtube/v3/docs/search/list
/// Search: Add Search Engine: https://programmablesearchengine.google.com/controlpanel/all
/// Search: API Key and Instructions: https://developers.google.com/custom-search/v1/overview


// Search YT and obtain video metadata information
pub async fn search_youtube_for_video_metadata(query: &str) -> Result<YTSearchResponse, Box<dyn std::error::Error>> {
  // Description: Searches YouTube to find a list of up to 30 videos
  // Returns: list of videos containing:  video id, video title, video description, published datetime, list of thumbnails
  dotenv().ok();

  // Extract API Key information
  let api_key: String = env::var("YOUTUBE_VIDEOS_API_KEY").expect("YOUTUBE_VIDEOS_API_KEY must be set");

  // Construct URL
  let base_url: &str = "https://www.googleapis.com/youtube/v3/search";
  let part: &str = "snippet";
  let max_results: i32 = 3;
  let search_type: &str = "video";
  let request_url: String = format!(
      "{}?part={}&q={}&maxResults={}&type={}&key={}",
      base_url, part, query, max_results, search_type, api_key
  );

  // Construct and return response
  let req_response: reqwest::Response = reqwest::get(&request_url).await?;
  let response_deserialized: YTSearchResponse = req_response.json().await?;
  Ok(response_deserialized)
}


// Search Google and obtain search urls
pub async fn search_google(search_query: &str) -> Result<(), Box<dyn std::error::Error>> {
  // Description: Searches Google based on input query for up to 30 results
  // Returns: list of videos containing:  video id, video title, video description, published datetime, list of thumbnails
  dotenv().ok();

  // Extract API Key information
  let api_key: String = env::var("GOOGLE_SEARCH_API_KEY").expect("GOOGLE_SEARCH_API_KEY must be set");
  let cx_id: String = env::var("GOOGLE_SEARCH_CX_ID").expect("GOOGLE_SEARCH_CX_ID must be set");

  let base_url: &str = "https://www.googleapis.com/customsearch/v1";
  let num_results: i32 = 3; // Maximum number of search results to fetch

  let request_url: String = format!(
      "{}?key={}&cx={}&q={}&num={}",
      base_url, api_key, cx_id, search_query, num_results
  );

  let response: reqwest::Response = reqwest::get(&request_url).await?;
  let result: serde_json::Value = response.json().await?;

  println!("Result: {:#?}", result);
  Ok(())
}


// Request human to perform a task and provide input back
pub async fn action_from_human(request_to_human: &str) -> String {
  // Description: Requests for a human to perform a task that a machine write code to perform.
  // Examples could include: Put food in the freezer, confirm how much you want to invest, etc...
  // Caution: If a computer on a server can perform this task, this function is not used
  // Returns: Returns output from human. For example: Task done, I want to invest $1Million

  // Extract objective from user
  println!("{}", request_to_human);
  let mut human_response: String = String::new();
  std::io::stdin().read_line(&mut human_response).expect("Failed to read response");
  return human_response;
}


// Summarize content or text
pub async fn llm_summarise_content(input_content: String) -> Result<String, Box<dyn std::error::Error>> {
  // Description: A large language model receives a body of text and can return a summary of this
  let api_eval_msg: Message = prompt_str(&input_content, &Stage::TextSummarizer);
  let api_endpoints_str: String = call_ai_llm(Vec::from([api_eval_msg])).await?;
  Ok(api_endpoints_str)
}


// Sumamrize webpage
pub async fn scrape_and_sumamrize_webpage_content(site_url: &str) -> Result<String, Box<dyn std::error::Error>> {
  // Description: Scrapes website data for a given site url and uses a large language model to summarize
  // Returns: list of videos containing:  video id, video title, video description, published datetime, list of thumbnails

  // Extract Webpage content
  println!("Extracting website data...");
  let client: Client = Client::new();
  let resp: reqwest::Response = client.get(site_url).send().await?;
  let body: String = resp.text().await?;
  let parsed_html: Html = Html::parse_document(&body);
  let selector: Selector = Selector::parse("*").unwrap();
  let mut text_content: String = String::new();

  // Iterate through the matching elements and extract text data
  for element in parsed_html.select(&selector) {
      // Ignore elements without any text content
      let text: String = element.text().collect::<String>();
      if text.trim().is_empty() {
          continue;
      }

      // Append the text content to the overall text string
      text_content.push_str(&text.trim());
      text_content.push_str(" ");
  }

  // Trim content size
  let trimmed_content: String = trim_string_to_length(text_content.as_str(), 300);

  // Summarize content
  println!("Text Summarizer: summerizing webpage content...");
  let webpage_summary: String = llm_summarise_content(trimmed_content).await?;

  // Return output
  Ok(webpage_summary)
}


// Write executable python code to use later on
pub async fn python_code_programmer(required_code_solution: &str) -> Result<(), Box<dyn std::error::Error>> {
  // Description: An expert code writer receives a request to write python code and saves it to a file to be used later on
  // Use Case: Used when existing functions are not available and custom code can be written to solve a task or problem
  // Examples: Write a python script that makes a flash loan, write some code for a user, write a script to get some information from an API
  // When to Use: Should be used if no other functions are already written to perform a necessary task
  // Returns: Returns the working directory path and filename for where the code is saved
  // Return format example: /Users/usr/Code/DEVELOPMENT/myawesomeapp/backend/solution.py

  // Initialize
  let script_path: &str = "/Users/shaun/Code/DEVELOPMENT/justgippity2/backend/pythonscripts/solution.py";
  let file_path_output: &str = "output.<ENTER FILE EXTENSION HERE>";
  let pip_directory: &str = "/Users/shaun/Code/DEVELOPMENT/justgippity2/backend/pythonscripts/venv/bin/pip";
  save_script_to_python_file("print('hello world!')", "pythonscripts/solution.py")?;

  // Evaluate Python Packages
  println!("Programming: Evaluating python packages...");
  let package_eval_msg: Message = prompt_str(&required_code_solution.to_string(), &Stage::ProgrammingEvaluatePackages);
  let python_packages_str: String = call_ai_llm(Vec::from([package_eval_msg])).await?;
  let python_packages: Vec<String> = serde_json::from_str(python_packages_str.as_str()).unwrap();

  // Handle installing required python packages
  if python_packages.len() > 0 {

    // Check if user is happy to install Python packages
    println!("Are you comfortable with installing these packages: {}?", python_packages_str);
    println!("yes/no");
    
    // Handle user response
    let mut user_agreement: String = String::new();
    std::io::stdin().read_line(&mut user_agreement).expect("Failed to read user response");
    let acceptable_responses: [&str; 14] = ["y", "yes", "Y", "Yes", "YES", "Ok", "sure", "y\n", "yes\n", "Y\n", "Yes\n", "YES\n", "Ok\n", "sure\n"];
    if !acceptable_responses.contains(&user_agreement.as_str()) {
      panic!("User declined packages. Aborting installation.");
    }

    // Install each Python package
    for pkg in python_packages {

      // Install package
      let mut command: AsyncCommand = AsyncCommand::new(pip_directory);
      command.arg("install").arg(&pkg);

      // Handle installation result
      let status: std::process::ExitStatus = command.status().await?;
      if !status.success() {
        let err_msf = format!("Package '{}' could not be installed", pkg);
        eprintln!("{}", err_msf);
        panic!("Failed to install python package");
      }
    }
  }

  // Evaluate API Endpoints
  println!("Programming: Evaluating endpoints...");
  let api_eval_msg: Message = prompt_str(&required_code_solution.to_string(), &Stage::ProgrammingEvaluateAPIs);
  let api_endpoints_str: String = call_ai_llm(Vec::from([api_eval_msg])).await?;
  let api_endpoints: Vec<String> = serde_json::from_str(api_endpoints_str.as_str()).unwrap();

  // Evaluate API Keys
  println!("Programming: Evaluating api keys...");
  if api_endpoints.len() > 0 {
    let api_key_msg: Message = prompt_str(&required_code_solution.to_string(), &Stage::ProgrammingEvaluateAPIKeys);
    let api_keys_str: String = call_ai_llm(Vec::from([api_key_msg])).await?;
    let api_keys: Vec<APIEndpointKey> = serde_json::from_str(api_keys_str.as_str()).unwrap();

    // Guard: Ensure keys are available:
    for k in api_keys {
      if k.api_key_available == 0 {
        eprintln!("You will need to add a key for '{}' to your .env file", k.endpoint);
        panic!("Missing API Key for task")
      }
    }
  }

  // Construct Junior programmer message
  println!("Programming: Junior programmer working...");
  let endpoints_msg: String = if api_endpoints.len() > 0 { api_endpoints_str } else { "NONE".to_string() };
  let concat_message: String = format!("PROGRAMMING TASK: {}, USE API ENDPOINTS: {}, PYTHON PACKAGES ALLOWED: {}, SCRIPT SAVE OUTPUT TO LOCATION PATH: {}", 
    required_code_solution, endpoints_msg, python_packages_str, file_path_output);
  let programmer_junior_msg: Message = prompt_str(&concat_message, &Stage::ProgrammingJunior);
  let programmer_junior_code: String = call_ai_llm(Vec::from([programmer_junior_msg])).await?;

  // Construct Junior programmer message
  println!("Programming: Senior programmer working...");
  let concat_code_message: String = format!("PROGRAMMING TASK: {}, USE API ENDPOINTS: {}, PYTHON PACKAGES ALLOWED: {}, JUNIOR CODE: {}. SCRIPT SAVE OUTPUT TO LOCATION PATH: {}. Remember to save output here: {}", 
    required_code_solution, endpoints_msg, python_packages_str, programmer_junior_code, file_path_output, file_path_output);
  let programmer_senior_msg: Message = prompt_str(&concat_code_message, &Stage::ProgrammingSenior);
  let programmer_senior_code: String = call_ai_llm(Vec::from([programmer_senior_msg])).await?;

  // Save script to file
  println!("Saving programmer script...");
  save_script_to_python_file(programmer_senior_code.as_str(), script_path)?;
  Ok(())
}


// Execute saved Python script
pub async fn execute_prewritten_python_script(script_path: &str) -> Result<(), Box<dyn std::error::Error>> {
  // Description: Executes the code written by the python_code_programmer function
  println!("Executing required script...");
  execute_python_script(script_path).await;
  Ok(())
}


#[cfg(test)]
mod tests {
  use super::*;


  #[tokio::test]
  async fn searches_yt_for_metadata() {
    let query: &str = "Crypto Wizards";
    let yt_search_res: Result<YTSearchResponse, Box<dyn std::error::Error>> = search_youtube_for_video_metadata(query).await;
    dbg!(yt_search_res.unwrap());
  }

  #[tokio::test]
  async fn searches_google() {
    let query: &str = "Best place to eat near me";
    let google_search_res: Result<(), Box<dyn std::error::Error>> = search_google(query).await;
  }

  #[tokio::test]
  async fn scrapes_and_summarizes_webpage_data() {
    let query: &str = "https://cryptowizards.net";
    let webpage_summary_res: Result<String, Box<dyn std::error::Error>> = scrape_and_sumamrize_webpage_content(query).await;
    dbg!(&webpage_summary_res);
    match webpage_summary_res {
      Ok(_) => assert!(true),
      Err(_) => assert!(false),
    }
  }

  #[tokio::test]
  async fn requests_human_to_perform_task() {
    let request_to_human: &str = "Bake a cake";
    let human_response: String = action_from_human(request_to_human).await;
    dbg!(human_response);
  }

  #[tokio::test]
  async fn writes_python_code() {
    let code_request: &str = "write the script that fetches me a list of YouTube videos.";
    let code_response_res: Result<(), Box<dyn std::error::Error>> = python_code_programmer(code_request).await;
    match code_response_res {
      Ok(_) => assert!(true),
      Err(_) => assert!(false),
    }
  }

  #[tokio::test]
  async fn executes_python_script() {
    let script_path: &str = "/Users/shaun/Code/DEVELOPMENT/justgippity2/backend/pythonscripts/solution.py";
    let exec_res: Result<(), Box<dyn std::error::Error>> = execute_prewritten_python_script(script_path).await;
    match exec_res {
      Ok(_) => assert!(true),
      Err(_) => assert!(false),
    }
  }

}
