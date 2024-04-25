#![warn(clippy::pedantic, clippy::nursery)]
#![deny(clippy::unwrap_used)]

use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

mod login;
mod users;

const BASE_URL: &str = "https://vrmapi.victronenergy.com/v2";

#[derive(Clone)]
pub struct Victron {
    client: Client,

    token: Token,
    user_id: Option<i32>,
}

#[derive(Clone)]
pub(crate) enum Token {
    Bearer(String),
    Other(String),
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Self::Bearer(token) => format!("Bearer {token}"),
            Self::Other(token) => format!("Token {token}"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Reqwest Error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Victron Error: {0:?}")]
    Victron(Failure),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Failure {
    pub success: bool,
    pub errors: Value,
    pub error_code: Option<String>,
}

impl From<Failure> for Error {
    fn from(failure: Failure) -> Self {
        Self::Victron(failure)
    }
}
