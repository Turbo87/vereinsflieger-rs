use html_escape::decode_html_entities;
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;

pub fn unescape<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Cow::<'de, str>::deserialize(deserializer).map(|cow| decode_html_entities(&cow).into_owned())
}

pub fn unescape_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Vec::<Cow<'de, str>>::deserialize(deserializer)?
        .into_iter()
        .map(|cow| decode_html_entities(&cow).into_owned())
        .collect())
}
