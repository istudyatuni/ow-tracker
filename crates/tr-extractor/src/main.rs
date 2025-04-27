use std::{fs::File, path::PathBuf};

use anyhow::{Result, anyhow};
use memmap2::{Mmap, MmapOptions};
use models::AstroObject;

mod models;

const ASTRO_OBJECT_START: &[u8] = b"<AstroObjectEntry>";
const ASTRO_OBJECT_END: &[u8] = b"</AstroObjectEntry>";
const SHARED_FILE: &str = "sharedassets1.assets";
/// Offset in file `sharedassets1.assets` for game version 1.1.15. Probably
/// unnecessary since search seems to be pretty fast
const V15_SHARED_OFFSET: u64 = 930000000;

fn main() -> Result<()> {
    let dir = find_data_dir()?;
    let ids_file = File::open(dir.join(SHARED_FILE))?;
    let mmap = unsafe {
        MmapOptions::new()
            .offset(V15_SHARED_OFFSET)
            .map(&ids_file)?
    };

    let mut offset = 0;
    let mut astro_objects = Vec::with_capacity(100);
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
        astro_objects.push(astro_object);
    }

    Ok(())
}

fn parse_astro_object(data: &str) -> Result<AstroObject> {
    Ok(serde_xml_rs::from_str(data)?)
}

fn extract_astro_object(mmap: &Mmap, offset: usize) -> Result<(&str, usize), FindError> {
    let astro_start = find_start_of(mmap, offset, ASTRO_OBJECT_START)?;
    let astro_end = find_end_of(mmap, astro_start, ASTRO_OBJECT_END)?;

    let astro_object = &mmap[astro_start..astro_end + 1];
    Ok((std::str::from_utf8(astro_object)?, astro_end + 1))
}

fn find_start_of(mmap: &Mmap, offset: usize, search: &[u8]) -> Result<usize, FindError> {
    Ok(find_indices_of(mmap, offset, search)?.0)
}

fn find_end_of(mmap: &Mmap, offset: usize, search: &[u8]) -> Result<usize, FindError> {
    Ok(find_indices_of(mmap, offset, search)?.1)
}

fn find_indices_of(mmap: &Mmap, offset: usize, search: &[u8]) -> Result<(usize, usize), FindError> {
    let search_len = search.len();
    let total_len = mmap.len();
    for (i, &_) in mmap.iter().skip(offset).enumerate() {
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
    use models::{Entry, ExploreFact, RumorFact};

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
                Entry::builder()
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
                        Entry::builder()
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
