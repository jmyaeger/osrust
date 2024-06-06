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

const REQUIRED_PRINTOUTS: [&str; 36] = [
    "Attack bonus",
    "Attack level",
    "Attack speed",
    "Attack style",
    "Combat level",
    "Crush defence bonus",
    "Defence level",
    "Hitpoints",
    "Freeze resistance",
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
    "Elemental weakness",
    "Elemental weakness percent",
    "Light range defence bonus",
    "Standard range defence bonus",
    "Heavy range defence bonus",
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct ElementalWeakness {
    pub element: String,
    pub severity: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Monster {
    info: MonsterInfo,
    stats: Stats,
    bonuses: Bonuses,
    immunities: Immunities,
}

#[derive(Debug, Deserialize, Serialize)]
struct MonsterInfo {
    id: Option<i64>,
    name: String,
    version: Option<String>,
    combat_level: i64,
    attack_speed: i64,
    attack_styles: Option<Vec<String>>,
    size: i64,
    max_hit: Option<Vec<String>>,
    attributes: Option<Vec<String>>,
    weakness: Option<ElementalWeakness>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Bonuses {
    attack: Offensive,
    defence: Defensive,
    strength: Strength,
}

#[derive(Debug, Deserialize, Serialize)]
struct Immunities {
    poison: bool,
    venom: bool,
    freeze: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Stats {
    attack: i64,
    defence: i64,
    hitpoints: i64,
    magic: i64,
    ranged: i64,
    strength: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Offensive {
    melee: i64,
    ranged: i64,
    magic: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Defensive {
    crush: i64,
    magic: i64,
    heavy: i64,
    standard: i64,
    light: i64,
    slash: i64,
    stab: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Strength {
    melee: i64,
    ranged: i64,
    magic: i64,
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

        let version = k.split('#').nth(1).map(|v| v.to_string());

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

        let mut monster = Monster {
            info: MonsterInfo {
                id: get_printout_value(&po.get("NPC ID").cloned(), false)
                    .and_then(|id| id.as_i64()),
                name: k.split('#').next().unwrap_or("").to_string(),
                version,
                combat_level: get_printout_value(&po.get("Combat level").cloned(), false)
                    .and_then(|level| level.as_i64())
                    .unwrap_or(0),
                attack_speed: get_printout_value(&po.get("Attack speed").cloned(), false)
                    .and_then(|speed| speed.as_i64())
                    .unwrap_or(0),
                attack_styles: monster_style,
                size: get_printout_value(&po.get("Size").cloned(), false)
                    .and_then(|size| size.as_i64())
                    .unwrap_or(0),
                max_hit: get_printout_value(&po.get("Max hit").cloned(), true).map(|hit| {
                    hit.as_array()
                        .unwrap()
                        .iter()
                        .map(|s| s.as_str().unwrap().to_string())
                        .collect::<Vec<_>>()
                }),
                weakness: get_printout_value(&po.get("Elemental weakness").cloned(), false)
                    .map(|w| w.to_string())
                    .map(|weakness| ElementalWeakness {
                        element: weakness.to_lowercase().replace('\"', ""),
                        severity: get_printout_value(
                            &po.get("Elemental weakness percent").cloned(),
                            false,
                        )
                        .map(|s| s.to_string().replace('\"', ""))
                        .and_then(|severity| severity.parse::<i64>().ok())
                        .unwrap_or(0),
                    }),
                attributes: po.get("Monster attribute").cloned().map(|attr| {
                    attr.as_array()
                        .unwrap()
                        .iter()
                        .map(|attr| attr.as_str().unwrap().to_string())
                        .collect::<Vec<_>>()
                }),
            },

            stats: Stats {
                attack: get_printout_value(&po.get("Attack level").cloned(), false)
                    .and_then(|atk| atk.as_i64())
                    .unwrap_or(0),
                ranged: get_printout_value(&po.get("Ranged level").cloned(), false)
                    .and_then(|ranged| ranged.as_i64())
                    .unwrap_or(0),
                magic: get_printout_value(&po.get("Magic level").cloned(), false)
                    .and_then(|magic| magic.as_i64())
                    .unwrap_or(0),
                defence: get_printout_value(&po.get("Defence level").cloned(), false)
                    .and_then(|def| def.as_i64())
                    .unwrap_or(0),
                hitpoints: get_printout_value(&po.get("Hitpoints").cloned(), false)
                    .and_then(|hp| hp.as_i64())
                    .unwrap_or(0),
                strength: get_printout_value(&po.get("Strength level").cloned(), false)
                    .and_then(|str| str.as_i64())
                    .unwrap_or(0),
            },
            bonuses: Bonuses {
                attack: Offensive {
                    melee: get_printout_value(&po.get("Attack bonus").cloned(), false)
                        .and_then(|atk| atk.as_i64())
                        .unwrap_or(0),
                    ranged: get_printout_value(&po.get("Range attack bonus").cloned(), false)
                        .and_then(|ranged| ranged.as_i64())
                        .unwrap_or(0),
                    magic: get_printout_value(&po.get("Magic attack bonus").cloned(), false)
                        .and_then(|magic| magic.as_i64())
                        .unwrap_or(0),
                },
                defence: Defensive {
                    crush: get_printout_value(&po.get("Crush defence bonus").cloned(), false)
                        .and_then(|crush| crush.as_i64())
                        .unwrap_or(0),
                    magic: get_printout_value(&po.get("Magic defence bonus").cloned(), false)
                        .and_then(|magic| magic.as_i64())
                        .unwrap_or(0),
                    heavy: get_printout_value(&po.get("Heavy range defence bonus").cloned(), false)
                        .and_then(|heavy| heavy.as_i64())
                        .unwrap_or(0),
                    light: get_printout_value(&po.get("Light range defence bonus").cloned(), false)
                        .and_then(|light| light.as_i64())
                        .unwrap_or(0),
                    standard: get_printout_value(
                        &po.get("Standard range defence bonus").cloned(),
                        false,
                    )
                    .and_then(|standard| standard.as_i64())
                    .unwrap_or(0),
                    slash: get_printout_value(&po.get("Slash defence bonus").cloned(), false)
                        .and_then(|slash| slash.as_i64())
                        .unwrap_or(0),
                    stab: get_printout_value(&po.get("Stab defence bonus").cloned(), false)
                        .and_then(|stab| stab.as_i64())
                        .unwrap_or(0),
                },
                strength: Strength {
                    melee: get_printout_value(&po.get("Strength bonus").cloned(), false)
                        .and_then(|str| str.as_i64())
                        .unwrap_or(0),
                    ranged: get_printout_value(&po.get("Ranged Strength bonus").cloned(), false)
                        .and_then(|ranged_str| ranged_str.as_i64())
                        .unwrap_or(0),
                    magic: get_printout_value(&po.get("Magic Damage bonus").cloned(), false)
                        .and_then(|magic_str| magic_str.as_i64())
                        .unwrap_or_default(),
                },
            },
            immunities: Immunities {
                poison: get_printout_value(&po.get("Immune to poison").cloned(), false)
                    .and_then(|poison| poison.as_bool())
                    .unwrap_or_default(),
                venom: get_printout_value(&po.get("Immune to venom").cloned(), false)
                    .and_then(|venom| venom.as_bool())
                    .unwrap_or_default(),
                freeze: get_printout_value(&po.get("Immune to freeze").cloned(), false)
                    .and_then(|freeze| freeze.as_i64())
                    .unwrap_or_default(),
            },
        };

        if monster.stats.hitpoints == 0
            || monster.info.id.is_none()
            || monster.info.name.to_lowercase().contains("(historical)")
            || monster.info.name.to_lowercase().contains("(pvm arena)")
            || monster
                .info
                .name
                .to_lowercase()
                .contains("(deadman: apocalypse)")
        {
            continue;
        }

        if monster.info.name.contains("Vardorvis") {
            if let Some(version) = &monster.info.version {
                match version.as_str() {
                    "Post-Quest" => {
                        monster.stats.strength = 270;
                        monster.stats.defence = 215;
                    }
                    "Quest" => {
                        monster.stats.strength = 210;
                        monster.stats.defence = 180;
                    }
                    "Awakened" => {
                        monster.stats.strength = 391;
                        monster.stats.defence = 268;
                    }
                    _ => {}
                }
            }
        }

        // let image = monster.image.clone();
        data.push(monster);
        // if !image.is_empty() {
        //     required_imgs.push(image);
        // }
    }

    for (name, id) in [("Tumeken's Warden", 11762), ("Elidinis' Warden", 11761)] {
        data.push(Monster {
            info: MonsterInfo {
                id: Some(id),
                name: name.to_string(),
                version: Some("Enraged".to_string()),
                combat_level: 544,
                attack_speed: 8,
                attack_styles: Some(vec![
                    "Melee".to_string(),
                    "Ranged".to_string(),
                    "Magic".to_string(),
                ]),
                size: 5,
                max_hit: Some(vec!["26".to_string()]),
                attributes: None,
                weakness: None,
            },
            stats: Stats {
                attack: 150,
                defence: 150,
                hitpoints: 180,
                magic: 150,
                ranged: 150,
                strength: 150,
            },
            bonuses: Bonuses {
                attack: Offensive {
                    melee: 0,
                    ranged: 300,
                    magic: 230,
                },
                defence: Defensive {
                    crush: 20,
                    magic: 20,
                    heavy: 20,
                    standard: 20,
                    light: 20,
                    slash: 40,
                    stab: 40,
                },
                strength: Strength {
                    melee: 40,
                    ranged: 40,
                    magic: 40,
                },
            },
            immunities: Immunities {
                poison: false,
                venom: false,
                freeze: 0,
            },
        });
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
