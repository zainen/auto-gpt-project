use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fs;
use dotenv::dotenv;

use crate::{apis::call_request::call_gpt, models::general::llm::Message};

use super::command_line::PrintCommand;

const CODE_TEMPLATE_PATH: &str =
  "../web_template/src/code_template.rs";
pub const WEB_SEVER_PROJECT_PATH: &str =
  "../web_template/";
pub const EXEC_MAIN_PATH: &str =
  "../web_template/src/main.rs";
const API_SCHEMA_PATH: &str =
  "../auto_gippity/schemas/api_schema.json";

pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
  let ai_function_str = ai_func(func_input);

  let msg: String = format!(
    "FUNCTION {}
  INSTRUCTION: you are a function printer. You ONLY print the results of functions.
  Nothing else. No commentary. Here is the input to the function {}.
  Print out what the function will return",
    ai_function_str, func_input
  );

  // dbg!(&msg);
  Message {
    role: "system".to_string(),
    content: msg,
  }
}

// fn performs call to llm gpt -- decoded
pub async fn ai_task_request(
  msg_context: String,
  agent_position: &str,
  agent_operation: &str,
  function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
  // extend ai function
  let extended_msg: Message = extend_ai_function(function_pass, &msg_context);

  // print current status
  PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

  // get llm response
  let llm_response_res: Result<String, Box<dyn std::error::Error + Send>> =
    call_gpt(vec![extended_msg.clone()]).await;

  // return success
  match llm_response_res {
    Ok(llm_resp) => llm_resp,
    Err(_) => call_gpt(vec![extended_msg.clone()])
      .await
      .expect("failed twice to call openai"),
  }
}

pub async fn ai_task_request_decoded<T: DeserializeOwned>(
  msg_context: String,
  agent_position: &str,
  agent_operation: &str,
  function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
  let llm_response: String =
    ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;

  let decoded_response: T = serde_json::from_str(llm_response.as_str())
    .expect("failed to decode ai response from serde_json");

  return decoded_response;
}

// check url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
  let response: reqwest::Response = client.get(url).send().await?;
  Ok(response.status().as_u16())
}

// get code template
pub fn read_code_template_contents() -> String {
  dotenv().ok();
  let path: String = String::from(CODE_TEMPLATE_PATH);
  fs::read_to_string(path).expect("failed to read code template")
}

// get exec main
pub fn read_exec_main_contents() -> String {
  let path: String = String::from(EXEC_MAIN_PATH);
  fs::read_to_string(path).expect("failed to read exec main")
}

// save new backend code
pub fn save_backend_code(contents: &String) {
  let path: String = String::from(EXEC_MAIN_PATH);
  fs::write(path, contents).expect("Failed to write main.rs file");
}

// save json api endpoint schema
pub fn save_api_endpoints(api_endpoints: &String) {
  let path: String = String::from(API_SCHEMA_PATH);
  fs::write(path, api_endpoints).expect("failed to write api endpoints to file");
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

  #[test]
  fn test_extending_ai_function() {
    let extended_msg = extend_ai_function(convert_user_input_to_goal, "dummy var");
    assert_eq!(extended_msg.role, "system".to_string());
  }

  #[tokio::test]
  async fn test_ai_task_request() {
    let ai_func_param: String = "build me a we server for stock prices requests.".to_string();

    let res: String = ai_task_request(
      ai_func_param,
      "Managing Agent",
      "Defining user requirements",
      convert_user_input_to_goal,
    )
    .await;

    dbg!(&res);
    assert!(res.len() > 20);
  }
  
  #[test]
  fn test_read_code_template_contents() {
    let msg = read_code_template_contents();
    dbg!(&msg);
  }
}
