// Search YouTube
const PLUGIN_1: &str = 
"FUNCTION 1: search_youtube_for_video_metadata(query: &str) -> Result<YTSearchResponse, Box<dyn std::error::Error>> {
  // Description: Searches YouTube to find a list of up to 30 videos
  // Returns: list of videos containing:  video id, video title, video description, published datetime, list of thumbnails
  return list // [{title: \"Video Title\", description: \"Video Description\", thumnails: [\"https://youtube.image.com/thumnail1\", ...]}, ...]
}";

// Search Google
const PLUGIN_2: &str = 
"FUNCTION 2: search_google(search_query: &str) -> Result<(), Box<dyn std::error::Error>> {
  // Description: Searches Google based on input query for up to 30 results
  // Returns: list of videos containing:  video id, video title, video description, published datetime, list of thumbnails
  return list // [{page_title: \"Page Title\", page_description: \"Description of webpage\", ...}, ...]
}";

// Get human to perform action
const PLUGIN_3: &str = 
"FUNCTION 3: action_from_human(request_to_human: &str) -> String {
  // Description: Requests for a human to perform a task that a machine write code to perform.
  // Examples could include: Put food in the freezer, confirm how much you want to invest, etc...
  // Caution: If a computer on a server can perform this task, this function is not used
  // Returns: Returns output from human. For example: Task done, I want to invest $1Million
  return response // Human response: done. I have actioned what you needed me to.
}";

// LLM task
const PLUGIN_4: &str = 
"FUNCTION 4: scrape_and_sumamrize_webpage_content(site_url: &str) -> Result<String, Box<dyn std::error::Error>> {
  // Description: Scrapes website data for a given site url and uses a large language model to summarize
  // Returns: list of videos containing:  video id, video title, video description, published datetime, list of thumbnails
}";

// LLM task
const PLUGIN_5: &str = 
"FUNCTION 5: llm_summarise_content(input_content: String) -> Result<String, Box<dyn std::error::Error>> {
  // Description: A large language model receives a body of text and can return a summary of this
  return text // i.e. Here is your requested output, you now have the following sumamry I as an AI language model provided...
}";

// Write Python Program
const PLUGIN_6: &str = 
"FUNCTION 6: python_code_programmer(required_code_solution: &str) -> Result<(), Box<dyn std::error::Error>> {
  // Description: An expert code writer receives a request to write python code and saves it to a file to be used later on
  // Use Case: Used when existing functions are not available and custom code can be written to solve a task or problem
  // Examples: Write a python script that makes a flash loan, write some code for a user, write a script to get some information from an API
  // When to Use: Should be used if no other functions are already written to perform a necessary task
  // Returns: Returns the working directory path and filename for where the code is saved
  // Return format example: /Users/usr/Code/DEVELOPMENT/myawesomeapp/backend/solution.py
}";

// Execute code written by python_code_programmer function
const PLUGIN_7: &str = 
"FUNCTION 7: execute_prewritten_python_script(script_path: &str) -> Result<(), Box<dyn std::error::Error>> {
  // Description: Executes the code written by the python_code_programmer function
  // Return format example: /Users/usr/Code/DEVELOPMENT/myawesomeapp/backend/output.(relevant file extension)
}";

// PLUGINS
pub const PLUGIN_FUNC_ARR: [&str; 7] = [
  PLUGIN_1,
  PLUGIN_2,
  PLUGIN_3,
  PLUGIN_4,
  PLUGIN_5,
  PLUGIN_6,
  PLUGIN_7
];

