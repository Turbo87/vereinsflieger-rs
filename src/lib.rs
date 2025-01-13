use md5::{Digest, Md5};

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

    pub async fn sign_in(
        &self,
        club_id: u32,
        username: &str,
        password: &str,
        app_key: &str,
    ) -> Result<(), reqwest::Error> {
        let hashed_password = md5(password);

        let url = "https://www.vereinsflieger.de/interface/rest/auth/signin";
        let mut url = reqwest::Url::parse(url).unwrap();

        url.query_pairs_mut()
            .append_pair("accesstoken", &self.access_token)
            .append_pair("cid", &club_id.to_string())
            .append_pair("appkey", app_key)
            .append_pair("username", username)
            .append_pair("password", &hashed_password);

        let query = url.query().unwrap().to_string();

        url.set_query(None);

        self.client
            .post(url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(query)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
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

fn md5(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}
