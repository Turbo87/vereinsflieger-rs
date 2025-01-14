#[derive(Debug, serde::Deserialize)]
pub struct User {
    /// Interne ID
    #[serde(rename = "uid")]
    pub user_id: String,
    /// Titel
    pub title: String,
    /// Vorname
    #[serde(rename = "firstname")]
    pub first_name: String,
    /// Nachname
    #[serde(rename = "lastname")]
    pub last_name: String,
    /// Spitzname
    pub nickname: String,
    /// Geschlecht (m, w)
    pub gender: String,

    /// Straße
    pub street: String,
    /// Postfach
    #[serde(rename = "postofficebox")]
    pub post_office_box: String,
    /// Adresszusatz
    pub careof: String,
    /// Postleitzahl
    pub zipcode: String,
    /// Ort
    pub town: String,
    /// Land
    pub country: String,

    /// Geburtsdatum (dd.mm.yyyy)
    pub birthday: String,
    /// Geburtsort
    pub birthplace: String,

    /// Mailadresse
    pub email: String,
    /// Telefon (privat)
    #[serde(rename = "homenumber")]
    pub home_number: String,
    /// Mobil (privat)
    #[serde(rename = "mobilenumber")]
    pub mobile_number: String,
    /// Telefon (gesch.)
    #[serde(rename = "phonenumber")]
    pub work_number: String,
    /// Mobil (gesch.)
    #[serde(rename = "phonenumber2")]
    pub work_mobile_number: String,

    /// Autokennzeichen
    #[serde(rename = "carlicenseplate")]
    pub car_licenseplate: String,
    /// Ausweisnummer
    pub identification: String,
    /// NatoID
    #[serde(rename = "natoid")]
    pub nato_id: String,
    /// Führungszeugnis
    pub policecert_validto: String,

    /// Notfallkontakt 1
    pub ice_contact1: String,
    /// Notfallkontakt 2
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
    #[serde(rename = "memberstatus")]
    pub member_status: String,

    /// Briefanrede
    #[serde(rename = "lettertitle")]
    pub letter_title: String,
    /// Rundmailempfänger
    pub mailrecipient: String,

    /// Nicht beendete Lehrpläne
    pub educations: Vec<String>,

    /// Zugeordnete Rollen
    pub roles: Vec<String>,
    /// Zugeordnete Sparten
    pub sector: Vec<String>,
    /// Zugeordnete Funktionen
    pub functions: Vec<String>,

    /// Liste der zugeordneten Schlüssel
    pub keymanagement: Vec<Key>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Key {
    pub title: String,
    #[serde(rename = "keyname")]
    pub name: String,
}
