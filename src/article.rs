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
