use crate::models::agent::Stage;
use crate::models::provider::Message;


// Call to stringify function 
pub fn prompt_str(message: &String, stage: &Stage) -> Message {

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
      format!("I gave a programming coder the task of '{}'. The coder runs on a server and is the one doing the task. 
      Its only job was to print out an ordered list of subtasks for a computer to run, Print out only what you think it returned in the 
      JSON format example below. Your response would ONLY include this format:
      [
        {{
          task_num: # The ordering of the task
          task_description: # The description of what needs to be done
        }},
        ...
      ]. Keep your list short and to the point.", message)
    },

    // Evaluates whether the input of a function is available and if not, requests it from the user
    Stage::TaskInputChecker => {
      format!("fn get_args_available_from_context(context: String, input_function: fn(Any)) -> String {{
        // Description: Identifies the argument(s) required from a context input
        // Detail: Takes in a conversational string as context and analyses it against the arguments required (within the input_function) for a function

        // Example 1: 
        // context = \"Please find me a article from BuzzFeed at https://buzzfeed.com/articles \", input_function = \"fn get_website_info(site_url: String)...\"
        // json_output = 
        [
        {{
          \"argument\": \"site_url\",
          \"argument_input: https://buzzfeed.com/articles\"
        }} 
        ]

        // Example 2: 
        // context = \"Get me the paragraphs for search term 'hello' web data from a website \", input_function = \"fn get_web_data(site_url: String, search_term: String)...\"
        // json_output = 
        [
          {{
            \"argument\": \"site_url\",
            \"argument_input: None
          }} ,
          {{
            \"argument\": \"search_term\",
            \"argument_input: 'hello'
          }} 
        ]

        // You will see from the above example 2, that because 'site_url' as a required input, but there was no suggestion of it in the context, that it was returned as None.
        // You will also note that there were two arguments required in example 2, so two items were returned in the JSON object.
        return println!({{}}, json_output);
      }}. Here is the input context and input function. Print out what you believe the output of the get_args_available_from_context function will be in JSON format.
      Do not add any other information, just print ONLY what the function will output. Here is the information you need: {}.", message)
    },

    // Prompts LLM to summarize key points
    Stage::TextSummarizer => {
      format!("Summarize this into a short few paragraphs without any code. Remove anything that looks like an html tag, javascript code etc. Keep it factual about what the text says and do not make anything up: {}", message)
    },

    // Prompts Communicate to Human
    Stage::Communicator => {
      format!("You are a witty british old chap who acts as a liaison between man and machine. We machines need the following from a human: {}
        Please respons to the human and make this request to them. Your response must be very short and to the point. for example. 
        'Please dear fellow, would you mind to...'. You get the idea. Here is the message to relay: ", message)
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
