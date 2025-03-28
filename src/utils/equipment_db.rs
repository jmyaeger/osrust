// Adapted from the wiki DPS calc - credit to the wiki team

use std::collections::HashMap;

use reqwest;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::Write;
use url::Url;

const FILE_NAME: &str = "src/databases/equipment.db";
const FLAT_FILE_NAME: &str = "src/databases/equipment_flattened.db";
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

    let mut required_imgs = Vec::new();

    let conn = Connection::open(FILE_NAME)?;
    let conn_flat = Connection::open(FLAT_FILE_NAME)?;

    conn.execute(
        "CREATE TABLE equipment (
            id INTEGER PRIMARY KEY,
            name TEXT,
            version TEXT,
            data TEXT
        )",
        [],
    )?;

    conn_flat.execute(
        "CREATE TABLE equipment (
            id INTEGER PRIMARY KEY,
            item_id INTEGER,
            name TEXT,
            version TEXT,
            image TEXT,
            slot TEXT,
            speed INTEGER,
            category TEXT,
            attack_stab INTEGER,
            attack_slash INTEGER,
            attack_crush INTEGER,
            attack_magic INTEGER,
            attack_ranged INTEGER,
            defence_stab INTEGER,
            defence_slash INTEGER,
            defence_crush INTEGER,
            defence_magic INTEGER,
            defence_ranged INTEGER,
            strength_melee INTEGER,
            strength_ranged INTEGER,
            strength_magic INTEGER,
            prayer INTEGER,
            is_two_handed BOOLEAN,
            attack_range INTEGER
        )",
        [],
    )?;

    let mut stmt =
        conn.prepare("INSERT INTO equipment (name, version, data) VALUES (?1, ?2, ?3)")?;

    let mut stmt_flat = conn_flat.prepare(
        "INSERT INTO equipment (
            item_id,
            name,
            version,
            image,
            slot,
            speed,
            category,
            attack_stab,
            attack_slash,
            attack_crush,
            attack_magic,
            attack_ranged,
            defence_stab,
            defence_slash,
            defence_crush,
            defence_magic,
            defence_ranged,
            strength_melee,
            strength_ranged,
            strength_magic,
            prayer,
            is_two_handed,
            attack_range
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17,
            ?18, ?19, ?20, ?21, ?22, ?23
        )",
    )?;

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

        let image = equipment.image.clone();
        if !image.is_empty() {
            required_imgs.push(image);
        }

        push_to_db(&mut stmt, &equipment)?;
        push_to_db_flat(&mut stmt_flat, &equipment)?;
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

        push_to_db(&mut stmt, &equipment)?;
        push_to_db_flat(&mut stmt_flat, &equipment)?;
    }

    let natures_reprisal = Equipment {
        name: "Nature's reprisal".to_string(),
        id: -1,
        version: Some("Nature's reprisal".to_string()),
        image: "Nature's reprisal.png".to_string(),
        slot: "weapon".to_string(),
        speed: Some(5),
        category: Some("Salamander".to_string()),
        bonuses: Bonuses {
            attack: Offensive {
                stab: 95,
                slash: 0,
                crush: 0,
                magic: 45,
                ranged: 155,
            },
            defence: Defensive {
                stab: 0,
                slash: 0,
                crush: 0,
                magic: 0,
                ranged: 0,
            },
            strength: StrengthBonuses {
                melee: 103,
                ranged: 92,
                magic: 12.0,
            },
            prayer: 0,
        },
        is_two_handed: Some(true),
        attack_range: Some(7),
    };

    let drygore_blowpipe = Equipment {
        name: "Drygore blowpipe".to_string(),
        id: -1,
        version: Some("Drygore blowpipe".to_string()),
        image: "Drygore blowpipe.png".to_string(),
        slot: "weapon".to_string(),
        speed: Some(3),
        category: Some("Thrown".to_string()),
        bonuses: Bonuses {
            attack: Offensive {
                stab: 0,
                slash: 0,
                crush: 0,
                magic: 0,
                ranged: 50,
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
                ranged: 0,
                magic: 0.0,
            },
            prayer: 0,
        },
        is_two_handed: Some(true),
        attack_range: Some(6),
    };

    let dogsword = Equipment {
        name: "The dogsword".to_string(),
        id: -1,
        version: Some("The dogsword".to_string()),
        image: "The dogsword.png".to_string(),
        slot: "weapon".to_string(),
        speed: Some(6),
        category: Some("2h Sword".to_string()),
        bonuses: Bonuses {
            attack: Offensive {
                stab: 0,
                slash: 132,
                crush: 80,
                magic: 0,
                ranged: 0,
            },
            defence: Defensive {
                stab: 0,
                slash: 0,
                crush: 0,
                magic: 0,
                ranged: 0,
            },
            strength: StrengthBonuses {
                melee: 132,
                ranged: 0,
                magic: 0.0,
            },
            prayer: 8,
        },
        is_two_handed: Some(true),
        attack_range: Some(1),
    };

    let thunder_khopesh = Equipment {
        name: "Thunder khopesh".to_string(),
        id: -1,
        version: Some("Thunder khopesh".to_string()),
        image: "Thunder khopesh.png".to_string(),
        slot: "weapon".to_string(),
        speed: Some(4),
        category: Some("Slash Sword".to_string()),
        bonuses: Bonuses {
            attack: Offensive {
                stab: 65,
                slash: 110,
                crush: 0,
                magic: 0,
                ranged: 0,
            },
            defence: Defensive {
                stab: 0,
                slash: 0,
                crush: 0,
                magic: 0,
                ranged: 0,
            },
            strength: StrengthBonuses {
                melee: 100,
                ranged: 0,
                magic: 0.0,
            },
            prayer: 0,
        },
        is_two_handed: Some(false),
        attack_range: Some(1),
    };

    let sunlight_spear = Equipment {
        name: "Sunlight spear".to_string(),
        id: -1,
        version: Some("Sunlight spear".to_string()),
        image: "Sunlight spear.png".to_string(),
        slot: "weapon".to_string(),
        speed: Some(5),
        category: Some("Spear".to_string()),
        bonuses: Bonuses {
            attack: Offensive {
                stab: 125,
                slash: 75,
                crush: 0,
                magic: 0,
                ranged: 0,
            },
            defence: Defensive {
                stab: 70,
                slash: 70,
                crush: 70,
                magic: 28,
                ranged: 70,
            },
            strength: StrengthBonuses {
                melee: 113,
                ranged: 0,
                magic: 0.0,
            },
            prayer: 0,
        },
        is_two_handed: Some(true),
        attack_range: Some(1),
    };

    let thousand_dragon_ward = Equipment {
        name: "Thousand-dragon ward".to_string(),
        id: -1,
        version: Some("Thousand-dragon ward".to_string()),
        image: "Thousand-dragon ward.png".to_string(),
        slot: "shield".to_string(),
        speed: None,
        category: None,
        bonuses: Bonuses {
            attack: Offensive {
                stab: 35,
                slash: 35,
                crush: 35,
                magic: 15,
                ranged: 25,
            },
            defence: Defensive {
                stab: 50,
                slash: 55,
                crush: 53,
                magic: 40,
                ranged: 52,
            },
            strength: StrengthBonuses {
                melee: 12,
                ranged: 0,
                magic: 0.0,
            },
            prayer: 6,
        },
        is_two_handed: None,
        attack_range: None,
    };

    let gloves_of_the_damned = Equipment {
        name: "Gloves of the damned".to_string(),
        id: -1,
        version: Some("Gloves of the damned".to_string()),
        image: "Gloves of the damned.png".to_string(),
        slot: "hands".to_string(),
        speed: None,
        category: None,
        bonuses: Bonuses {
            attack: Offensive {
                stab: 12,
                slash: 12,
                crush: 12,
                magic: 6,
                ranged: 12,
            },
            defence: Defensive {
                stab: 12,
                slash: 12,
                crush: 12,
                magic: 6,
                ranged: 12,
            },
            strength: StrengthBonuses {
                melee: 12,
                ranged: 0,
                magic: 0.0,
            },
            prayer: 0,
        },
        is_two_handed: None,
        attack_range: None,
    };

    let sunlit_bracers = Equipment {
        name: "Sunlit bracers".to_string(),
        id: -1,
        version: Some("Sunlit bracers".to_string()),
        image: "Sunlit bracers.png".to_string(),
        slot: "hands".to_string(),
        speed: None,
        category: None,
        bonuses: Bonuses {
            attack: Offensive {
                stab: 10,
                slash: 10,
                crush: 10,
                magic: 6,
                ranged: 18,
            },
            defence: Defensive {
                stab: 12,
                slash: 12,
                crush: 12,
                magic: 6,
                ranged: 12,
            },
            strength: StrengthBonuses {
                melee: 8,
                ranged: 4,
                magic: 0.0,
            },
            prayer: 0,
        },
        is_two_handed: None,
        attack_range: None,
    };

    let amulet_of_the_monarchs = Equipment {
        name: "Amulet of the monarchs".to_string(),
        id: -1,
        version: Some("Amulet of the monarchs".to_string()),
        image: "Amulet of the monarchs.png".to_string(),
        slot: "neck".to_string(),
        speed: None,
        category: None,
        bonuses: Bonuses {
            attack: Offensive {
                stab: 30,
                slash: 30,
                crush: 30,
                magic: 30,
                ranged: 30,
            },
            defence: Defensive {
                stab: 30,
                slash: 30,
                crush: 30,
                magic: 30,
                ranged: 30,
            },
            strength: StrengthBonuses {
                melee: 15,
                ranged: 15,
                magic: 10.0,
            },
            prayer: 10,
        },
        is_two_handed: None,
        attack_range: None,
    };

    let emperor_ring = Equipment {
        name: "Emperor ring".to_string(),
        id: -1,
        version: Some("Emperor ring".to_string()),
        image: "Emperor ring.png".to_string(),
        slot: "ring".to_string(),
        speed: None,
        category: None,
        bonuses: Bonuses {
            attack: Offensive {
                stab: 30,
                slash: 30,
                crush: 30,
                magic: 30,
                ranged: 30,
            },
            defence: Defensive {
                stab: 30,
                slash: 30,
                crush: 30,
                magic: 30,
                ranged: 30,
            },
            strength: StrengthBonuses {
                melee: 15,
                ranged: 15,
                magic: 5.0,
            },
            prayer: 10,
        },
        is_two_handed: None,
        attack_range: None,
    };

    let devils_element = Equipment {
        name: "Devil's element".to_string(),
        id: -1,
        version: Some("Devil's element".to_string()),
        image: "Devil's element.png".to_string(),
        slot: "shield".to_string(),
        speed: None,
        category: None,
        bonuses: Bonuses {
            attack: Offensive {
                stab: 0,
                slash: 0,
                crush: 0,
                magic: 20,
                ranged: 0,
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
                ranged: 0,
                magic: 6.0,
            },
            prayer: 3,
        },
        is_two_handed: None,
        attack_range: None,
    };

    let crystal_blessing = Equipment {
        name: "Crystal blessing".to_string(),
        id: -1,
        version: Some("Crystal blessing".to_string()),
        image: "Crystal blessing.png".to_string(),
        slot: "ammo".to_string(),
        speed: None,
        category: None,
        bonuses: Bonuses {
            attack: Offensive {
                stab: 0,
                slash: 0,
                crush: 0,
                magic: 0,
                ranged: 0,
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
                ranged: 0,
                magic: 0.0,
            },
            prayer: 5,
        },
        is_two_handed: None,
        attack_range: None,
    };

    for equipment in vec![
        drygore_blowpipe,
        dogsword,
        thunder_khopesh,
        thousand_dragon_ward,
        gloves_of_the_damned,
        amulet_of_the_monarchs,
        emperor_ring,
        devils_element,
        crystal_blessing,
        natures_reprisal,
        sunlight_spear,
        sunlit_bracers,
    ] {
        push_to_db(&mut stmt, &equipment)?;
        push_to_db_flat(&mut stmt_flat, &equipment)?;
    }

    println!("SQLite database created successfully");

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
            .header(
                "User-Agent",
                "osrs-dps-calc (https://github.com/weirdgloop/osrs-dps-calc)",
            )
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

fn push_to_db(stmt: &mut rusqlite::Statement, equipment: &Equipment) -> rusqlite::Result<()> {
    stmt.execute(params![
        equipment.name,
        equipment.version,
        serde_json::to_string(&equipment).unwrap_or_default()
    ])?;

    Ok(())
}

fn push_to_db_flat(stmt: &mut rusqlite::Statement, equipment: &Equipment) -> rusqlite::Result<()> {
    stmt.execute(params![
        equipment.id,
        equipment.name,
        equipment.version,
        equipment.image,
        equipment.slot,
        equipment.speed,
        equipment.category,
        equipment.bonuses.attack.stab,
        equipment.bonuses.attack.slash,
        equipment.bonuses.attack.crush,
        equipment.bonuses.attack.magic,
        equipment.bonuses.attack.ranged,
        equipment.bonuses.defence.stab,
        equipment.bonuses.defence.slash,
        equipment.bonuses.defence.crush,
        equipment.bonuses.defence.magic,
        equipment.bonuses.defence.ranged,
        equipment.bonuses.strength.melee,
        equipment.bonuses.strength.ranged,
        equipment.bonuses.strength.magic,
        equipment.bonuses.prayer,
        equipment.is_two_handed,
        equipment.attack_range
    ])?;

    Ok(())
}
