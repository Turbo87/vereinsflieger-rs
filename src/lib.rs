mod md5;

use md5::serialize_md5;

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

    async fn get_access_token(self) -> Result<Client<AccessToken>, reqwest::Error> {
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
    pub async fn new_unauthenticated() -> Result<Self, reqwest::Error> {
        Client::new_without_access_token().get_access_token().await
    }

    pub fn access_token(&self) -> &str {
        &self.state.0
    }

    pub async fn authenticate(
        self,
        params: &Credentials<'_>,
    ) -> Result<Client<Authenticated>, reqwest::Error> {
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
    pub async fn new(params: &Credentials<'_>) -> Result<Self, reqwest::Error> {
        Client::new_unauthenticated()
            .await?
            .authenticate(params)
            .await
    }

    pub fn access_token(&self) -> &str {
        &self.state.0 .0
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
