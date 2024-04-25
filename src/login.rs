use std::string::ToString;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{Error, Failure, Token, Victron, BASE_URL};

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub username: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_token: Option<String>,
    #[serde(skip_serializing_if = "not")]
    pub remember_me: bool,
}

#[allow(clippy::trivially_copy_pass_by_ref)] // damn you, serde.
const fn not(value: &bool) -> bool {
    !*value
}

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
            .json(&Request {
                username: username.to_string(),
                password: password.to_string(),
                sms_token: sms_token.map(ToString::to_string),
                remember_me,
            })
            .send()
            .await?;

        if resp.status().is_success() {
            let success = resp.json::<Success>().await?;

            return Ok(Self {
                client,
                token: Token::Bearer(success.token.ok_or(Error::Victron(Failure {
                    error_code: Some("no_token".to_string()),
                    errors: json! { vec!["No token returned".to_string()] },
                    success: false,
                }))?),
                user_id: Some(success.user_id),
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
                    errors: json! { vec!["No token returned".to_string()] },
                    success: false,
                }))?),
                user_id: None,
            });
        }

        let failure = resp.json::<Failure>().await?;

        Err(failure.into())
    }
}
