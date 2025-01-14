mod article;
mod md5;
mod sale;
mod user;

pub use article::Article;
use md5::serialize_md5;
pub use sale::NewSale;
pub use user::User;

pub struct NoAccessToken;
pub struct AccessToken(String);
pub struct Authenticated(AccessToken);

pub trait AuthenticationState {}
impl AuthenticationState for NoAccessToken {}
impl AuthenticationState for AccessToken {}
impl AuthenticationState for Authenticated {}

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

    async fn get_access_token(self) -> anyhow::Result<Client<AccessToken>> {
        #[derive(Debug, serde::Deserialize)]
        struct Response {
            #[serde(rename = "accesstoken")]
            access_token: String,
        }

        let client = self.client;

        let response = client
            .get("https://www.vereinsflieger.de/interface/rest/auth/accesstoken")
            .send()
            .await?
            .error_for_status()?;

        let state = AccessToken(response.json::<Response>().await?.access_token);

        Ok(Client { client, state })
    }
}

impl Client<AccessToken> {
    pub async fn new_unauthenticated() -> anyhow::Result<Self> {
        Client::new_without_access_token().get_access_token().await
    }

    pub fn access_token(&self) -> &str {
        &self.state.0
    }

    pub async fn authenticate(
        self,
        params: &Credentials<'_>,
    ) -> anyhow::Result<Client<Authenticated>> {
        let params = WithAccessToken {
            access_token: self.access_token(),
            params,
        };

        self.client
            .post("https://www.vereinsflieger.de/interface/rest/auth/signin")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(serde_urlencoded::to_string(params).unwrap())
            .send()
            .await?
            .error_for_status()?;

        let client = self.client;
        let state = Authenticated(self.state);
        Ok(Client { client, state })
    }
}

impl Client<Authenticated> {
    pub async fn new(params: &Credentials<'_>) -> anyhow::Result<Self> {
        Client::new_unauthenticated()
            .await?
            .authenticate(params)
            .await
    }

    pub fn access_token(&self) -> &str {
        &self.state.0 .0
    }

    fn with_access_token<'a, T>(&'a self, params: &'a T) -> WithAccessToken<'a, T> {
        WithAccessToken {
            access_token: self.access_token(),
            params,
        }
    }

    pub async fn list_users(&self) -> anyhow::Result<Vec<User>> {
        let params = self.with_access_token(&());

        let response = self
            .client
            .post("https://www.vereinsflieger.de/interface/rest/user/list")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(serde_urlencoded::to_string(params).unwrap())
            .send()
            .await?
            .error_for_status()?;

        response
            .json::<serde_json::Map<String, serde_json::Value>>()
            .await?
            .into_iter()
            .filter(|(k, _)| k.parse::<usize>().is_ok())
            .map(|(_, v)| serde_path_to_error::deserialize(v))
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub async fn list_articles(&self) -> anyhow::Result<Vec<Article>> {
        let params = self.with_access_token(&());

        let response = self
            .client
            .post("https://www.vereinsflieger.de/interface/rest/articles/list")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(serde_urlencoded::to_string(params).unwrap())
            .send()
            .await?
            .error_for_status()?;

        response
            .json::<serde_json::Map<String, serde_json::Value>>()
            .await?
            .into_iter()
            .filter(|(k, _)| k.parse::<usize>().is_ok())
            .map(|(_, v)| serde_path_to_error::deserialize(v))
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub async fn add_sale(&self, new_sale: &NewSale<'_>) -> anyhow::Result<()> {
        let params = self.with_access_token(new_sale);

        self.client
            .post("https://www.vereinsflieger.de/interface/rest/sale/add")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(serde_urlencoded::to_string(params).unwrap())
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[derive(serde::Serialize)]
struct WithAccessToken<'a, T> {
    #[serde(rename = "accesstoken")]
    access_token: &'a str,

    #[serde(flatten)]
    params: &'a T,
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
