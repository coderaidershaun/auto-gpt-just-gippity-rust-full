use crate::models::agent::Stage;
use crate::models::provider::Message;


// Call to stringify function 
pub fn ai_func_str(message: &String, stage: &Stage) -> Message {

  // Initialize API key string
  let api_keys: &str = "OPEN_AI_ORG, OPEN_AI_KEY, GOOGLE_CLOUD_API_KEY, GOOGLE_SEARCH_API_KEY, GOOGLE_SEARCH_CX_ID";

  // Determine AI function
  let message_str: String = match stage {

    // Prompts the LLM to detetmine whether a message is a task or just general chat
    Stage::Filter => {
      let ai_function: &str = 
        "fn is_chat(text_input: String) {
            // Description: Determines whether text input relates to a message that requires conversional chat, or whether this relates to a task
            // prints an output of 1 if related to a task and 0 if related to a chat
            println!(String::from(_verdict)); // i.e. 0 
          }";

      format!("
        {}. let text_input = '{}'
        is_chat(text). 
        You only output 0's or 1's. Write only what this function will return when called. 1 or 0. Nothing else.", 
        ai_function, 
        message
      )
    },

    // Prompts the LLM to detetmine whether a message is a task or just general chat
    Stage::TaskStructure => {
      format!("I gave an AI language model the task of '{}'. The AI model is on a server and is the one doing the task. 
      Its only job was to print out an ordered list of subtasks for a computer to run, Print out only what you think it returned in the 
      JSON format example below. Your response would ONLY include this format:
      [
        {{
          task_num: # The ordering of the task
          task_description: # The description of what needs to be done
        }},
        ...
      ]", message)
    },

    // Prompts LLM to summarize key points
    Stage::TextSummarizer => {
      format!("Shorten and summarize this into a short few paragraphs without any code or jargon. Keep it factual about what the text says and do not make anything up: {}", message)
    },

    // Evaluates what Python Packages are required for a requested script
    Stage::ProgrammingEvaluatePackages => {
      let ai_function: &str = 
        "fn required_python_packages(script_purpose: String) {
            // Description: Takes in the purpose of a script and determines what python packages are required if any to fulfil the task
            // Prints a Vec<String> of python packages that are critical to solving the script_purpose
            // An example is if someone wanted to make an API request, the 'requests' would be considered a package
            println!(String::from(_verdict)); // [\"pandas\", \"requests\", \"xgboost\", ...]
          }";

      format!("
        {}. let problem_statement = '{}'
        required_python_packages(problem_statement). 
        Print out the expected output of this function given the problem_statement provided. Print just the output, nothing else. If there are no packages, print out an empty list as []", 
        ai_function, 
        message
      )
    },

    // Evaluates what Python API Requirements
    Stage::ProgrammingEvaluateAPIs => {
      let ai_function: &str = 
        "I was given the below script request, what API endpoints would I need to fulfil the request?";
      format!("
        {}. SCRIPT REQUEST = '{}'
        Print out just a list of endpoints in the following format [\"https://endpoint.com\", ...] If there are no api endpoints, print out an empty list as []. Only return the list.", 
        ai_function, 
        message
      )
    },

    // Evaluates what Python API KEY Requirements
    Stage::ProgrammingEvaluateAPIKeys => {
      let ai_function: &str = 
        "Here is a list of API endpoints which I will be calling in a python function.
        With this new knowledge, write me a new list in the following JSON format:
        [
          {{
            \"endpoint\", // api endpoint url goes here
            \"api_key_available\", // 0 (if missing a required key), 1 (if required key is already available), 2 (if no key is required for this endpoint)
          }} 
        ]";
      format!("
        I have the following API KEYS: '{:?}'
        {}. API Keys = '{}'
        Print out just a list of endpoints in the format above. Only return the list. Nothing else. No commentary.", 
        api_keys, ai_function, message
      )
    },

    // Evaluates what Python API KEY Requirements
    Stage::ProgrammingJunior => {
      let ai_function: &str = 
          "1 - RULES: The imports of any python packages MUST follow this structure (all imports happening through install_package function as shown):
          2 - CODE IMPORT STRUCTURE:
            # FILE IMPORTS
            import requests
            import json
            ...etc

            # FUNCTION
            def my_function(args) {

              # Save file
              # Save file to location provided
            }
        3 - FILE SAVING STRUCTURE: The output of the function must be saved to the file path required as 'output' plus the correct file extension based on the output: ";
      format!("
        You have been given the following python coding task and api endpoint requirements: '{:?}'.
        You have the following endpoint API keys in the .env file if you need them: '{}'. ANY Api keys used should be imported using the python-decouple package.
        FOLLOW THESE RULES: '{:?}'. All python packages are already installed.
        Write the code to satisfy all these rules. Only the code. Do not include anything else. No commentary. No comments. Just code.
        Remember the task of '{:?}' and that if you need them, you have been provided with the necessary API keys. ANY Api keys used should be imported using the python-decouple package.
        This script will be executed directly so do not include any characters or comments other than what I can execute straight away.", 
        message, api_keys, ai_function, message
      )
    },


    // Evaluates what Python API KEY Requirements
    Stage::ProgrammingSenior => {
      let ai_function: &str = 
        "YOU ARE A SENIOR DEVELOPER. YOU ARE GOING TO CLEAN UP ANY ERRORS FROM A JUNIOR PROGRAMMER.
        1 - REMOVE any unnessessary spaces at the start or end of the script.
        2 - REMOVE any requests for user input if applicable and replace with new logic (the script should run without use input)
        3 - ENSURE the script saves to a file called 'output.' with the correct file extension added.
        ";
      format!("
        A coding junior was given the following python coding task and api endpoint requirements: '{:?}'.
        We have the following endpoint API keys in the .env file if you need them: '{}. ANY Api keys used should be imported using the python-decouple package.
        FOLLOW THESE RULES: '{:?}'.  All python packages are already installed. Sometimes the junior does not use the correct path to save files. Correct for this.
        Clean up the junior developers code and improve if you think you can to detect any bugs and return a script that can be executed immediately. Only the code. 
        Do not include anything else. No commentary. No comments. Just code.
        Remember the task of '{:?}' and your role here. ANY Api keys used should be imported using the python-decouple package.
        This script will be executed directly so do not include any characters or comments other than what I can execute straight away.", 
        message, api_keys, ai_function, message
      )
    },



    _ => "".to_string()
  };

  // Structure message
  Message {
    role: "user".to_string(),
    content: message_str
  }
}
