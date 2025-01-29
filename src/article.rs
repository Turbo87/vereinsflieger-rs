use crate::WithAccessToken;

pub async fn list_articles(
    client: &reqwest::Client,
    access_token: &str,
) -> anyhow::Result<Vec<Article>> {
    let params = WithAccessToken::new(access_token, &());

    let response = client
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

#[derive(Debug, serde::Deserialize)]
pub struct Article {
    /// Artikelnr
    #[serde(rename = "articleid")]
    pub article_id: String,

    /// Artikelbeschreibung
    pub designation: String,

    /// Artikeleinheit
    #[serde(rename = "unittype")]
    pub unit_type: String,

    /// Gebührenbereich
    #[serde(rename = "costtype")]
    pub cost_type: String,

    /// Spähre
    #[serde(rename = "spid")]
    pub sphere: String,

    /// Sachkonto
    pub account: String,

    /// Preise
    pub prices: Vec<Price>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Price {
    /// Gültig ab (`YYYY-MM-DD`)
    #[serde(rename = "validfrom")]
    pub valid_from: String,

    /// Gültig bis (`YYYY-MM-DD`)
    #[serde(rename = "validto")]
    pub valid_to: String,

    /// Mehrwertsteuer
    #[serde(rename = "salestax")]
    pub sales_tax: String,

    /// Bruttopreis
    #[serde(rename = "unitprice")]
    pub unit_price: String,
}
