mod article;
mod error;
mod md5;
mod sale;
mod user;
mod utils;

pub use article::{list_articles, Article};
pub use error::{Error, Result};
use md5::serialize_md5;
pub use sale::{add_sale, NewSale};
use std::fmt::{Debug, Formatter};
pub use user::{list_users, User};

pub struct NoAccessToken;
pub struct AccessToken(String);
pub struct Authenticated(AccessToken);

pub trait AuthenticationState {}
impl AuthenticationState for NoAccessToken {}
impl AuthenticationState for AccessToken {}
impl AuthenticationState for Authenticated {}

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
    credentials: &Credentials<'_>,
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

pub struct Client<S: AuthenticationState> {
    #[allow(dead_code)]
    client: reqwest::Client,
    state: S,
}

impl Client<NoAccessToken> {
    fn new_without_access_token() -> Self {
        let client = reqwest::Client::new();
        let state = NoAccessToken;
        Self { client, state }
    }

    async fn get_access_token(self) -> Result<Client<AccessToken>> {
        let client = self.client;
        get_access_token(&client).await.map(|access_token| {
            let state = AccessToken(access_token);
            Client { client, state }
        })
    }
}

impl Client<AccessToken> {
    pub async fn new_unauthenticated() -> Result<Self> {
        Client::new_without_access_token().get_access_token().await
    }

    pub fn access_token(&self) -> &str {
        &self.state.0
    }

    pub async fn authenticate(self, params: &Credentials<'_>) -> Result<Client<Authenticated>> {
        let client = self.client;
        let state = self.state;

        let access_token = &state.0;
        authenticate(&client, access_token, params).await?;

        let state = Authenticated(state);
        Ok(Client { client, state })
    }
}

impl Client<Authenticated> {
    pub async fn new(params: &Credentials<'_>) -> Result<Self> {
        Client::new_unauthenticated()
            .await?
            .authenticate(params)
            .await
    }

    pub fn access_token(&self) -> &str {
        &self.state.0 .0
    }

    pub async fn list_users(&self) -> Result<Vec<User>> {
        list_users(&self.client, self.access_token()).await
    }

    pub async fn list_articles(&self) -> Result<Vec<Article>> {
        list_articles(&self.client, self.access_token()).await
    }

    pub async fn add_sale(&self, new_sale: &NewSale<'_>) -> Result<()> {
        add_sale(&self.client, self.access_token(), new_sale).await
    }
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
pub struct Credentials<'a> {
    /// Eindeutige Nummer des Vereins
    ///
    /// Hinweis: Die eindeutige cid wird nur benötigt, wenn der Benutzer in
    // mehreren Vereinen existiert.
    #[serde(rename = "cid")]
    pub club_id: Option<u32>,

    /// Eindeutiger Applikationsschlüssel
    #[serde(rename = "appkey")]
    pub app_key: &'a str,

    /// Benuztername oder E-Mail-Adresse
    pub username: &'a str,

    /// Passwort
    #[serde(serialize_with = "serialize_md5")]
    pub password: &'a str,

    /// Zwei-Faktor-Authentifizierung
    pub auth_secret: Option<&'a str>,
}

impl Debug for Credentials<'_> {
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
