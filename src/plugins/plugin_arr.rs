const plugin_0_transcribe_youtube_video: &str = 
"fn transcribe_youtube_video(video_url: &str) -> String {{
  // Description: takes in a video url and outputs the video transcription
  return transcription // Good afternoon and welcome to my video about…
}}";

const plugin_1_browse_internet_site: &str = 
"fn browse_internet_site(site_url: &str) -> String {{
  // Description: takes in a website url and extracts the source code
  return site_source_code // <html><body><p>hello my good friend</p>...
}}";

const plugin_2_summarize_text: &str = 
"fn perform_llm_task(input_text: &str) -> String {{
  // Description: large language model summarizes input text and makes it more concise
  return llm_output_message // i.e. Here is the executive summary of model is the XYZ company...
}}";

const plugin_10_summarize_text: &str = 
"fn give_task_to_human(task_num: u8, task_description: &str) -> String {{
  // Description: Gives a task to a human to perform as is not possible for a machine on a server to perform
  return task_for_human // i.e. Need a person to take over on getting a drink. I am a machine and do not have hands...
}}";

pub const PLUGIN_FUNC_ARR: [&str; 4] = [
  plugin_0_transcribe_youtube_video,
  plugin_1_browse_internet_site,
  plugin_2_summarize_text,
  plugin_10_summarize_text
];

