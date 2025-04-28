use std::{collections::HashMap, fs::File, path::PathBuf};

use anyhow::{Result, anyhow};
use clap::Parser;
use memmap2::{Mmap, MmapOptions};
use tracing::{Level, debug, info, warn};
use tracing_subscriber::FmtSubscriber;

use info::Lang;
use models::{
    entries::{AstroObject, JsonEntry, XmlEntry},
    translations::{Translation, Translations},
};

mod args;
mod info;
mod models;

const ASTRO_OBJECT_START: &[u8] = b"<AstroObjectEntry>";
const ASTRO_OBJECT_END: &[u8] = b"</AstroObjectEntry>";
const SHARED_FILE: &str = "sharedassets1.assets";
/// Offset in file `sharedassets1.assets` for game version 1.1.15. Probably
/// unnecessary since search seems to be pretty fast
const V15_SHARED_OFFSET: u64 = 930000000;

const TR_SHIPLOG_START: &[u8] = b"<table_shipLog>";
const TR_SHIPLOG_END: &[u8] = b"</table_shipLog>";
const RES_FILE: &str = "resources.assets";

/// Order of `TranslationTable_XML`s in `resources.assets` for game version 1.1.15
const V15_LANG_ORDER: &[Lang] = &[
    Lang::SpanishLa,
    Lang::English,
    Lang::Turkish,
    Lang::PortugueseBr,
    Lang::Italian,
    Lang::French,
    Lang::Polish,
    Lang::Korean,
    Lang::ChineseSimple,
    Lang::German,
    Lang::Russian,
    Lang::Japanese,
];

fn main() -> Result<()> {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .without_time()
            .with_target(false)
            .with_max_level(Level::DEBUG)
            .finish(),
    )?;

    let args = args::Cli::parse();

    let dir = find_data_dir()?;
    let astro_objects = load_astro_objects(File::open(dir.join(SHARED_FILE))?)?;
    let tr_objects = load_tr_objects(File::open(dir.join(RES_FILE))?)?;

    debug!("count of astro objects: {}", astro_objects.len());
    debug!(
        "count of astro entries: {}",
        astro_objects
            .iter()
            .map(|a| count_entries(&a.entries))
            .sum::<u32>()
    );
    debug!(
        "count of translation entries: {}",
        tr_objects[0].entries.len()
    );

    // save info about astro objects
    if args.write {
        let output = PathBuf::from("output/entries.json");
        info!("writing {}", output.display());
        serde_json::to_writer(File::create(output)?, &astro_objects)?;
    }

    // names and ids of astro objects for searching in translations
    let mut astro_names_keys = HashMap::<String, Vec<String>>::with_capacity(100);
    for a in &astro_objects {
        for (name, id) in collect_astro_names(&a.entries) {
            astro_names_keys
                .entry(name)
                .and_modify(|ids| ids.push(id.clone()))
                .or_insert_with(|| vec![id]);
        }
    }
    debug!("count of astro names: {}", astro_names_keys.len());

    // keys for texts
    let mut astro_facts = HashMap::<String, Vec<String>>::with_capacity(400);
    for a in &astro_objects {
        for (text, id) in collect_astro_texts(&a.entries) {
            astro_facts
                .entry(text)
                .and_modify(|ids| ids.push(id.clone()))
                .or_insert_with(|| vec![id]);
        }
    }
    debug!("count of astro texts: {}", astro_facts.len());

    // remap translations
    let mut translations = HashMap::new();
    let astro_names = astro_names_keys.keys().map(ToOwned::to_owned).collect();
    for (lang, tr) in clean_translations(tr_objects, astro_names)? {
        let mut translation = HashMap::with_capacity(tr.len());
        for (text, ids) in &astro_facts {
            for id in ids {
                translation.insert(
                    id.to_owned(),
                    tr.get(text).expect("should have key for text").to_owned(),
                );
            }
        }
        for (name, ids) in &astro_names_keys {
            for id in ids {
                translation.insert(
                    id.to_owned(),
                    tr.get(name)
                        .expect("should have name for astro object")
                        .to_owned(),
                );
            }
        }
        debug!(
            "count of keys in {} translation: {}",
            lang.file_name(),
            translation.len()
        );

        if args.write {
            let output = PathBuf::from(format!("output/translations/{}.json", lang.file_name()));
            info!("writing {}", output.display());
            serde_json::to_writer(File::create(output)?, &translation)?;
        }

        translations.insert(lang, translation);
    }

    // validate
    debug!("checking for missing keys in translations for entries");
    for (lang, tr) in translations {
        debug!("checking {}", lang.file_name());
        for a in &astro_objects {
            validate_entries_tr(&a.entries, &tr);
        }
    }

    Ok(())
}

fn validate_entries_tr(entries: &[JsonEntry], tr: &HashMap<String, String>) {
    for e in entries {
        if tr.get(&e.id).is_none() {
            warn!("missing translation for {}", e.id);
        }
        for f in &e.facts.explore {
            if tr.get(&f.id).is_none() {
                warn!("missing translation for {}", f.id);
            }
        }
        for f in &e.facts.rumor {
            if tr.get(&f.id).is_none() {
                warn!("missing translation for {}", f.id);
            }
        }
        if !e.entries.is_empty() {
            validate_entries_tr(&e.entries, tr);
        }
    }
}

fn count_entries(entries: &[JsonEntry]) -> u32 {
    if entries.is_empty() {
        return 0;
    }
    entries.len() as u32
        + entries
            .iter()
            .map(|e| count_entries(&e.entries))
            .sum::<u32>()
}

/// Clean translation keys from prefixes and map translations to keys for all
/// languages
///
/// Most translation keys starts from prefix, equal to one of astro object
/// names, e.g "VillageThe one and only Hearthian village, ...". Find them
/// and trim
fn clean_translations(
    tr_objects: Vec<Translations>,
    astro_names: Vec<String>,
) -> Result<HashMap<Lang, HashMap<String, String>>> {
    let mut last_prefix = astro_names
        .first()
        .ok_or_else(|| anyhow!("bug: astro_names can't be empty"))?
        .to_owned();

    let mut lang_order = V15_LANG_ORDER.iter();
    let mut translations = HashMap::new();
    for tr in tr_objects {
        let mut translation = HashMap::new();
        for Translation {
            key: original,
            value: translated,
        } in tr.entries
        {
            // todo: fix this case ("Escape Pod 3" detected as prefix)
            if original == "Escape Pod 3 Survivors" {
                translation.insert(original, translated);
                continue;
            }

            if !original.starts_with(&last_prefix) {
                let Some(prefix) = astro_names.iter().find(|&p| {
                    // remove prefix only if key is bigger
                    original.len() > p.len()
                        && original.starts_with(p)
                        // if character after prefix is not whitespace (not work)
                        && original.chars().nth(p.len()).is_some_and(|ch| !ch.is_whitespace())
                }) else {
                    // rumor translation
                    translation.insert(original, translated);
                    continue;
                };
                last_prefix = prefix.to_owned();
            }
            translation.insert(
                original
                    .strip_prefix(&last_prefix)
                    .expect("checked for prefix above")
                    .to_string(),
                translated,
            );
        }
        translations.insert(
            lang_order
                .next()
                .expect("there should be known language")
                .to_owned(),
            translation,
        );
    }

    Ok(translations)
}

/// Extract info about astro objects
fn load_astro_objects(file: File) -> Result<Vec<AstroObject<JsonEntry>>> {
    let mmap = unsafe { MmapOptions::new().offset(V15_SHARED_OFFSET).map(&file)? };

    let mut offset = 0;
    let mut astro_objects: Vec<AstroObject<JsonEntry>> = Vec::with_capacity(100);
    loop {
        let astro_object = match extract_astro_object(&mmap, offset) {
            Ok((astro_object, next_offset)) => {
                offset = next_offset;
                parse_astro_object(astro_object)?
            }
            Err(e) => match e {
                FindError::NotFound => break,
                FindError::Utf8Error(e) => return Err(e.into()),
            },
        };
        debug!("extracted {}", astro_object.id);
        astro_objects.push(astro_object.into());
    }

    Ok(astro_objects)
}

/// Extract translations
fn load_tr_objects(file: File) -> Result<Vec<Translations>> {
    let mmap = unsafe { Mmap::map(&file)? };

    let mut offset = 0;
    let mut tr_objects: Vec<Translations> = Vec::with_capacity(100);
    loop {
        let tr_object = match extract_shiplog_tr_object(&mmap, offset) {
            Ok((tr_object, next_offset)) => {
                offset = next_offset;
                parse_tr_object(tr_object)?
            }
            Err(e) => match e {
                FindError::NotFound => break,
                FindError::Utf8Error(e) => return Err(e.into()),
            },
        };
        tr_objects.push(tr_object);
    }

    Ok(tr_objects)
}

/// Returns Vec of (text, id)
fn collect_astro_texts(entries: &[JsonEntry]) -> Vec<(String, String)> {
    let mut kvs = vec![];
    for e in entries {
        for fact in &e.facts.explore {
            kvs.push((fact.text.clone(), fact.id.clone()));
        }
        for fact in &e.facts.rumor {
            kvs.push((fact.text.clone(), fact.id.clone()));
        }
        if !e.entries.is_empty() {
            kvs.extend_from_slice(&collect_astro_texts(&e.entries));
        }
    }
    kvs
}

/// Returns Vec of (name, id)
fn collect_astro_names(entries: &[JsonEntry]) -> Vec<(String, String)> {
    let mut names = vec![];
    for e in entries {
        names.push((e.name.clone(), e.id.clone()));
        if !e.entries.is_empty() {
            names.extend_from_slice(&collect_astro_names(&e.entries));
        }
    }
    names
}

fn parse_tr_object(data: &str) -> Result<Translations> {
    Ok(serde_xml_rs::from_str(data)?)
}

fn parse_astro_object(data: &str) -> Result<AstroObject<XmlEntry>> {
    Ok(serde_xml_rs::from_str(data)?)
}

fn extract_shiplog_tr_object(mmap: &Mmap, offset: usize) -> Result<(&str, usize), FindError> {
    extract_utf8(mmap, offset, TR_SHIPLOG_START, TR_SHIPLOG_END)
}

fn extract_astro_object(mmap: &Mmap, offset: usize) -> Result<(&str, usize), FindError> {
    extract_utf8(mmap, offset, ASTRO_OBJECT_START, ASTRO_OBJECT_END)
}

fn extract_utf8<'m>(
    mmap: &'m Mmap,
    offset: usize,
    start_marker: &[u8],
    end_marker: &[u8],
) -> Result<(&'m str, usize), FindError> {
    let start = find_start_of(mmap, offset, start_marker)?;
    let end = find_end_of(mmap, start, end_marker)?;

    let object = &mmap[start..end + 1];
    Ok((std::str::from_utf8(object)?, end + 1))
}

fn find_start_of(mmap: &Mmap, offset: usize, search: &[u8]) -> Result<usize, FindError> {
    Ok(find_indices_of(mmap, offset, search)?.0)
}

fn find_end_of(mmap: &Mmap, offset: usize, search: &[u8]) -> Result<usize, FindError> {
    Ok(find_indices_of(mmap, offset, search)?.1)
}

/// Search for byte substring in Mmap with offset. Returns indices of first
/// byte and next byte after substring
fn find_indices_of(mmap: &Mmap, offset: usize, search: &[u8]) -> Result<(usize, usize), FindError> {
    let search_len = search.len();
    let total_len = mmap.len();
    for (i, &_) in mmap.iter().enumerate().skip(offset) {
        // ....f...t
        // ......i   - break
        // t = total_len
        // f = t - search_len
        if i + search_len > total_len {
            break;
        }
        if &mmap[i..i + search_len] == search {
            return Ok((i, i + search_len - 1));
        }
    }
    Err(FindError::NotFound)
}

fn find_data_dir() -> Result<PathBuf> {
    Ok(dirs::home_dir()
        .ok_or_else(|| anyhow!("home dir not found"))?
        .join(".local/share/Steam/steamapps/common/Outer Wilds/OuterWilds_Data"))
}

#[derive(Debug, thiserror::Error)]
enum FindError {
    #[error("marker not found")]
    NotFound,

    #[error("failed to convert bytes to utf-8 string: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
}

#[cfg(test)]
mod tests {
    use models::entries::{ExploreFact, RumorFact, XmlEntry};

    use super::*;

    #[test]
    fn test_parse_astro_object() {
        let data = r#"
<AstroObjectEntry>
<ID>TIMBER_HEARTH</ID>
<Entry>
    <ID>TH_VILLAGE</ID>
    <Name>Village</Name>
    <Curiosity>1</Curiosity>
    <ExploreFact>
        <ID>TH_VILLAGE_X1</ID>
        <Text>2</Text>
    </ExploreFact>
    <Entry>
        <ID>TH_ZERO_G_CAVE</ID>
        <Name>Zero-G Cave</Name>
        <RumorFact>
            <ID>TH_ZERO_G_CAVE_R1</ID>
            <SourceID>5</SourceID>
            <RumorName>6</RumorName>
            <RumorNamePriority>0</RumorNamePriority>
            <Text>3</Text>
        </RumorFact>
        <ExploreFact>
            <ID>TH_ZERO_G_CAVE_X1</ID>
            <ClueType>7</ClueType>
            <IgnoreMoreToExplore/>
            <Text>4</Text>
        </ExploreFact>
    </Entry>
</Entry>
</AstroObjectEntry>
        "#;

        let parsed = parse_astro_object(data).unwrap();
        let expected = AstroObject::builder()
            .id("TIMBER_HEARTH".to_string())
            .entries(vec![
                XmlEntry::builder()
                    .id("TH_VILLAGE".to_string())
                    .name("Village".to_string())
                    .curiosity("1".to_string())
                    .explore_facts(vec![
                        ExploreFact::builder()
                            .id("TH_VILLAGE_X1".to_string())
                            .text("2".to_string())
                            .build(),
                    ])
                    .entries(vec![
                        XmlEntry::builder()
                            .id("TH_ZERO_G_CAVE".to_string())
                            .name("Zero-G Cave".to_string())
                            .rumor_facts(vec![
                                RumorFact::builder()
                                    .id("TH_ZERO_G_CAVE_R1".to_string())
                                    .source_id("5".to_string())
                                    // .name("6".to_string())
                                    // .name_priority(0)
                                    .text("3".to_string())
                                    .build(),
                            ])
                            .explore_facts(vec![
                                ExploreFact::builder()
                                    .id("TH_ZERO_G_CAVE_X1".to_string())
                                    // .clue_type("7".to_string())
                                    .ignore_more_to_explore(true)
                                    .text("4".to_string())
                                    .build(),
                            ])
                            .build(),
                    ])
                    .build(),
            ])
            .build();
        similar_asserts::assert_eq!(expected, parsed);
    }
}
