pub struct Client {
    #[allow(dead_code)]
    client: reqwest::Client,
    access_token: String,
}

impl Client {
    pub async fn new() -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::new();
        let access_token = get_access_token(&client).await?;
        Ok(Self {
            client,
            access_token,
        })
    }

    pub fn access_token(&self) -> &str {
        &self.access_token
    }
}

async fn get_access_token(client: &reqwest::Client) -> Result<String, reqwest::Error> {
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
