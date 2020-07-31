use thiserror::Error as ThisError;
use anyhow::Error as AnyHowError;
use serde::Serialize;
use serde_json::json;
use isahc::prelude::*;
use crate::pollen::PollenParseError;

pub type Result<T> = anyhow::Result<T>;

#[derive(ThisError, Debug)]
pub enum FlowerError {
    #[error("An error occurred `{0}`")]
    SimpleError(String),
}

impl From<PollenParseError> for FlowerError {
    fn from(e: PollenParseError) -> Self {
        Self::SimpleError(e.to_string())
    }
}

#[derive(Default, Serialize)]
struct ErrorMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    value1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value3: Option<String>,
}

impl From<&AnyHowError> for ErrorMessage {
    fn from(error: &AnyHowError) -> Self {
        match error {
            _ => ErrorMessage {
                value1: Some(format!("{:?}", error)),
                ..Default::default()
            }
        }
    }
}

pub struct ErrorHandler {
    hook_uri: String,
}

impl ErrorHandler {
    pub fn new(ifttt_key: &str) -> ErrorHandler {
        ErrorHandler {
            hook_uri: format!("https://maker.ifttt.com/trigger/flower/with/key/{}", ifttt_key),
        }
    }

    fn send_message(&self, message: ErrorMessage) -> Result<()> {
        Request::post(&self.hook_uri)
            .header("Content-Type", "application/json")
            .body(json!(message).to_string())?
            .send()?;
        Ok(())
    }

    pub fn handle_error(&self, error: &AnyHowError) {
        let message: ErrorMessage = error.into();
        if let Err(send_error) = self.send_message(message) {
            eprintln!("{}", send_error);
        }
        eprintln!("{}", error);
    }
}
