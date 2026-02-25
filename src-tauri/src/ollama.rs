use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;

const DEFAULT_MODEL: &str = "llama3.2:3b";
const DEFAULT_BASE_URL: &str = "http://localhost:11434";

#[derive(Debug, Error)]
pub enum OllamaError {
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
struct OllamaResponse {
    message: Option<OllamaMessage>,
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OllamaMessage {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OllamaTagsResponse {
    models: Option<Vec<OllamaTagModel>>,
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OllamaTagModel {
    name: Option<String>,
}

pub async fn send_message(
    base_url: Option<&str>,
    model: Option<&str>,
    message: &str,
    temperature: Option<f32>,
) -> Result<String, OllamaError> {
    let message = message.trim();
    if message.is_empty() {
        return Err(OllamaError::InvalidResponse("Message is empty".to_string()));
    }

    let selected_base_url = base_url
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_BASE_URL)
        .trim_end_matches('/');
    let selected_model = model
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_MODEL);
    let selected_temperature = temperature.unwrap_or(0.7);

    let url = format!("{selected_base_url}/api/chat");
    let client = Client::new();
    let response = client
        .post(url)
        .json(&json!({
            "model": selected_model,
            "messages": [{
                "role": "user",
                "content": message
            }],
            "stream": false,
            "options": {
                "temperature": selected_temperature
            }
        }))
        .send()
        .await?;

    let status = response.status();
    let raw_body = response.text().await?;
    let body: OllamaResponse = serde_json::from_str(&raw_body).map_err(|error| {
        OllamaError::InvalidResponse(format!(
            "Failed to parse Ollama response (status {status}): {error}"
        ))
    })?;

    if !status.is_success() {
        return Err(parse_api_error(status, &body));
    }

    if let Some(error) = body.error {
        if !error.trim().is_empty() {
            return Err(OllamaError::ApiError(error));
        }
    }

    body.message
        .and_then(|message| message.content)
        .map(|content| content.trim().to_string())
        .filter(|content| !content.is_empty())
        .ok_or(OllamaError::NoContent)
}

fn parse_api_error(status: reqwest::StatusCode, response: &OllamaResponse) -> OllamaError {
    if let Some(error) = response.error.as_ref().filter(|error| !error.trim().is_empty()) {
        return OllamaError::ApiError(error.clone());
    }

    OllamaError::ApiError(format!("HTTP {status}"))
}

pub async fn list_models(base_url: Option<&str>) -> Result<Vec<String>, OllamaError> {
    let selected_base_url = base_url
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_BASE_URL)
        .trim_end_matches('/');

    let url = format!("{selected_base_url}/api/tags");
    let client = Client::new();
    let response = client.get(url).send().await?;
    let status = response.status();
    let raw_body = response.text().await?;
    let body: OllamaTagsResponse = serde_json::from_str(&raw_body).map_err(|error| {
        OllamaError::InvalidResponse(format!(
            "Failed to parse Ollama tags response (status {status}): {error}"
        ))
    })?;

    if !status.is_success() {
        if let Some(error) = body.error.filter(|error| !error.trim().is_empty()) {
            return Err(OllamaError::ApiError(error));
        }
        return Err(OllamaError::ApiError(format!("HTTP {status}")));
    }

    let mut models = body
        .models
        .unwrap_or_default()
        .into_iter()
        .filter_map(|model| model.name.map(|name| name.trim().to_string()))
        .filter(|name| !name.is_empty())
        .collect::<Vec<String>>();

    models.sort();
    models.dedup();
    Ok(models)
}
