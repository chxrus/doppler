use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;

const GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";
const DEFAULT_MODEL: &str = "gemini-2.5-flash";

pub fn default_model() -> &'static str {
    DEFAULT_MODEL
}

#[derive(Debug, Error)]
pub enum GeminiError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Invalid API response: {0}")]
    InvalidResponse(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("No content in response")]
    NoContent,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<Candidate>>,
    error: Option<ApiError>,
    #[serde(rename = "promptFeedback")]
    prompt_feedback: Option<PromptFeedback>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: Option<Content>,
}

#[derive(Debug, Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Deserialize)]
struct Part {
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiError {
    message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PromptFeedback {
    #[serde(rename = "blockReason")]
    block_reason: Option<String>,
}

pub async fn validate_api_key(api_key: &str) -> Result<(), GeminiError> {
    let api_key = api_key.trim();
    if api_key.is_empty() {
        return Err(GeminiError::InvalidResponse("API key is empty".to_string()));
    }

    let client = Client::new();
    let url = format!("{GEMINI_BASE_URL}/models");
    let response = client.get(url).query(&[("key", api_key)]).send().await?;

    if response.status().is_success() {
        return Ok(());
    }

    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    Err(parse_api_error(status, &body))
}

pub async fn send_message(api_key: &str, message: &str) -> Result<String, GeminiError> {
    let api_key = api_key.trim();
    if api_key.is_empty() {
        return Err(GeminiError::InvalidResponse("API key is empty".to_string()));
    }
    let message = message.trim();
    if message.is_empty() {
        return Err(GeminiError::InvalidResponse("Message is empty".to_string()));
    }

    let client = Client::new();
    let url = format!("{GEMINI_BASE_URL}/models/{DEFAULT_MODEL}:generateContent");

    let response = client
        .post(url)
        .header("x-goog-api-key", api_key)
        .json(&json!({
            "contents": [{
                "parts": [{
                    "text": message
                }]
            }],
            "generationConfig": {
                "responseMimeType": "text/plain",
                "temperature": 0.7,
                "maxOutputTokens": 8192
            }
        }))
        .send()
        .await?;

    let status = response.status();
    let raw_body = response.text().await?;
    let body: GeminiResponse = serde_json::from_str(&raw_body).map_err(|error| {
        GeminiError::InvalidResponse(format!(
            "Failed to parse Gemini response (status {status}): {error}"
        ))
    })?;

    if !status.is_success() {
        return Err(parse_api_error(status, &raw_body));
    }

    if let Some(error) = body.error {
        return Err(GeminiError::ApiError(error.message.unwrap_or_else(|| {
            "Gemini returned an unknown API error".to_string()
        })));
    }

    if let Some(prompt_feedback) = body.prompt_feedback {
        if let Some(block_reason) = prompt_feedback.block_reason {
            return Err(GeminiError::ApiError(format!(
                "Prompt blocked by Gemini: {block_reason}"
            )));
        }
    }

    let text = body.candidates.and_then(|candidates| {
        candidates.into_iter().find_map(|candidate| {
            candidate.content.and_then(|content| {
                content
                    .parts
                    .into_iter()
                    .find_map(|part| part.text.filter(|text| !text.trim().is_empty()))
            })
        })
    });

    text.ok_or(GeminiError::NoContent)
}

fn parse_api_error(status: reqwest::StatusCode, raw_body: &str) -> GeminiError {
    let parsed = serde_json::from_str::<GeminiResponse>(raw_body)
        .ok()
        .and_then(|body| body.error)
        .and_then(|error| error.message);

    match parsed {
        Some(message) if !message.trim().is_empty() => GeminiError::ApiError(message),
        _ => GeminiError::ApiError(format!("HTTP {status}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_first_text_candidate() {
        let sample = r#"{
          "candidates":[
            {
              "content":{
                "parts":[{"text":"Hello from Gemini"}]
              },
              "finishReason":"STOP"
            }
          ]
        }"#;

        let parsed = serde_json::from_str::<GeminiResponse>(sample).expect("valid response json");
        let text = parsed
            .candidates
            .unwrap_or_default()
            .into_iter()
            .find_map(|candidate| {
                candidate.content.and_then(|content| {
                    content
                        .parts
                        .into_iter()
                        .find_map(|part| part.text.filter(|value| !value.trim().is_empty()))
                })
            });

        assert_eq!(text.as_deref(), Some("Hello from Gemini"));
    }
}
