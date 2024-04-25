use serde::Deserialize;

use crate::{Error, Failure, Victron, BASE_URL};

impl Victron {
    /// Retrieves id, name, email and country of the user that is currently logged in
    ///
    /// # Errors
    /// - `Error::Reqwest` if there was an error sending the request.
    /// - `Error::Victron` if the request failed.
    pub async fn get_user_info(&self) -> Result<User, Error> {
        let resp = self
            .client
            .get(format!("{BASE_URL}/users/me"))
            .header("x-authorization", self.token.to_string())
            .header("content-type", "application/json")
            .send()
            .await?;

        if resp.status().is_success() {
            let success = resp.json::<UserSuccess>().await?;

            return Ok(success.user);
        }

        let failure = resp.json::<Failure>().await?;

        Err(failure.into())
    }

    pub async fn get_all_installations_or_sites(&self) -> Result<(), Error> {
        todo!("Implement get_all_installations_or_sites")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserSuccess {
    pub success: bool,
    pub user: User,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub country: String,
}
