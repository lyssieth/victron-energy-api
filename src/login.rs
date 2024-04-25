use std::{string::ToString, sync::Arc};

use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use tokio::sync::RwLock;

use crate::{Error, Failure, Token, Victron, BASE_URL};

#[derive(Debug, Clone, Deserialize)]
pub struct Success {
    pub token: Option<String>,
    #[serde(rename = "idUser")]
    pub user_id: i32,
    pub verification_mode: String,
    pub verification_sent: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DemoSuccess {
    pub token: Option<String>,
}

impl Victron {
    /// Logs into the Victron API.
    ///
    /// # Errors
    /// - `Error::Reqwest` if there was an error sending the request.
    /// - `Error::Victron` if the login failed, for example due to incorrect credentials.
    pub async fn login(
        username: &str,
        password: &str,
        sms_token: Option<&str>,
        remember_me: bool,
    ) -> Result<Self, Error> {
        let client = Client::new();

        let resp = client
            .post(format!("{BASE_URL}/auth/login"))
            .json(&json!({
                "username": username,
                "password": password,
                "sms_token": sms_token.map(ToString::to_string),
                "remember_me": remember_me,
            }))
            .send()
            .await?;

        if resp.status().is_success() {
            let success = resp.json::<Success>().await?;

            return Ok(Self {
                client,
                token: Token::Bearer(success.token.ok_or(Error::Victron(Failure {
                    error_code: Some("no_token".to_string()),
                    errors: json!("No token returned"),
                    success: false,
                }))?),
                user_id: Arc::new(RwLock::new(Some(success.user_id))),
            });
        }

        let failure = resp.json::<Failure>().await?;

        Err(failure.into())
    }

    /// Logs into the Victron API with an access token.
    ///
    /// # Errors
    /// - `Error::Reqwest` if there was an error sending the request.
    /// - `Error::Victron` if the login failed, for example due to incorrect credentials.
    pub async fn login_access_token(username: &str, access_token: &str) -> Result<Self, Error> {
        let client = Client::new();

        let resp = client
            .post(format!("{BASE_URL}/auth/login"))
            .json(&json!({
                "username": username,
                "password": access_token,
                "remember_me": true,
            }))
            .send()
            .await?;

        if resp.status().is_success() {
            let success = resp.json::<Success>().await?;

            return Ok(Self {
                client,
                token: Token::Access(success.token.ok_or(Error::Victron(Failure {
                    error_code: Some("no_token".to_string()),
                    errors: json!("No token returned"),
                    success: false,
                }))?),
                user_id: Arc::new(RwLock::new(Some(success.user_id))),
            });
        }

        let failure = resp.json::<Failure>().await?;

        Err(failure.into())
    }

    /// Logs into the Victron API as a demo user.
    ///
    /// # Errors
    /// - `Error::Reqwest` if there was an error sending the request.
    /// - `Error::Victron` if the login failed, for example due to incorrect credentials.
    pub async fn login_as_demo() -> Result<Self, Error> {
        let client = Client::new();

        let resp = client
            .post(format!("{BASE_URL}/auth/loginAsDemo"))
            .header("content-type", "application/json")
            .send()
            .await?;

        if resp.status().is_success() {
            let demo_success = resp.json::<DemoSuccess>().await?;

            return Ok(Self {
                client,
                token: Token::Bearer(demo_success.token.ok_or(Error::Victron(Failure {
                    error_code: Some("no_token".to_string()),
                    errors: json!("No token returned"),
                    success: false,
                }))?),
                user_id: Arc::default(),
            });
        }

        let failure = resp.json::<Failure>().await?;

        Err(failure.into())
    }
}
