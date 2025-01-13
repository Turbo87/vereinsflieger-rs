pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        Self { client }
    }

    pub async fn get_access_token(&self) -> Result<String, reqwest::Error> {
        #[derive(Debug, serde::Deserialize)]
        struct Response {
            #[serde(rename = "accesstoken")]
            access_token: String,
        }

        let response = self
            .client
            .get("https://www.vereinsflieger.de/interface/rest/auth/accesstoken")
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json::<Response>().await?.access_token)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
