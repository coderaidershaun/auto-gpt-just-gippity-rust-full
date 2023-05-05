use std::io::prelude::*;
use std::fs::File;
use tokio::process::Command as TokioCommand;
use std::fs;


// Saves LLM generated script to file to be executed
pub fn save_script_to_python_file(script: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
  let mut file = File::create(filename)?;
  file.write_all(script.as_bytes())?;
  Ok(())
}


// Remove any files named output
fn remove_any_existing_output_files() -> Result<(), Box<dyn std::error::Error>> {
  let output_dir = "/Users/shaun/Code/DEVELOPMENT/justgippity2/backend/";

  // Read the directory and filter out any errors
  let entries = fs::read_dir(output_dir)?.filter_map(Result::ok);

  // Iterate through each entry and remove the file if it's a file
  for entry in entries {
      let path = entry.path();
      if path.is_file() {
          fs::remove_file(path)?;
      }
  }

  Ok(())
}


// Executes a given Python script
// WARNING: THIS COULD POTENTIALLY BE DANGEROUS!!!
pub async fn execute_python_script(script_path: &str) -> Result<(), Box<dyn std::error::Error>> {

  // Remove exiting output files
  let remove_res: Result<(), Box<dyn std::error::Error>> = remove_any_existing_output_files();
  if let Err(_) =  remove_res{
    eprintln!("Unable to remove existing output file");
  }

  // Spawn a new process to execute the Python script
  let python_exec_path: &str = "/Users/shaun/Code/DEVELOPMENT/justgippity2/backend/pythonscripts/venv/bin/python3";
  let output = TokioCommand::new(python_exec_path)
    .arg(script_path)
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
