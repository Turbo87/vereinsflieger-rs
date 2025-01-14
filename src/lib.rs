use md5::{Digest, Md5};

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
    pub async fn new() -> Result<Self, reqwest::Error> {
        Client::new_without_access_token().get_access_token().await
    }

    pub fn access_token(&self) -> &str {
        &self.state.0
    }

    pub async fn sign_in(
        self,
        club_id: u32,
        username: &str,
        password: &str,
        app_key: &str,
    ) -> Result<Client<Authenticated>, reqwest::Error> {
        let hashed_password = md5(password);

        let url = "https://www.vereinsflieger.de/interface/rest/auth/signin";
        let mut url = reqwest::Url::parse(url).unwrap();

        url.query_pairs_mut()
            .append_pair("accesstoken", self.access_token())
            .append_pair("cid", &club_id.to_string())
            .append_pair("appkey", app_key)
            .append_pair("username", username)
            .append_pair("password", &hashed_password);

        let query = url.query().unwrap().to_string();

        url.set_query(None);

        let client = self.client;

        client
            .post(url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(query)
            .send()
            .await?
            .error_for_status()?;

        let state = Authenticated(self.state);
        Ok(Client { client, state })
    }
}

impl Client<Authenticated> {
    pub fn access_token(&self) -> &str {
        &self.state.0 .0
    }
}

fn md5(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}
