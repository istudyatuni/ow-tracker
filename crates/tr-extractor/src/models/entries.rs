use std::fmt::Debug;

use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, bon::Builder))]
pub struct AstroObject<Entry>
where
    Entry: Debug,
    Vec<Entry>: DeserializeOwned,
{
    #[serde(rename(deserialize = "ID"))]
    pub id: String,

    #[serde(
        default,
        rename(deserialize = "Entry"),
        skip_serializing_if = "Vec::is_empty"
    )]
    pub entries: Vec<Entry>,
}

#[derive(Debug, Default, Deserialize)]
#[cfg_attr(test, derive(PartialEq, bon::Builder))]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct XmlEntry {
    #[serde(rename(deserialize = "ID"))]
    pub id: String,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub curiosity: Option<String>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    #[cfg_attr(test, builder(default))]
    pub is_curiosity: bool,

    #[serde(default, rename(deserialize = "RumorFact"))]
    #[cfg_attr(test, builder(default))]
    pub rumor_facts: Vec<RumorFact>,

    #[serde(default, rename(deserialize = "ExploreFact"))]
    pub explore_facts: Vec<ExploreFact>,

    #[serde(default, rename(deserialize = "Entry"))]
    #[cfg_attr(test, builder(default))]
    pub entries: Vec<XmlEntry>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JsonEntry {
    pub id: String,

    pub name: String,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_curiosity: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub curiosity: Option<String>,

    pub facts: JsonEntryFacts,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<JsonEntry>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JsonEntryFacts {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub explore: Vec<ExploreFact>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rumor: Vec<RumorFact>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, bon::Builder))]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct RumorFact {
    #[serde(rename(deserialize = "ID"))]
    pub id: String,

    #[serde(
        rename(deserialize = "SourceID"),
        skip_serializing_if = "Option::is_none"
    )]
    pub source_id: Option<String>,

    // #[serde(rename(deserialize = "RumorName"))]
    // pub name: Option<String>,

    // #[serde(rename(deserialize = "RumorNamePriority"))]
    // pub name_priority: Option<u32>,

    //
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, bon::Builder))]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct ExploreFact {
    #[serde(rename(deserialize = "ID"))]
    pub id: String,

    // pub clue_type: Option<String>,

    //
    #[serde(
        default,
        deserialize_with = "bool_when_present",
        skip_serializing_if = "std::ops::Not::not"
    )]
    #[cfg_attr(test, builder(default))]
    pub ignore_more_to_explore: bool,

    pub text: String,
}

impl From<AstroObject<XmlEntry>> for AstroObject<JsonEntry> {
    fn from(value: AstroObject<XmlEntry>) -> Self {
        Self {
            id: value.id,
            entries: value.entries.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<XmlEntry> for JsonEntry {
    fn from(value: XmlEntry) -> Self {
        Self {
            id: value.id,
            name: value.name,
            curiosity: value.curiosity,
            is_curiosity: value.is_curiosity,
            facts: JsonEntryFacts {
                rumor: value.rumor_facts,
                explore: value.explore_facts,
            },
            entries: value.entries.into_iter().map(Into::into).collect(),
        }
    }
}

/// Returns `true` if field present, but doesn't contain value "true"
fn bool_when_present<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<String>::deserialize(deserializer)?.is_some())
}
