use std::fs::File;
use std::io::Write;

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use url::Url;

const FILE_NAME: &str = "src/databases/equipment.json";
// const WIKI_BASE: &str = "https://oldschool.runescape.wiki";
const API_BASE: &str = "https://oldschool.runescape.wiki/api.php";
// const IMG_PATH: &str = "src/images/equipment/";

const REQUIRED_PRINTOUTS: [&str; 21] = [
    "Crush attack bonus",
    "Crush defence bonus",
    "Equipment slot",
    "Item ID",
    "Image",
    "Magic Damage bonus",
    "Magic attack bonus",
    "Magic defence bonus",
    "Prayer bonus",
    "Range attack bonus",
    "Ranged Strength bonus",
    "Range defence bonus",
    "Slash attack bonus",
    "Slash defence bonus",
    "Stab attack bonus",
    "Stab defence bonus",
    "Strength bonus",
    "Version anchor",
    "Weapon attack range",
    "Weapon attack speed",
    "Combat style",
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
struct Equipment {
    name: String,
    id: i64,
    version: String,
    slot: String,
    image: String,
    speed: i64,
    category: String,
    bonuses: Bonuses,
    offensive: Offensive,
    defensive: Defensive,
    is_two_handed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct Bonuses {
    str: Option<i64>,
    ranged_str: Option<i64>,
    magic_str: Option<i64>,
    prayer: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Offensive {
    stab: Option<i64>,
    slash: Option<i64>,
    crush: Option<i64>,
    magic: Option<i64>,
    ranged: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Defensive {
    stab: Option<i64>,
    slash: Option<i64>,
    crush: Option<i64>,
    magic: Option<i64>,
    ranged: Option<i64>,
}

async fn get_equipment_data() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let mut equipment = serde_json::Value::Object(Default::default());
    let mut offset = 0;

    loop {
        println!("Fetching equipment info: {}", offset);

        let query = format!(
            "[[Equipment slot::+]][[Item ID::+]]|?{}|limit=500|offset={}",
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
                equipment
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

    Ok(equipment)
}

fn get_printout_value(prop: &Option<serde_json::Value>) -> Option<i64> {
    prop.as_ref().and_then(|values| {
        if let Some(array) = values.as_array() {
            if array.is_empty() {
                None
            } else {
                array[0].as_i64()
            }
        } else {
            values.as_i64()
        }
    })
}

fn get_magic_damage_value(prop: &Option<serde_json::Value>) -> Option<i64> {
    prop.as_ref().and_then(|values| {
        if let Some(array) = values.as_array() {
            if array.is_empty() {
                None
            } else {
                array[0].as_f64().map(|v| (v * 10.0) as i64)
            }
        } else {
            values.as_f64().map(|v| (v * 10.0) as i64)
        }
    })
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wiki_data = get_equipment_data().await?;

    let mut data = Vec::new();
    // let mut required_imgs = Vec::new();

    for (k, v) in wiki_data.as_object().unwrap() {
        println!("Processing {}", k);

        if v.get("printouts").is_none() {
            println!("{} is missing SMW printouts - skipping.", k);
            continue;
        }

        let po = v.get("printouts").unwrap();
        let item_id = get_printout_value(&po.get("Item ID").cloned()).unwrap_or_default();

        let mut equipment = Equipment {
            name: k.split('#').next().unwrap().to_string(),
            id: item_id,
            version: get_printout_value(&po.get("Version anchor").cloned())
                .map(|v| v.to_string())
                .unwrap_or_default(),
            slot: get_printout_value(&po.get("Equipment slot").cloned())
                .map(|v| v.to_string())
                .unwrap_or_default(),
            image: po
                .get("Image")
                .and_then(|v| v.as_array())
                .and_then(|a| a.first())
                .and_then(|v| v.get("fulltext"))
                .and_then(|v| v.as_str())
                .map(|s| s.replace("File:", ""))
                .unwrap_or_default(),
            speed: get_printout_value(&po.get("Weapon attack speed").cloned()).unwrap_or_default(),
            category: get_printout_value(&po.get("Combat style").cloned())
                .map(|v| v.to_string())
                .unwrap_or_default(),
            bonuses: Bonuses {
                str: get_printout_value(&po.get("Strength bonus").cloned()),
                ranged_str: get_printout_value(&po.get("Ranged Strength bonus").cloned()),
                magic_str: get_magic_damage_value(&po.get("Magic Damage bonus").cloned()),
                prayer: get_printout_value(&po.get("Prayer bonus").cloned()),
            },
            offensive: Offensive {
                stab: get_printout_value(&po.get("Stab attack bonus").cloned()),
                slash: get_printout_value(&po.get("Slash attack bonus").cloned()),
                crush: get_printout_value(&po.get("Crush attack bonus").cloned()),
                magic: get_printout_value(&po.get("Magic attack bonus").cloned()),
                ranged: get_printout_value(&po.get("Range attack bonus").cloned()),
            },
            defensive: Defensive {
                stab: get_printout_value(&po.get("Stab defence bonus").cloned()),
                slash: get_printout_value(&po.get("Slash defence bonus").cloned()),
                crush: get_printout_value(&po.get("Crush defence bonus").cloned()),
                magic: get_printout_value(&po.get("Magic defence bonus").cloned()),
                ranged: get_printout_value(&po.get("Range defence bonus").cloned()),
            },
            is_two_handed: false,
        };

        if equipment.slot == "2h" {
            equipment.slot = "weapon".to_string();
            equipment.is_two_handed = true;
        }

        if equipment.version == "Nightmare Zone" {
            equipment.version = "".to_string();
        }

        // let image = equipment.image.clone();
        data.push(equipment);
        // if !image.is_empty() {
        //     required_imgs.push(image);
        // }
    }

    data.push(Equipment {
        name: "Snail shell".to_string(),
        id: 7800,
        version: "".to_string(),
        slot: "feet".to_string(),
        image: "Snail shell.png".to_string(),
        speed: 0,
        category: "".to_string(),
        bonuses: Bonuses {
            str: Some(0),
            ranged_str: Some(0),
            magic_str: Some(0),
            prayer: Some(0),
        },
        offensive: Offensive {
            stab: Some(0),
            slash: Some(0),
            crush: Some(0),
            magic: Some(0),
            ranged: Some(0),
        },
        defensive: Defensive {
            stab: Some(0),
            slash: Some(0),
            crush: Some(0),
            magic: Some(0),
            ranged: Some(0),
        },
        is_two_handed: false,
    });

    println!("Total equipment: {}", data.len());
    data.sort_by(|a, b| a.name.cmp(&b.name));

    let mut file = File::create(FILE_NAME)?;
    writeln!(file, "Saving to JSON at file: {}", FILE_NAME)?;
    serde_json::to_writer_pretty(&file, &data)?;

    // let mut success_img_dls = 0;
    // let mut failed_img_dls = 0;
    // let mut skipped_img_dls = 0;
    // let required_imgs: std::collections::HashSet<_> = required_imgs.into_iter().collect();

    // for (idx, img) in required_imgs.iter().enumerate() {
    //     let img_path = format!("{}{}", IMG_PATH, img);
    //     if std::path::Path::new(&img_path).exists() {
    //         skipped_img_dls += 1;
    //         continue;
    //     }

    //     println!(
    //         "({}/{}) Fetching image: {}",
    //         idx + 1,
    //         required_imgs.len(),
    //         img
    //     );
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
    //         let mut file = File::create(&img_path)?;
    //         let content = response.bytes().await?;
    //         file.write_all(&content)?;
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
