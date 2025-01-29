use crate::error::error_for_status;
use crate::WithAccessToken;

pub async fn add_sale(
    client: &reqwest::Client,
    access_token: &str,
    new_sale: &NewSale<'_>,
) -> crate::Result<()> {
    let params = WithAccessToken::new(access_token, new_sale);

    let response = client
        .post("https://www.vereinsflieger.de/interface/rest/sale/add")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(serde_urlencoded::to_string(params)?)
        .send()
        .await?;

    error_for_status(response).await?;

    Ok(())
}

#[derive(Debug, serde::Serialize)]
pub struct NewSale<'a> {
    /// Datum der Buchung (`YYYY-mm-dd`)
    #[serde(rename = "bookingdate")]
    pub booking_date: &'a str,

    /// Artikelnummer
    #[serde(rename = "articleid")]
    pub article_id: &'a str,

    /// Menge
    pub amount: f64,

    /// Mitgliedsnummer des Käufers
    #[serde(rename = "memberid")]
    pub member_id: Option<u32>,

    /// Callsign bzw. Verwendung
    pub callsign: Option<&'a str>,

    /// Steuer
    #[serde(rename = "salestax")]
    pub sales_tax: Option<f64>,

    /// Bruttopreis
    #[serde(rename = "totalprice")]
    pub total_price: Option<f64>,

    /// Zählerstand
    pub counter: Option<f64>,

    /// Kommentar
    pub comment: Option<&'a str>,

    /// Gebührenbereich
    #[serde(rename = "costtype")]
    pub cost_type: Option<&'a str>,

    /// id des Habenkontos (muss ein Aufwand- oder Ertragskonto sein)
    pub caid2: Option<u32>,

    /// Sphäre
    pub spid: Option<u32>,
}
