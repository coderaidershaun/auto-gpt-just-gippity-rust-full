use std::io::prelude::*;
use std::fs::File;
use std::fs;
use tokio::process::Command as TokioCommand;

use std::env;
use std::path::PathBuf;

// Trims string to length
pub fn trim_string_to_length(s: &str, max_length: usize) -> String {
  s.chars()
    .enumerate()
    .filter_map(|(i, c)| if i < max_length { Some(c) } else { None })
    .collect::<String>()
}


// Saves LLM generated script to file to be executed
pub fn save_script_to_python_file(script: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
  let mut file = File::create(filename)?;
  file.write_all(script.as_bytes())?;
  Ok(())
}


// Remove any files named output
fn remove_any_existing_output_files() -> Result<(), Box<dyn std::error::Error>> {
  let target_dir = "/Users/shaun/Code/DEVELOPMENT/justgippity2/backend/";
  let keyword = "output";

  // Read the directory and filter out any errors
  let entries = fs::read_dir(target_dir)?.filter_map(Result::ok);

  // Iterate through each entry and remove the file if it's a file and contains the keyword
  for entry in entries {
      let path = entry.path();
      if path.is_file() {
          let file_name = path.file_name().unwrap().to_string_lossy().to_string();

          if file_name.contains(keyword) {
              fs::remove_file(path)?;
          }
      }
  }

  Ok(())
}


// Executes a given Python script
// WARNING: THIS COULD POTENTIALLY BE DANGEROUS!!!
pub async fn execute_python_script(script_path: &str) -> Result<(), Box<dyn std::error::Error>> {

  fn get_project_root() -> PathBuf {
    let current_exe_path = env::current_exe().expect("Failed to get current exe path");
    let mut project_root = current_exe_path.parent().expect("Failed to get parent directory").to_path_buf();
    while project_root.file_name().expect("Failed to get file name") != "target" {
        project_root.pop();
    }
    project_root.pop(); // Move up one directory from 'target' to the project root
    project_root
  }

  let project_root = get_project_root();
  let output_dir = project_root.join("output"); // Set the desired output directory

  // Remove exiting output files
  let remove_res: Result<(), Box<dyn std::error::Error>> = remove_any_existing_output_files();
  if let Err(_) =  remove_res{
    eprintln!("Unable to remove existing output file");
  }

  // Spawn a new process to execute the Python script
  let python_exec_path: &str = "/Users/shaun/Code/DEVELOPMENT/justgippity2/backend/pythonscripts/venv/bin/python3";
  let output = TokioCommand::new(python_exec_path)
    .arg(script_path)
    .arg("--output-dir")
    .arg(output_dir)
    .output()
    .await?;

  // Handle Result
  if output.status.success() {
      let output_str = String::from_utf8(output.stdout)?;
      println!("Python script output: {}", output_str);
  } else {
      let error_str = String::from_utf8(output.stderr)?;
      eprintln!("Error: {}", error_str);
  }
  Ok(())
}
