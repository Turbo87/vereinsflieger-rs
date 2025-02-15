mod article;
mod client;
mod error;
mod md5;
mod sale;
mod user;
mod utils;

pub use crate::client::Client;
pub use article::{list_articles, Article, Price};
pub use error::{Error, Result};
use md5::serialize_md5;
pub use sale::{add_sale, NewSale};
use std::fmt::{Debug, Formatter};
pub use user::{list_users, Key, User};

pub async fn get_access_token(client: &reqwest::Client) -> Result<String> {
    #[derive(Debug, serde::Deserialize)]
    struct Response {
        #[serde(rename = "accesstoken")]
        access_token: String,
    }

    let response = client
        .get("https://www.vereinsflieger.de/interface/rest/auth/accesstoken")
        .send()
        .await?
        .error_for_status()?;

    Ok(response.json::<Response>().await?.access_token)
}

pub async fn authenticate(
    client: &reqwest::Client,
    access_token: &str,
    credentials: &Credentials,
) -> Result<()> {
    let params = WithAccessToken::new(access_token, credentials);

    client
        .post("https://www.vereinsflieger.de/interface/rest/auth/signin")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(serde_urlencoded::to_string(params)?)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

#[derive(serde::Serialize)]
struct WithAccessToken<'a, T> {
    #[serde(rename = "accesstoken")]
    access_token: &'a str,

    #[serde(flatten)]
    params: &'a T,
}

impl<'a, T> WithAccessToken<'a, T> {
    pub fn new(access_token: &'a str, params: &'a T) -> Self {
        Self {
            access_token,
            params,
        }
    }
}

#[derive(serde::Serialize)]
pub struct Credentials {
    /// Eindeutige Nummer des Vereins
    ///
    /// Hinweis: Die eindeutige cid wird nur benötigt, wenn der Benutzer in
    // mehreren Vereinen existiert.
    #[serde(rename = "cid")]
    pub club_id: Option<u32>,

    /// Eindeutiger Applikationsschlüssel
    #[serde(rename = "appkey")]
    pub app_key: String,

    /// Benuztername oder E-Mail-Adresse
    pub username: String,

    /// Passwort
    #[serde(serialize_with = "serialize_md5")]
    pub password: String,

    /// Zwei-Faktor-Authentifizierung
    pub auth_secret: Option<String>,
}

impl Debug for Credentials {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credentials")
            .field("club_id", &self.club_id)
            .field("app_key", &self.app_key)
            .field("username", &self.username)
            .field("password", &"********")
            .field("auth_secret", &"********")
            .finish()
    }
}
