mod llm;
mod models;
mod plugins;
mod helpers;

use models::agent::{Agent, Stage};
use models::provider::{Message, Task};
use plugins::plugin_arr::PLUGIN_FUNC_ARR;
use serde_json;


#[tokio::main]
async fn main() {

    // // Extract objective from user
    // println!("Hey there, what can I help you with today?");
    // let mut objective: String = String::new();
    // std::io::stdin().read_line(&mut objective).expect("Failed to read response");

    // // Set initial message
    // let initial_message = objective;


    // !!! DELETE
    let initial_message: String = String::from("summarise this YT video for me");



    // // Structure message
    // let message: Message = Message {
    //     role: "user".to_string(),
    //     content: initial_message.clone()
    // };

    // // Create an instance of our agent
    // let mut agent: Agent = Agent::new(
    //     "Just Gippity".to_string(), 
    //     "Shaun".to_string(), 
    //     initial_message.clone(),
    //     message
    // );

    // // Determine message for ai function
    // let ai_function_filtration: Message = agent.get_ai_function(&initial_message);

    // // Task type filtration
    // println!("Filtering...");
    // let task_type_res: Result<String, Box<dyn std::error::Error>> = agent.get_response(Some(ai_function_filtration)).await;

    // // Extract Number
    // let chat_task_filter: Option<char> = match task_type_res {
    //     Ok(task_type) => task_type.chars().next(),
    //     Err(e) => {
    //         eprintln!("{e}");
    //         panic!("Failed to return a task type")
    //     }
    // };

    // // Obtain respond to chat message
    // if chat_task_filter == Some('0') {
    //     println!("Response: {:?}", chat_task_filter);
    //     return println!("Ending chat as not a task");
    // }

    // // Structure and order tasks 
    // println!("Structuring tasks...");
    // let task_listing_res: Result<String, Box<dyn std::error::Error>> = match chat_task_filter {
    //     Some('1') => {
    //         agent.stage = Stage::TaskStructure;
    //         let ai_function_task_structure: Message = agent.get_ai_function(&initial_message);
    //         agent.get_response(Some(ai_function_task_structure)).await
    //     },
    //     _ => {
    //         panic!("An option was returned which was not recognised")
    //     }
    // };

    // // Assign tasks to functions
    // println!("Assigning task functions...");
    // let tasks_with_funcitons_res: Result<String, Box<dyn std::error::Error>> = match task_listing_res {
    //     Ok(task_listing) => {
    //         let msg_part_1: String = format!("I gave an AI language model the task to '{}'. This is what it came up with: ", {initial_message});
    //         let msg_part_3: &str = " The model has the following functions at its disposal to help achieve any given task from above:";
    //         let msg_part_5: &str = "Your job is to re-write the tasks from scratch in the way you best see fit, now that you have an idea of what functions are at the AI models disposal. Your response MUST be in the following json format only. Nothing else:
    //             [
    //                 {
    //                     task_number: 1 # The ordering of the task
    //                     function_number: 5 # Relates to the most appropriate function above
    //                     task_description: Description of the task
    //                     is_machine: true # true or false as to whether a machine can do the task
    //                 },
    //                 {...},
    //                 ...
    //             ]
    //         ";
    //         let msg_concat: String = format!("{}, {}, {}, {:?}, {}", msg_part_1, task_listing, msg_part_3, PLUGIN_FUNC_ARR, msg_part_5);
    //         let message: Message = Message {
    //             role: "user".to_string(),
    //             content: msg_concat
    //         };
    //         agent.get_response(Some(message)).await
    //     },
    //     Err(e) => {
    //         eprintln!("{}", e);
    //         panic!("Failed at obtaining the full task list")
    //     }
    // };

    // // Deserialize tasks and functions into array of tasks
    // let full_tasklist: Vec<Task> = match tasks_with_funcitons_res {
    //     Ok(tasks_with_funcitons) => {
    //         dbg!(&tasks_with_funcitons);
    //         let tasks: Vec<Task> = serde_json::from_str(tasks_with_funcitons.as_str()).expect("Failed to unwrap tasklist from JSON format to Vec<Task>");
    //         tasks
    //     },
    //     Err(e) => {
    //         eprintln!("{}", e);
    //         panic!("Failed at obtaining the full task list")   
    //     }
    // };



    // DELETE !!!!!!!!!
    let full_tasklist_str_delete = "[\n  {\n    \"task_number\": 1,\n    \"function_number\": 1,\n    \"task_description\": \"Transcribe the YouTube video URL into text\",\n    \"is_machine\": true\n  },\n  {\n    \"task_number\": 2,\n    \"function_number\": 3,\n    \"task_description\": \"Use the large language model to summarize the transcribed video text\",\n    \"is_machine\": true\n  },\n  {\n    \"task_number\": 3,\n    \"function_number\": 2,\n    \"task_description\": \"Optional: Browse any related website URL to gather additional information\",\n    \"is_machine\": true\n  }]";
    let full_tasklist: Vec<Task> = serde_json::from_str(full_tasklist_str_delete).expect("Failed to unwrap tasklist from JSON format to Vec<Task>");


    for task in full_tasklist {
        println!("Task Number: {:?}", task.task_number);
        println!("Function Number: {:?}", task.function_number);
        println!("Task Description: {:?}", task.task_description);
        println!("Is Machine: {:?}", task.is_machine);
        println!("");

        // Perform Function
    }

    // dbg!(full_tasklist);
}
