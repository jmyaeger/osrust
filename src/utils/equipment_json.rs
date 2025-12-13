#![allow(unused)]

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

const API_FIELDS: [&str; 25] = [
    "page_name",
    "page_name_sub",
    "item_name",
    "image",
    "item_id",
    "version_anchor",
    "infobox_bonuses.crush_attack_bonus",
    "infobox_bonuses.crush_defence_bonus",
    "infobox_bonuses.equipment_slot",
    "infobox_bonuses.magic_damage_bonus",
    "infobox_bonuses.magic_attack_bonus",
    "infobox_bonuses.magic_defence_bonus",
    "infobox_bonuses.prayer_bonus",
    "infobox_bonuses.range_attack_bonus",
    "infobox_bonuses.ranged_strength_bonus",
    "infobox_bonuses.range_defence_bonus",
    "infobox_bonuses.slash_attack_bonus",
    "infobox_bonuses.slash_defence_bonus",
    "infobox_bonuses.stab_attack_bonus",
    "infobox_bonuses.stab_defence_bonus",
    "infobox_bonuses.strength_bonus",
    "infobox_bonuses.weapon_attack_range",
    "infobox_bonuses.weapon_attack_speed",
    "infobox_bonuses.combat_style",
    "infobox_bonuses.equipment_slot",
];

const ITEMS_TO_SKIP: [&str; 26] = [
    "The dogsword",
    "Drygore blowpipe",
    "Amulet of the monarchs",
    "Emperor ring",
    "Devil's element",
    "Nature's reprisal",
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
    "Dagon'hai hat (or)",
    "Dagon'hai robe bottom (or)",
    "Dagon'hai robe top (or)",
    "Dragon warhammer (or)",
    "Centurion cuirass",
    "Ruinous powers (item)",
    "Battlehat",
    "Zaryte bow",
];

#[derive(Debug, Deserialize, Serialize)]
struct WikiResponse {
    bucket: Option<serde_json::Value>,
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

async fn get_equipment_data() -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let mut equipment = vec![];
    let mut offset = 0;

    loop {
        println!("Fetching equipment info: {offset}");

        let fields = API_FIELDS
            .iter()
            .map(|f| format!("'{f}'"))
            .collect::<Vec<_>>()
            .join(",");

        let query = format!(
            "bucket('infobox_item')
            .select({fields}).limit(500)
            .offset({offset})
            .where('infobox_bonuses.equipment_slot', '!=', bucket.Null())
            .where('item_id', '!=', bucket.Null())
            .join('infobox_bonuses', 'infobox_bonuses.page_name_sub', 'infobox_item.page_name_sub')
            .orderBy('page_name_sub', 'asc')
            .run()",
        );
        let url = Url::parse_with_params(
            API_BASE,
            &[("action", "bucket"), ("format", "json"), ("query", &query)],
        )
        .map_err(|e| format!("Failed to parse URL: {e}"))?;

        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header("User-Agent", "@jmyaeger (Orion) on discord")
            .send()
            .await?;

        let wiki_response: WikiResponse = response.json().await?;

        if let Some(bucket) = wiki_response.bucket {
            if let Some(arr) = bucket.as_array() {
                equipment.extend(arr.iter().cloned());

                if arr.len() == 500 {
                    offset += 500;
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    Ok(equipment)
}

#[cfg(feature = "data-generation")]
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;

    let wiki_data = get_equipment_data().await?;

    let mut data = vec![];
    let mut required_imgs = Vec::new();

    for v in wiki_data {
        if let Some(name) = v.get("page_name_sub") {
            if data
                .iter()
                .any(&|eq: &Equipment| eq.name == name.as_str().unwrap())
            {
                continue;
            } else {
                println!("Processing {name}");
            }
        }

        let item_id = match v.get("item_id").and_then(|v| v.as_array()) {
            Some(arr) if !arr.is_empty() => {
                match arr[0].as_str().and_then(|s| s.parse::<i64>().ok()) {
                    Some(id) => Some(id),
                    None => {
                        println!("Skipping - invalid item ID (not an int)");
                        continue;
                    }
                }
            }
            _ => None,
        };

        let offensive = Offensive {
            stab: v
                .get("infobox_bonuses.stab_attack_bonus")
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
            slash: v
                .get("infobox_bonuses.slash_attack_bonus")
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
            crush: v
                .get("infobox_bonuses.crush_attack_bonus")
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
            magic: v
                .get("infobox_bonuses.magic_attack_bonus")
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
            ranged: v
                .get("infobox_bonuses.range_attack_bonus")
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
        };

        let defensive = Defensive {
            stab: v
                .get("infobox_bonuses.stab_defence_bonus")
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
            slash: v
                .get("infobox_bonuses.slash_defence_bonus")
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
            crush: v
                .get("infobox_bonuses.crush_defence_bonus")
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
            magic: v
                .get("infobox_bonuses.magic_defence_bonus")
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
            ranged: v
                .get("infobox_bonuses.range_defence_bonus")
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
        };

        let strength_bonuses = StrengthBonuses {
            melee: v
                .get("infobox_bonuses.strength_bonus")
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
            ranged: v
                .get("infobox_bonuses.ranged_strength_bonus")
                .and_then(|v| v.as_i64())
                .unwrap_or_default(),
            magic: v
                .get("infobox_bonuses.magic_damage_bonus")
                .and_then(|v| v.as_f64())
                .unwrap_or_default(),
        };

        let mut equipment = Equipment {
            name: v.get("page_name").unwrap().to_string(),
            id: item_id.unwrap_or(-1),
            version: v
                .get("version_anchor")
                .map(|v| v.to_string().replace('\"', "")),
            image: v
                .get("image")
                .and_then(|v| v.as_array())
                .and_then(|a| a.last())
                .map(|v| v.to_string().replace("File:", "").replace('\"', ""))
                .unwrap_or_default(),
            slot: v
                .get("infobox_bonuses.equipment_slot")
                .map(|v| v.to_string())
                .unwrap_or_default()
                .replace('\"', ""),
            speed: v
                .get("infobox_bonuses.weapon_attack_speed")
                .and_then(|v| v.as_i64()),
            category: v
                .get("infobox_bonuses.combat_style")
                .map(|v| v.to_string().replace('\"', "")),
            bonuses: Bonuses {
                attack: offensive,
                defence: defensive,
                strength: strength_bonuses,
                prayer: v
                    .get("infobox_bonuses.prayer_bonus")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default(),
            },
            is_two_handed: None,
            attack_range: v
                .get("infobox_bonuses.weapon_attack_range")
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
    println!("Saving to JSON at file: {FILE_NAME}");
    serde_json::to_writer_pretty(&file, &data)?;
    println!("Equipment JSON file created successfully");

    let mut success_img_dls = 0;
    let mut failed_img_dls = 0;
    let mut skipped_img_dls = 0;
    let required_imgs: std::collections::HashSet<_> = required_imgs.into_iter().collect();
    for (idx, img) in required_imgs.iter().enumerate() {
        let img_path = format!("{IMG_PATH}{img}");
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
        let url = format!("{WIKI_BASE}/w/Special:Filepath/{img}");
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
                    println!("Saved image: {img}");
                    success_img_dls += 1;
                } else {
                    println!("Unable to save image: {img}");
                    failed_img_dls += 1;
                }
            }
            Err(_) => {
                println!("Error fetching image for {img}");
                continue;
            }
        }
    }
    println!("Total images saved: {success_img_dls}");
    println!("Total images skipped (already exists): {skipped_img_dls}");
    println!("Total images failed to save: {failed_img_dls}");

    Ok(())
}
