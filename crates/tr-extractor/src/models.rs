#![cfg_attr(not(test), expect(unused))]

use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq, bon::Builder))]
pub struct AstroObject {
    #[serde(rename(deserialize = "ID"))]
    pub id: String,

    #[serde(default, rename(deserialize = "Entry"))]
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq, bon::Builder))]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Entry {
    #[serde(rename(deserialize = "ID"))]
    pub id: String,

    pub name: String,

    pub curiosity: Option<String>,

    #[serde(default)]
    #[cfg_attr(test, builder(default))]
    pub is_curiosity: bool,

    #[serde(default, rename(deserialize = "RumorFact"))]
    #[cfg_attr(test, builder(default))]
    pub rumor_facts: Vec<RumorFact>,

    #[serde(default, rename(deserialize = "ExploreFact"))]
    pub explore_facts: Vec<ExploreFact>,

    #[serde(default, rename(deserialize = "Entry"))]
    #[cfg_attr(test, builder(default))]
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq, bon::Builder))]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct RumorFact {
    #[serde(rename(deserialize = "ID"))]
    pub id: String,

    #[serde(rename(deserialize = "SourceID"))]
    pub source_id: Option<String>,

    // #[serde(rename(deserialize = "RumorName"))]
    // pub name: Option<String>,

    // #[serde(rename(deserialize = "RumorNamePriority"))]
    // pub name_priority: Option<u32>,

    pub text: String,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq, bon::Builder))]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct ExploreFact {
    #[serde(rename(deserialize = "ID"))]
    pub id: String,

    // pub clue_type: Option<String>,

    #[serde(default, deserialize_with = "bool_when_present")]
    #[cfg_attr(test, builder(default))]
    pub ignore_more_to_explore: bool,

    pub text: String,
}

/// Returns `true` if field present, but doesn't contain value "true"
fn bool_when_present<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<String>::deserialize(deserializer)?.is_some())
}
