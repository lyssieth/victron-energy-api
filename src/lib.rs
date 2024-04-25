#![warn(clippy::pedantic, clippy::nursery)]
#![deny(clippy::unwrap_used)]

use std::sync::Arc;

use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::RwLock;

pub mod installations;
pub mod login;
pub mod users;

const BASE_URL: &str = "https://vrmapi.victronenergy.com/v2";

#[derive(Clone)]
pub struct Victron {
    /// A [`Client`] used to send requests.
    client: Client,

    /// The token used to authenticate requests.
    /// This is either a Bearer token or an Access token, depending on the endpoint.
    token: Token,
    /// The user id of the currently logged in user.
    ///
    /// This is fetched when needed and stored in a [`RefCell`] to allow for mutable access.
    /// The [`RefCell`] is ideally updated only once, when any function that needs the user id is called.
    user_id: Arc<RwLock<Option<i32>>>,
}

impl Victron {
    /// Gets the user id of the currently logged in user, fetching it if it hasn't been fetched yet.
    ///
    /// # Errors
    /// - [`Error::Reqwest`] if there was an error sending the request.
    /// - [`Error::Victron`] if the request failed.
    /// - [`Error::ParseInt`] if the user id could not be parsed as an integer.
    pub async fn ensure_user_id(&self) -> Result<i32, Error> {
        if let Some(user_id) = self.user_id.read().await.as_ref() {
            return Ok(*user_id);
        }

        let user = self.get_user_info().await?;

        self.user_id.write().await.replace(user.id.parse()?);

        Ok(user.id.parse()?)
    }
}

#[derive(Clone)]
pub(crate) enum Token {
    Bearer(String),
    Access(String),
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Self::Bearer(token) => format!("Bearer {token}"),
            Self::Access(token) => format!("Token {token}"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Reqwest Error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Failed to parse integer: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

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
