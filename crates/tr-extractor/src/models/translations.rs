use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Translations {
    #[serde(rename(deserialize = "TranslationTableEntry"), alias = "TranslationTableEntryUI")]
    pub entries: Vec<Translation>,
}

#[derive(Debug, Deserialize)]
pub struct Translation {
    pub key: String,
    pub value: String,
}
