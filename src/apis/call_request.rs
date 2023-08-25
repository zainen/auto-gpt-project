use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

// call LLM ie gpt-4
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
  dotenv().ok();

  let api_key: String = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in env varibales");
  let api_org: String = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in env varibales");

  let url: &str = "https://api.openai.com/v1/chat/completions";

  // create headers
  let mut headers: HeaderMap = HeaderMap::new();

  // create api key header
  headers.insert(
    "authorization",
    HeaderValue::from_str(&format!("Bearer {}", api_key))
      .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
  );

  // create open ai org
  headers.insert(
    "OpenAI-Organization",
    HeaderValue::from_str(&format!("{}", api_org.as_str()))
      .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
  );

  let client = Client::builder()
    .default_headers(headers)
    .build()
    .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

  // create chat completion
  let chat_completion: ChatCompletion = ChatCompletion {
    model: "gpt-4".to_string(),
    messages,
    temperature: 0.1,
  };

  // ToubleShooting
  // let res_raw = client
  // .post(url)
  // .json(&chat_completion)
  // .send()
  // .await
  // .unwrap();

  // dbg!(res_raw.text().await.unwrap());

  // extract api response
  let res: APIResponse = client
    .post(url)
    .json(&chat_completion)
    .send()
    .await
    .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
    .json()
    .await
    .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

  // send response

  Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_call_to_openai() {
    let message: Message = Message {
      role: "user".to_string(),
      content: "Hi there, this is a test, give me a short response.".to_string(),
    };

    let messages: Vec<Message> = vec![message];

    let response = call_gpt(messages).await;

    match response {
      Ok(res_str) => {
        dbg!(res_str);
        assert!(true)
      }
      Err(_) => {
        assert!(false);
      }
    }
  }
}
