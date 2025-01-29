use crate::WithAccessToken;

pub async fn list_users(client: &reqwest::Client, access_token: &str) -> anyhow::Result<Vec<User>> {
    let params = WithAccessToken::new(access_token, &());

    let response = client
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

#[derive(Debug, serde::Deserialize)]
pub struct User {
    /// Interne ID
    #[serde(rename = "uid")]
    pub user_id: String,
    /// Titel
    #[serde(deserialize_with = "crate::utils::serde::unescape")]
    pub title: String,
    /// Vorname
    #[serde(
        rename = "firstname",
        deserialize_with = "crate::utils::serde::unescape"
    )]
    pub first_name: String,
    /// Nachname
    #[serde(
        rename = "lastname",
        deserialize_with = "crate::utils::serde::unescape"
    )]
    pub last_name: String,
    /// Spitzname
    #[serde(deserialize_with = "crate::utils::serde::unescape")]
    pub nickname: String,
    /// Geschlecht (m, w)
    pub gender: String,

    /// Straße
    #[serde(deserialize_with = "crate::utils::serde::unescape")]
    pub street: String,
    /// Postfach
    #[serde(
        rename = "postofficebox",
        deserialize_with = "crate::utils::serde::unescape"
    )]
    pub post_office_box: String,
    /// Adresszusatz
    #[serde(deserialize_with = "crate::utils::serde::unescape")]
    pub careof: String,
    /// Postleitzahl
    #[serde(deserialize_with = "crate::utils::serde::unescape")]
    pub zipcode: String,
    /// Ort
    #[serde(deserialize_with = "crate::utils::serde::unescape")]
    pub town: String,
    /// Land
    #[serde(deserialize_with = "crate::utils::serde::unescape")]
    pub country: String,

    /// Geburtsdatum (dd.mm.yyyy)
    pub birthday: String,
    /// Geburtsort
    #[serde(deserialize_with = "crate::utils::serde::unescape")]
    pub birthplace: String,

    /// Mailadresse
    #[serde(deserialize_with = "crate::utils::serde::unescape")]
    pub email: String,
    /// Telefon (privat)
    #[serde(
        rename = "homenumber",
        deserialize_with = "crate::utils::serde::unescape"
    )]
    pub home_number: String,
    /// Mobil (privat)
    #[serde(
        rename = "mobilenumber",
        deserialize_with = "crate::utils::serde::unescape"
    )]
    pub mobile_number: String,
    /// Telefon (gesch.)
    #[serde(
        rename = "phonenumber",
        deserialize_with = "crate::utils::serde::unescape"
    )]
    pub work_number: String,
    /// Mobil (gesch.)
    #[serde(
        rename = "phonenumber2",
        deserialize_with = "crate::utils::serde::unescape"
    )]
    pub work_mobile_number: String,

    /// Autokennzeichen
    #[serde(
        rename = "carlicenseplate",
        deserialize_with = "crate::utils::serde::unescape"
    )]
    pub car_licenseplate: String,
    /// Ausweisnummer
    #[serde(deserialize_with = "crate::utils::serde::unescape")]
    pub identification: String,
    /// NatoID
    #[serde(rename = "natoid", deserialize_with = "crate::utils::serde::unescape")]
    pub nato_id: String,
    /// Führungszeugnis
    pub policecert_validto: String,

    /// Notfallkontakt 1
    #[serde(deserialize_with = "crate::utils::serde::unescape")]
    pub ice_contact1: String,
    /// Notfallkontakt 2
    #[serde(deserialize_with = "crate::utils::serde::unescape")]
    pub ice_contact2: String,

    /// Mitgliedsnummer
    #[serde(rename = "memberid")]
    pub member_id: String,
    /// Eintrittsdatum
    #[serde(rename = "memberbegin")]
    pub member_begin: String,
    /// Ausstrittsdatum
    #[serde(rename = "memberend")]
    pub member_end: String,
    /// Mitgliedsstatus
    #[serde(
        rename = "memberstatus",
        deserialize_with = "crate::utils::serde::unescape"
    )]
    pub member_status: String,

    /// Briefanrede
    #[serde(
        rename = "lettertitle",
        deserialize_with = "crate::utils::serde::unescape"
    )]
    pub letter_title: String,
    /// Rundmailempfänger
    pub mailrecipient: String,

    /// Nicht beendete Lehrpläne
    #[serde(deserialize_with = "crate::utils::serde::unescape_vec")]
    pub educations: Vec<String>,

    /// Zugeordnete Rollen
    #[serde(deserialize_with = "crate::utils::serde::unescape_vec")]
    pub roles: Vec<String>,
    /// Zugeordnete Sparten
    #[serde(deserialize_with = "crate::utils::serde::unescape_vec")]
    pub sector: Vec<String>,
    /// Zugeordnete Funktionen
    #[serde(deserialize_with = "crate::utils::serde::unescape_vec")]
    pub functions: Vec<String>,

    /// Liste der zugeordneten Schlüssel
    pub keymanagement: Vec<Key>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Key {
    #[serde(deserialize_with = "crate::utils::serde::unescape")]
    pub title: String,
    #[serde(rename = "keyname", deserialize_with = "crate::utils::serde::unescape")]
    pub name: String,
}
