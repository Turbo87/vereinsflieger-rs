use crate::error::error_for_status;
use crate::WithAccessToken;

pub async fn list_articles(
    client: &reqwest::Client,
    access_token: &str,
) -> crate::Result<Vec<Article>> {
    let params = WithAccessToken::new(access_token, &());

    let response = client
        .post("https://www.vereinsflieger.de/interface/rest/articles/list")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(serde_urlencoded::to_string(params)?)
        .send()
        .await?;

    let bytes = error_for_status(response).await?.bytes().await?;

    let json: serde_json::Map<String, serde_json::Value> =
        serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_slice(&bytes))?;

    json.into_iter()
        .filter(|(k, _)| k.parse::<usize>().is_ok())
        .map(|(_, v)| serde_path_to_error::deserialize(v).map_err(crate::Error::from))
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}

#[derive(Debug, serde::Deserialize)]
pub struct Article {
    /// Artikelnr
    #[serde(
        rename = "articleid",
        deserialize_with = "crate::utils::serde::unescape"
    )]
    pub article_id: String,

    /// Artikelbezeichnung
    #[serde(deserialize_with = "crate::utils::serde::unescape")]
    pub designation: String,

    /// Artikelbeschreibung
    #[serde(default, deserialize_with = "crate::utils::serde::unescape")]
    pub description: String,

    /// Artikeleinheit
    #[serde(
        rename = "unittype",
        deserialize_with = "crate::utils::serde::unescape"
    )]
    pub unit_type: String,

    /// Gebührenbereich
    #[serde(
        rename = "costtype",
        deserialize_with = "crate::utils::serde::unescape"
    )]
    pub cost_type: String,

    /// Spähre
    #[serde(rename = "spid", deserialize_with = "crate::utils::serde::unescape")]
    pub sphere: String,

    /// Sachkonto
    #[serde(deserialize_with = "crate::utils::serde::unescape")]
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
