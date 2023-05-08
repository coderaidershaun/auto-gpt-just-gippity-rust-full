mod llm;
mod models;
mod plugins;
mod helpers;

use models::agent::{Agent, Stage};
use models::provider::{Message, Task};
use models::plugins::{TaskArgs};
use helpers::task_execution::handle_task_function;
use plugins::plugin_arr::PLUGIN_FUNC_ARR;

use serde_json;


#[tokio::main]
async fn main() {

    // Extract objective from user
    println!("Hey there, what can I help you with today?");
    let mut objective: String = String::new();
    std::io::stdin().read_line(&mut objective).expect("Failed to read response");

    // Set initial message
    let initial_message = objective;



    // // !!! DEBUG ONLY
    // let initial_message: String = String::from("summarise this YT video for me");



    // Structure message
    let message: Message = Message {
        role: "user".to_string(),
        content: initial_message.clone()
    };

    // Create an instance of our agent
    let mut agent: Agent = Agent::new(
        "Just Gippity".to_string(), 
        "Shaun".to_string(), 
        initial_message.clone(),
        message
    );

    // Determine message for ai function
    let ai_function_filtration: Message = agent.get_ai_function(&initial_message);

    // Task type filtration
    println!("Filtering...");
    let task_type_res: Result<String, Box<dyn std::error::Error>> = agent.get_response(Some(ai_function_filtration)).await;

    // Extract Number
    let chat_task_filter: Option<char> = match task_type_res {
        Ok(task_type) => task_type.chars().next(),
        Err(e) => {
            eprintln!("{e}");
            panic!("Failed to return a task type")
        }
    };

    // Obtain respond to chat message
    if chat_task_filter == Some('0') {
        println!("Response: {:?}", chat_task_filter);
        return println!("Ending chat as not a task");
    }

    // Structure and order tasks 
    println!("Structuring tasks...");
    let task_listing_res: Result<String, Box<dyn std::error::Error>> = match chat_task_filter {
        Some('1') => {
            agent.stage = Stage::TaskStructure;
            let ai_function_task_structure: Message = agent.get_ai_function(&initial_message);
            agent.get_response(Some(ai_function_task_structure)).await
        },
        _ => {
            panic!("An option was returned which was not recognised")
        }
    };

    // Assign tasks to functions
    println!("Assigning task functions...");
    let tasks_with_funcitons_res: Result<String, Box<dyn std::error::Error>> = match task_listing_res {
        Ok(task_listing) => {
            let msg_part_1: String = format!("I gave an AI language model the task to '{}'. This is what it came up with: ", {initial_message});
            let msg_part_3: &str = " The model has the following functions at its disposal to help achieve any given task from above:";
            let msg_part_5: &str = "Your job is to re-write the tasks from scratch in the way you best see fit, now that you have an idea of what functions are at the AI models disposal. Your response MUST be in the following json format only. Nothing else:
                [
                    {
                        \"task_number\": 1 # The ordering of the task
                        \"function_number\": 5 # Relates to the most appropriate function above
                        \"task_description\": Description of the task
                    },
                    {...},
                    ...
                ]

                Keep the list as short as possible. We do not need to use all functions unless absolutely necessary. Do not make any assumptions about the task. If you think something is missing, ask a human to clarify as one of the task steps.
            ";
            let msg_concat: String = format!("{}, {}, {}, {:?}, {}", msg_part_1, task_listing, msg_part_3, PLUGIN_FUNC_ARR, msg_part_5);
            let message: Message = Message {
                role: "user".to_string(),
                content: msg_concat
            };
            agent.get_response(Some(message)).await
        },
        Err(e) => {
            eprintln!("{}", e);
            panic!("Failed at obtaining the full task list")
        }
    };

    // Deserialize tasks and functions into array of tasks
    let full_tasklist: Vec<Task> = match tasks_with_funcitons_res {
        Ok(tasks_with_funcitons) => {
            dbg!(&tasks_with_funcitons);
            let tasks: Vec<Task> = serde_json::from_str(tasks_with_funcitons.as_str()).expect("Failed to unwrap tasklist from JSON format to Vec<Task>");
            tasks
        },
        Err(e) => {
            eprintln!("{}", e);
            panic!("Failed at obtaining the full task list")   
        }
    };


    // Show Full Tasklist
    println!("Tasks as follows:");
    for (index, task) in full_tasklist.iter().enumerate() {
        println!("Task: {:?}", index);
        println!("Function: {:?}", task.function_number);
        println!("Description: {:?}", task.task_description);
        println!("");
    }



    // // DEBUG ONLY !!!!!!!!!
    // let full_tasklist_str_delete = "[\n    {\n        \"task_number\": 1,\n        \"function_number\": 1,\n        \"task_description\": \"Search YouTube for the desired video metadata based on user query\",\n        \"is_machine\": true\n    },\n    {\n        \"task_number\": 2,\n        \"function_number\": 3,\n        \"task_description\": \"Request a human to watch the video and provide a timestamp-based transcription\",\n        \"is_machine\": false\n    },\n    {\n        \"task_number\": 3,\n        \"function_number\": 5,\n        \"task_description\": \"Use the large language model to extract key points from the transcription\",\n        \"is_machine\": true\n    },\n    {\n        \"task_number\": 4,\n        \"function_number\": 5,\n        \"task_description\": \"Condense the extracted key points into a summary using the large language model\",\n        \"is_machine\": true\n    },\n    {\n        \"task_number\": 5,\n        \"function_number\": 6,\n        \"task_description\": \"Write a python script to convert the summary into JSON format\",\n        \"is_machine\": true\n    },\n    {\n        \"task_number\": 6,\n        \"function_number\": 7,\n        \"task_description\": \"Execute the pre-written python script to convert the summary into JSON format\",\n        \"is_machine\": true\n    },\n    {\n        \"task_number\": 7,\n        \"function_number\": 4,\n        \"task_description\": \"Return the JSON formatted summary to the user\",\n        \"is_machine\": true\n    }\n]";
    // let full_tasklist: Vec<Task> = serde_json::from_str(full_tasklist_str_delete).expect("Failed to unwrap tasklist from JSON format to Vec<Task>");



    // Loop through and manage tasks
    for task in full_tasklist {

        // Check task function inputs - Message structure
        println!("Checking task function inputs to accomplish task: \"{}\"...", task.task_description);
        let function_str: &str = PLUGIN_FUNC_ARR[task.function_number as usize - 1];
        let context: &Vec<Message> = &agent.memory;
        let task_context: String = format!("Context {{ {:?} {} }}", context, task.task_description);
        let input_message: String = format!("let context = {:?} \n let input_function = {}", task_context, function_str);
        
        // Request LLM Feedback
        agent.stage = Stage::TaskInputChecker;
        let ai_function_task_checker: Message = agent.get_ai_function(&input_message);
        let task_check_res: Result<String, Box<dyn std::error::Error>> = agent.get_response(Some(ai_function_task_checker)).await;

        // Extract function arguments
        let task_args: Vec<TaskArgs> = if let Ok(task_check) = task_check_res {
            let task_args_decoded: Vec<TaskArgs> = serde_json::from_str(task_check.as_str()).expect("Failed to unwrap task argument inputs from Vec<TaskArgs>");
            task_args_decoded
        } else {
            eprintln!("{:?}", task_check_res);
            panic!("Failed to retrieve task arguments in correct input")
        };

        // Handle the selected function
        println!("Performing task {}, function number {}, function args: {:?}", task.task_number, task.function_number, task_args);
        let _ = handle_task_function(task.function_number, &mut agent, task_args).await;

        // Decide whether to continue
        dbg!(&agent.memory);
        println!("Conitnue to next Task?");
        let mut human_response: String = String::new();
        std::io::stdin().read_line(&mut human_response).expect("Failed to read response");
    }
}
