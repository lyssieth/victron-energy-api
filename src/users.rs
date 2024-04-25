use serde::Deserialize;
use serde_json::json;

use crate::{installations::Installation, Error, Failure, Victron, BASE_URL};

impl Victron {
    /// Adds a new site to the user. An email will be sent to the user with a link when the procedure is complete.
    ///
    /// # Errors
    /// - [`Error::Reqwest`] if there was an error sending the request.
    /// - [`Error::Victron`] if the request failed.
    pub async fn add_new_site(&self, identifier: &str) -> Result<String, Error> {
        let resp = self
            .client
            .post(format!(
                "{BASE_URL}/users/{}/addSite",
                self.ensure_user_id().await?
            ))
            .header("x-authorization", self.token.to_string())
            .json(&json!({ "siteIdentifier": identifier}))
            .send()
            .await?;

        if resp.status().is_success() {
            let success = resp.json::<AddNewSiteSuccess>().await?;

            return Ok(success.records.site_id);
        }

        let failure = resp.json::<Failure>().await?;

        Err(failure.into())
    }

    /// Retrieves a list of installations to which the user is connected. Normal users can only retrieve their own,
    /// and dealers can retrieve all installations of their linked customers, and admins those of all users.
    ///
    /// # Errors
    /// - [`Error::Reqwest`] if there was an error sending the request.
    /// - [`Error::Victron`] if the request failed.
    pub async fn get_all_installations_or_sites(
        &self,
        extended: bool,
    ) -> Result<Vec<Installation>, Error> {
        let resp = self
            .client
            .get(format!(
                "{BASE_URL}/users/{}/installations",
                self.ensure_user_id().await?
            ))
            .header("x-authorization", self.token.to_string())
            .json(&json!({
                "extended": i32::from(extended),
            }))
            .send()
            .await?;

        if resp.status().is_success() {
            let success = resp.json::<InstallationSuccess>().await?;

            return Ok(success.records);
        }

        let failure = resp.json::<Failure>().await?;

        Err(failure.into())
    }

    /// Retrieves a specific installation or site by its id. See [`get_all_installations_or_sites`] for more information.
    ///
    /// # Errors
    /// - [`Error::Reqwest`] if there was an error sending the request.
    /// - [`Error::Victron`] if the request failed.
    pub async fn get_installation_or_site(
        &self,
        extended: bool,
        site_id: i32,
    ) -> Result<Installation, Error> {
        let resp = self
            .client
            .get(format!(
                "{BASE_URL}/users/{}/installations",
                self.ensure_user_id().await?
            ))
            .header("x-authorization", self.token.to_string())
            .json(&json!({ "extended": i32::from(extended), "siteId": site_id }))
            .send()
            .await?;

        if resp.status().is_success() {
            let success = resp.json::<InstallationSuccess>().await?;

            return Ok(success.records[0].clone());
        }

        let failure = resp.json::<Failure>().await?;

        Err(failure.into())
    }

    /// Retrieves id, name, email and country of the user that is currently logged in
    ///
    /// # Errors
    /// - [`Error::Reqwest`] if there was an error sending the request.
    /// - [`Error::Victron`] if the request failed.
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
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserSuccess {
    pub success: bool,
    pub user: User,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub country: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddNewSiteSuccess {
    pub success: bool,
    pub records: AddNewSite,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddNewSite {
    pub site_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InstallationSuccess {
    pub success: bool,
    pub records: Vec<Installation>,
}
