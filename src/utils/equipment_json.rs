// Adapted from the wiki DPS calc - credit to the wiki team

use std::collections::HashMap;

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::Write;
use url::Url;

const FILE_NAME: &str = "src/databases/equipment.json";
const API_BASE: &str = "https://oldschool.runescape.wiki/api.php";
const IMG_PATH: &str = "src/images/equipment/";
const WIKI_BASE: &str = "https://oldschool.runescape.wiki";

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

const ITEMS_TO_SKIP: [&str; 23] = [
    "The dogsword",
    "Drygore blowpipe",
    "Amulet of the monarchs",
    "Emperor ring",
    "Devil\"s element",
    "Nature\"s reprisal",
    "Gloves of the damned",
    "Crystal blessing",
    "Sunlight spear",
    "Sunlit bracers",
    "Thunder khopesh",
    "Thousand-dragon ward",
    "Arcane grimoire",
    "Wristbands of the arena",
    "Wristbands of the arena (i)",
    "Armadyl chainskirt (or)",
    "Armadyl chestplate (or)",
    "Armadyl helmet (or)",
    "Dagon\"hai hat (or)",
    "Dagon\"hai robe bottom (or)",
    "Dagon\"hai robe top (or)",
    "Dragon warhammer (or)",
    "Centurion cuirass",
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
    version: Option<String>,
    slot: String,
    image: String,
    speed: Option<i64>,
    category: Option<String>,
    bonuses: Bonuses,
    is_two_handed: Option<bool>,
    attack_range: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Bonuses {
    attack: Offensive,
    defence: Defensive,
    strength: StrengthBonuses,
    prayer: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct StrengthBonuses {
    melee: i64,
    ranged: i64,
    magic: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Offensive {
    stab: i64,
    slash: i64,
    crush: i64,
    magic: i64,
    ranged: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Defensive {
    stab: i64,
    slash: i64,
    crush: i64,
    magic: i64,
    ranged: i64,
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
            .header("User-Agent", "@jmyaeger (Orion) on discord")
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

fn get_printout_value(prop: &Option<serde_json::Value>) -> Option<serde_json::Value> {
    prop.as_ref().and_then(|values| {
        if let Some(array) = values.as_array() {
            if array.is_empty() {
                None
            } else {
                Some(array[0].clone())
            }
        } else {
            Some(values.clone())
        }
    })
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wiki_data = get_equipment_data().await?;

    let mut data = vec![];
    let mut required_imgs = Vec::new();

    for (k, v) in wiki_data.as_object().unwrap() {
        println!("Processing {}", k);

        if v.get("printouts").is_none() {
            println!("{} is missing SMW printouts - skipping.", k);
            continue;
        }

        let po = v.get("printouts").unwrap();
        let item_id = get_printout_value(&po.get("Item ID").cloned())
            .and_then(|v| v.as_i64())
            .unwrap_or_default();

        let offensive = Offensive {
            stab: get_printout_value(&po.get("Stab attack bonus").cloned())
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
            slash: get_printout_value(&po.get("Slash attack bonus").cloned())
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
            crush: get_printout_value(&po.get("Crush attack bonus").cloned())
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
            magic: get_printout_value(&po.get("Magic attack bonus").cloned())
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
            ranged: get_printout_value(&po.get("Range attack bonus").cloned())
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
        };

        let defensive = Defensive {
            stab: get_printout_value(&po.get("Stab defence bonus").cloned())
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
            slash: get_printout_value(&po.get("Slash defence bonus").cloned())
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
            crush: get_printout_value(&po.get("Crush defence bonus").cloned())
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
            magic: get_printout_value(&po.get("Magic defence bonus").cloned())
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
            ranged: get_printout_value(&po.get("Range defence bonus").cloned())
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
        };

        let strength_bonuses = StrengthBonuses {
            melee: get_printout_value(&po.get("Strength bonus").cloned())
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
            ranged: get_printout_value(&po.get("Ranged Strength bonus").cloned())
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
            magic: get_printout_value(&po.get("Magic Damage bonus").cloned())
                .and_then(|v| v.as_f64())
                .unwrap_or_default(),
        };

        let mut equipment = Equipment {
            name: k.split('#').next().unwrap().to_string(),
            id: item_id,
            version: get_printout_value(&po.get("Version anchor").cloned())
                .map(|v| v.to_string().replace('\"', "")),
            image: po
                .get("Image")
                .and_then(|v| v.as_array())
                .and_then(|a| a.first())
                .and_then(|v| v.get("fulltext"))
                .and_then(|v| v.as_str())
                .map(|s| s.replace("File:", ""))
                .unwrap_or_default(),
            slot: get_printout_value(&po.get("Equipment slot").cloned())
                .map(|v| v.to_string())
                .unwrap_or_default()
                .replace('\"', ""),
            speed: get_printout_value(&po.get("Weapon attack speed").cloned())
                .and_then(|v| v.as_i64()),
            category: get_printout_value(&po.get("Combat style").cloned())
                .map(|v| v.to_string().replace('\"', "")),
            bonuses: Bonuses {
                attack: offensive,
                defence: defensive,
                strength: strength_bonuses,
                prayer: get_printout_value(&po.get("Prayer bonus").cloned())
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default(),
            },
            is_two_handed: None,
            attack_range: get_printout_value(&po.get("Weapon attack range").cloned())
                .and_then(|v| v.as_i64()),
        };

        if equipment.slot == "2h" {
            equipment.slot = "weapon".to_string();
            equipment.is_two_handed = Some(true);
        } else if equipment.slot == "weapon" {
            equipment.is_two_handed = Some(false);
        }

        if equipment.version == Some("Nightmare Zone".to_string()) {
            equipment.version = None;
        }

        if let Some(version) = &equipment.version.clone() {
            if ["100", "75", "50", "25", "0"].contains(&version.as_str()) {
                continue;
            }

            if ["New", "Used"].contains(&version.as_str()) {
                equipment.version = None;
            }

            if equipment.name == "Toxic blowpipe"
                && ["Empty", "Charged"].contains(&version.as_str())
            {
                continue;
            }

            if [
                "Accursed sceptre",
                "Accursed sceptre (a)",
                "Corrupted tumeken's shadow",
                "Craw's bow",
                "Holy sanguinesti staff",
                "Sanguinesti staff",
                "Thammaron's sceptre",
                "Thammaron's sceptre (a)",
                "Trident of the seas",
                "Trident of the seas (e)",
                "Trident of the swamp",
                "trident of the swamp (e)",
                "Tumeken's shadow",
                "Ursine chainmace",
                "Viggora's chainmace",
                "Warped sceptre",
                "Webweaver bow",
            ]
            .contains(&equipment.name.as_str())
                && version == "Uncharged"
            {
                continue;
            }

            if [
                "Blade of saeldor",
                "Bow of faerdhinen",
                "Crystal body",
                "Crystal helm",
                "Crystal legs",
                "Crystal shield",
            ]
            .contains(&equipment.name.as_str())
                && version == "Inactive"
            {
                continue;
            }

            if equipment.name.contains("Black mask") {
                if ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"].contains(&version.as_str()) {
                    continue;
                }

                if version == "Uncharged" {
                    equipment.version = None;
                }
            }

            if ["Locked", "Broken"].contains(&version.as_str()) {
                continue;
            }

            if ["Normal", "Restored", "Undamaged"].contains(&version.as_str()) {
                equipment.version = None;
            }
        }

        if equipment.name.contains("historical") {
            continue;
        }

        if ITEMS_TO_SKIP.contains(&equipment.name.as_str()) {
            continue;
        }

        if equipment.name.contains("(Last Man Standing)") {
            continue;
        }

        let image = equipment.image.clone();
        if !image.is_empty() {
            required_imgs.push(image);
        }

        data.push(equipment);
    }

    let ammo_list = HashMap::from([
        ("Bronze", 21),
        ("Iron", 22),
        ("Steel", 23),
        ("Black", 26),
        ("Mithril", 29),
        ("Adamant", 37),
        ("Rune", 46),
        ("Amethyst", 48),
        ("Dragon", 55),
    ]);

    for (k, v) in &ammo_list {
        let equipment = Equipment {
            name: "Toxic blowpipe".to_string(),
            id: 12926,
            version: Some(k.to_string()),
            image: "Toxic blowpipe.png".to_string(),
            slot: "weapon".to_string(),
            speed: Some(3),
            category: Some("Thrown".to_string()),
            bonuses: Bonuses {
                attack: Offensive {
                    stab: 0,
                    slash: 0,
                    crush: 0,
                    magic: 0,
                    ranged: 30,
                },
                defence: Defensive {
                    stab: 0,
                    slash: 0,
                    crush: 0,
                    magic: 0,
                    ranged: 0,
                },
                strength: StrengthBonuses {
                    melee: 0,
                    ranged: *v,
                    magic: 0.0,
                },
                prayer: 0,
            },
            is_two_handed: Some(true),
            attack_range: Some(5),
        };

        data.push(equipment);
    }

    let file = File::create(FILE_NAME)?;
    println!("Saving to JSON at file: {}", FILE_NAME);
    serde_json::to_writer_pretty(&file, &data)?;
    println!("Equipment JSON file created successfully");

    let mut success_img_dls = 0;
    let mut failed_img_dls = 0;
    let mut skipped_img_dls = 0;
    let required_imgs: std::collections::HashSet<_> = required_imgs.into_iter().collect();
    for (idx, img) in required_imgs.iter().enumerate() {
        let img_path = format!("{}{}", IMG_PATH, img);
        if std::path::Path::new(&img_path).exists() {
            skipped_img_dls += 1;
            continue;
        }
        println!(
            "({}/{}) Fetching image: {}",
            idx + 1,
            required_imgs.len(),
            img
        );
        let url = format!("{}/w/Special:Filepath/{}", WIKI_BASE, img);
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("User-Agent", "@jmyaeger (Orion) on discord")
            .send()
            .await;
        match response {
            Ok(response) => {
                if response.status().is_success() {
                    let mut file = File::create(&img_path)?;
                    let content = response.bytes().await?;
                    file.write_all(&content)?;
                    println!("Saved image: {}", img);
                    success_img_dls += 1;
                } else {
                    println!("Unable to save image: {}", img);
                    failed_img_dls += 1;
                }
            }
            Err(_) => {
                println!("Error fetching image for {}", img);
                continue;
            }
        }
    }
    println!("Total images saved: {}", success_img_dls);
    println!("Total images skipped (already exists): {}", skipped_img_dls);
    println!("Total images failed to save: {}", failed_img_dls);

    Ok(())
}
