use std::fs::File;
// use std::io::Write;
// use std::path::Path;

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use url::Url;

const FILE_NAME: &str = "src/databases/monsters.json";
// const WIKI_BASE: &str = "https://oldschool.runescape.wiki";
const API_BASE: &str = "https://oldschool.runescape.wiki/api.php";
// const IMG_PATH: &str = "src/images/monsters/";

const REQUIRED_PRINTOUTS: [&str; 31] = [
    "Attack bonus",
    "Attack level",
    "Attack speed",
    "Attack style",
    "Combat level",
    "Crush defence bonus",
    "Defence level",
    "Hitpoints",
    "Image",
    "Immune to poison",
    "Immune to venom",
    "Magic Damage bonus",
    "Magic attack bonus",
    "Magic defence bonus",
    "Magic level",
    "Max hit",
    "Monster attribute",
    "Name",
    "Range attack bonus",
    "Ranged Strength bonus",
    "Range defence bonus",
    "Ranged level",
    "Slash defence bonus",
    "Slayer category",
    "Slayer experience",
    "Stab defence bonus",
    "Strength bonus",
    "Strength level",
    "Size",
    "NPC ID",
    "Category",
];

#[derive(Debug, Deserialize, Serialize)]
struct WikiResponse {
    query: Option<Query>,
    #[serde(rename = "query-continue-offset")]
    query_continue_offset: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Query {
    results: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Monster {
    id: Option<i64>,
    name: String,
    version: String,
    image: String,
    level: i64,
    speed: i64,
    style: Option<Vec<String>>,
    size: i64,
    max_hit: i64,
    skills: Skills,
    offensive: Offensive,
    defensive: Defensive,
    attributes: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Skills {
    atk: i64,
    def: i64,
    hp: i64,
    magic: i64,
    ranged: i64,
    str: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Offensive {
    atk: i64,
    magic: i64,
    magic_str: i64,
    ranged: i64,
    ranged_str: i64,
    str: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Defensive {
    crush: i64,
    magic: i64,
    ranged: i64,
    slash: i64,
    stab: i64,
}

async fn get_monster_data() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let mut monsters = serde_json::Value::Object(Default::default());
    let mut offset = 0;

    loop {
        println!("Fetching monster info: {}", offset);

        let query = format!(
            "[[Uses infobox::Monster]]|?{}|limit=500|offset={}",
            REQUIRED_PRINTOUTS.join("|?"),
            offset
        );
        let url = Url::parse_with_params(
            API_BASE,
            &[("action", "ask"), ("format", "json"), ("query", &query)],
        )
        .map_err(|e| format!("Failed to parse URL: {}", e))?;

        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header(
                "User-Agent",
                "osrs-dps-calc (https://github.com/weirdgloop/osrs-dps-calc)",
            )
            .send()
            .await?;

        let wiki_response: WikiResponse = response.json().await?;

        if let Some(query) = wiki_response.query {
            if let Some(results) = query.results {
                monsters
                    .as_object_mut()
                    .unwrap()
                    .extend(results.as_object().unwrap().clone().into_iter());
            } else {
                break;
            }
        } else {
            break;
        }

        if let Some(query_continue_offset) = wiki_response.query_continue_offset {
            if query_continue_offset < offset {
                break;
            } else {
                offset = query_continue_offset;
            }
        } else {
            break;
        }
    }

    Ok(monsters)
}

fn get_printout_value(
    prop: &Option<serde_json::Value>,
    all_results: bool,
) -> Option<serde_json::Value> {
    prop.as_ref().and_then(|value| {
        if let Some(array) = value.as_array() {
            if array.is_empty() {
                None
            } else if all_results {
                Some(serde_json::Value::Array(array.clone()))
            } else {
                Some(array[0].clone())
            }
        } else {
            Some(value.clone())
        }
    })
}

fn has_category(category_array: &[serde_json::Value], category: &str) -> bool {
    category_array.iter().any(|c| {
        c.get("fulltext")
            .and_then(|fulltext| fulltext.as_str())
            .map_or(false, |fulltext| {
                fulltext == format!("Category:{}", category)
            })
    })
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wiki_data = get_monster_data().await?;

    let mut data = Vec::new();
    // let mut required_imgs = Vec::new();

    for (k, v) in wiki_data.as_object().unwrap() {
        println!("Processing {}", k);

        if v.get("printouts").is_none() {
            println!("{} is missing SMW printouts - skipping.", k);
            continue;
        }

        let po = v.get("printouts").unwrap();

        let version = k.split('#').nth(1).unwrap_or("").to_string();

        // if version.contains("Challenge Mode") {
        //     println!("{} is a CoX CM variant - skipping.", k);
        //     continue;
        // }

        if k.contains(':') {
            continue;
        }

        let category = po.get("Category").unwrap().as_array().unwrap();
        if has_category(category, "Non-interactive scenery")
            || has_category(category, "Discontinued content")
        {
            continue;
        }

        let monster_style = get_printout_value(&po.get("Attack style").cloned(), true);
        let monster_style = monster_style.and_then(|style| {
            if style == serde_json::Value::String("None".to_string())
                || style == serde_json::Value::String("N/A".to_string())
            {
                None
            } else {
                Some(
                    style
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|s| s.as_str().unwrap().to_string())
                        .collect::<Vec<_>>(),
                )
            }
        });

        let monster = Monster {
            id: get_printout_value(&po.get("NPC ID").cloned(), false).and_then(|id| id.as_i64()),
            name: k.split('#').next().unwrap_or("").to_string(),
            version,
            image: get_printout_value(&po.get("Image").cloned(), false).map_or_else(
                || "".to_string(),
                |image| image.as_str().unwrap_or("").replace("File:", ""),
            ),
            level: get_printout_value(&po.get("Combat level").cloned(), false)
                .and_then(|level| level.as_i64())
                .unwrap_or(0),
            speed: get_printout_value(&po.get("Attack speed").cloned(), false)
                .and_then(|speed| speed.as_i64())
                .unwrap_or(0),
            style: monster_style,
            size: get_printout_value(&po.get("Size").cloned(), false)
                .and_then(|size| size.as_i64())
                .unwrap_or(0),
            max_hit: get_printout_value(&po.get("Max hit").cloned(), false)
                .and_then(|max_hit| max_hit.as_i64())
                .unwrap_or(0),
            skills: Skills {
                atk: get_printout_value(&po.get("Attack level").cloned(), false)
                    .and_then(|atk| atk.as_i64())
                    .unwrap_or(0),
                def: get_printout_value(&po.get("Defence level").cloned(), false)
                    .and_then(|def| def.as_i64())
                    .unwrap_or(0),
                hp: get_printout_value(&po.get("Hitpoints").cloned(), false)
                    .and_then(|hp| hp.as_i64())
                    .unwrap_or(0),
                magic: get_printout_value(&po.get("Magic level").cloned(), false)
                    .and_then(|magic| magic.as_i64())
                    .unwrap_or(0),
                ranged: get_printout_value(&po.get("Ranged level").cloned(), false)
                    .and_then(|ranged| ranged.as_i64())
                    .unwrap_or(0),
                str: get_printout_value(&po.get("Strength level").cloned(), false)
                    .and_then(|str| str.as_i64())
                    .unwrap_or(0),
            },
            offensive: Offensive {
                atk: get_printout_value(&po.get("Attack bonus").cloned(), false)
                    .and_then(|atk| atk.as_i64())
                    .unwrap_or(0),
                magic: get_printout_value(&po.get("Magic attack bonus").cloned(), false)
                    .and_then(|magic| magic.as_i64())
                    .unwrap_or(0),
                magic_str: get_printout_value(&po.get("Magic Damage bonus").cloned(), false)
                    .and_then(|magic_str| magic_str.as_i64())
                    .unwrap_or(0),
                ranged: get_printout_value(&po.get("Range attack bonus").cloned(), false)
                    .and_then(|ranged| ranged.as_i64())
                    .unwrap_or(0),
                ranged_str: get_printout_value(&po.get("Ranged Strength bonus").cloned(), false)
                    .and_then(|ranged_str| ranged_str.as_i64())
                    .unwrap_or(0),
                str: get_printout_value(&po.get("Strength bonus").cloned(), false)
                    .and_then(|str| str.as_i64())
                    .unwrap_or(0),
            },
            defensive: Defensive {
                crush: get_printout_value(&po.get("Crush defence bonus").cloned(), false)
                    .and_then(|crush| crush.as_i64())
                    .unwrap_or(0),
                magic: get_printout_value(&po.get("Magic defence bonus").cloned(), false)
                    .and_then(|magic| magic.as_i64())
                    .unwrap_or(0),
                ranged: get_printout_value(&po.get("Range defence bonus").cloned(), false)
                    .and_then(|ranged| ranged.as_i64())
                    .unwrap_or(0),
                slash: get_printout_value(&po.get("Slash defence bonus").cloned(), false)
                    .and_then(|slash| slash.as_i64())
                    .unwrap_or(0),
                stab: get_printout_value(&po.get("Stab defence bonus").cloned(), false)
                    .and_then(|stab| stab.as_i64())
                    .unwrap_or(0),
            },
            attributes: po
                .get("Monster attribute")
                .cloned()
                .unwrap_or_else(|| serde_json::Value::Array(Vec::new()))
                .as_array()
                .unwrap()
                .iter()
                .map(|attr| attr.as_str().unwrap().to_string())
                .collect(),
        };

        if monster.skills.hp == 0
            || monster.id.is_none()
            || monster.name.to_lowercase().contains("(historical)")
            || monster.name.to_lowercase().contains("(pvm arena)")
            || monster
                .name
                .to_lowercase()
                .contains("(deadman: apocalypse)")
        {
            continue;
        }

        // let image = monster.image.clone();
        data.push(monster);
        // if !image.is_empty() {
        //     required_imgs.push(image);
        // }
    }

    println!("Total monsters: {}", data.len());

    let file = File::create(FILE_NAME)?;
    serde_json::to_writer_pretty(file, &data)?;
    println!("Saving to JSON at file: {}", FILE_NAME);

    // let mut success_img_dls = 0;
    // let mut failed_img_dls = 0;
    // let mut skipped_img_dls = 0;
    // let required_imgs: std::collections::HashSet<_> = required_imgs.into_iter().collect();

    // let mut saved_image_paths = std::collections::HashSet::new();
    // for (idx, img) in required_imgs.iter().enumerate() {
    //     let dest_path = format!("{}{}", IMG_PATH, img);
    //     if saved_image_paths.contains(&dest_path.to_lowercase()) {
    //         println!(
    //             "[WARN] Case-sensitive image filename clashes: {}",
    //             dest_path
    //         );
    //         continue;
    //     }

    //     saved_image_paths.insert(dest_path.to_lowercase());
    //     if Path::new(&dest_path).exists() {
    //         skipped_img_dls += 1;
    //         continue;
    //     }

    //     println!("({}/{}) Fetching image: {}", idx, required_imgs.len(), img);
    //     let url = format!("{}/w/Special:Filepath/{}", WIKI_BASE, img);
    //     let client = reqwest::Client::new();
    //     let response = client
    //         .get(&url)
    //         .header(
    //             "User-Agent",
    //             "osrs-dps-calc (https://github.com/weirdgloop/osrs-dps-calc)",
    //         )
    //         .send()
    //         .await?;

    //     if response.status().is_success() {
    //         let mut file = File::create(&dest_path)?;
    //         file.write_all(&response.bytes().await?)?;
    //         println!("Saved image: {}", img);
    //         success_img_dls += 1;
    //     } else {
    //         println!("Unable to save image: {}", img);
    //         failed_img_dls += 1;
    //     }
    // }

    // println!("Total images saved: {}", success_img_dls);
    // println!("Total images skipped (already exists): {}", skipped_img_dls);
    // println!("Total images failed to save: {}", failed_img_dls);

    Ok(())
}
