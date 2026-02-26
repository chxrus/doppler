use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;

const DEFAULT_BASE_URL: &str = "http://localhost:1234/v1";

#[derive(Debug, Error)]
pub enum LmStudioError {
    #[error("{0}")]
    RequestFailed(String),

    #[error("Invalid API response: {0}")]
    InvalidResponse(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("No content in response")]
    NoContent,
}

#[derive(Debug, Deserialize)]
struct LmStudioChatResponse {
    choices: Option<Vec<LmStudioChoice>>,
    error: Option<LmStudioApiError>,
}

#[derive(Debug, Deserialize)]
struct LmStudioChoice {
    message: Option<LmStudioMessage>,
}

#[derive(Debug, Deserialize)]
struct LmStudioMessage {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LmStudioApiError {
    message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LmStudioModelsResponse {
    data: Option<Vec<LmStudioModel>>,
    error: Option<LmStudioApiError>,
}

#[derive(Debug, Deserialize)]
struct LmStudioModel {
    id: Option<String>,
}

pub async fn send_message(
    base_url: Option<&str>,
    model: Option<&str>,
    message: &str,
    temperature: Option<f32>,
) -> Result<String, LmStudioError> {
    let message = message.trim();
    if message.is_empty() {
        return Err(LmStudioError::InvalidResponse(
            "Message is empty".to_string(),
        ));
    }

    let selected_base_url = normalize_base_url(base_url);
    let selected_model = model
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| LmStudioError::InvalidResponse("LM Studio model is empty".to_string()))?;
    let selected_temperature = temperature.unwrap_or(0.7);
    let url = format!("{selected_base_url}/chat/completions");

    let client = Client::new();
    let response = client
        .post(url)
        .json(&json!({
            "model": selected_model,
            "messages": [{
                "role": "user",
                "content": message
            }],
            "temperature": selected_temperature
        }))
        .send()
        .await
        .map_err(map_request_error)?;

    let status = response.status();
    let raw_body = response.text().await.map_err(map_request_error)?;
    let body: LmStudioChatResponse = serde_json::from_str(&raw_body).map_err(|error| {
        LmStudioError::InvalidResponse(format!(
            "Failed to parse LM Studio response (status {status}): {error}"
        ))
    })?;

    if !status.is_success() {
        if let Some(error_message) = body
            .error
            .and_then(|error| error.message)
            .filter(|error| !error.trim().is_empty())
        {
            return Err(LmStudioError::ApiError(error_message));
        }
        return Err(LmStudioError::ApiError(format!("HTTP {status}")));
    }

    body.choices
        .and_then(|choices| choices.into_iter().next())
        .and_then(|choice| choice.message)
        .and_then(|message| message.content)
        .map(|content| content.trim().to_string())
        .filter(|content| !content.is_empty())
        .ok_or(LmStudioError::NoContent)
}

pub async fn list_models(base_url: Option<&str>) -> Result<Vec<String>, LmStudioError> {
    let selected_base_url = normalize_base_url(base_url);
    let url = format!("{selected_base_url}/models");
    let client = Client::new();

    let response = client.get(url).send().await.map_err(map_request_error)?;
    let status = response.status();
    let raw_body = response.text().await.map_err(map_request_error)?;
    let body: LmStudioModelsResponse = serde_json::from_str(&raw_body).map_err(|error| {
        LmStudioError::InvalidResponse(format!(
            "Failed to parse LM Studio models response (status {status}): {error}"
        ))
    })?;

    if !status.is_success() {
        if let Some(error_message) = body
            .error
            .and_then(|error| error.message)
            .filter(|error| !error.trim().is_empty())
        {
            return Err(LmStudioError::ApiError(error_message));
        }
        return Err(LmStudioError::ApiError(format!("HTTP {status}")));
    }

    let mut models = body
        .data
        .unwrap_or_default()
        .into_iter()
        .filter_map(|model| model.id.map(|id| id.trim().to_string()))
        .filter(|id| !id.is_empty())
        .collect::<Vec<String>>();

    models.sort();
    models.dedup();
    Ok(models)
}

fn normalize_base_url(base_url: Option<&str>) -> String {
    base_url
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_BASE_URL)
        .trim_end_matches('/')
        .to_string()
}

fn map_request_error(error: reqwest::Error) -> LmStudioError {
    if error.is_connect() {
        return LmStudioError::RequestFailed(
            "LM Studio server is not running / connection refused".to_string(),
        );
    }

    LmStudioError::RequestFailed(error.to_string())
}
