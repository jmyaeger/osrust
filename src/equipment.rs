use lazy_static::lazy_static;

use crate::constants::*;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use strum_macros::EnumIter;

lazy_static! {
    static ref EQUIPMENT_DB: PathBuf =
        fs::canonicalize("src/databases/equipment.db").expect("Failed to get database path");
    static ref EQUIPMENT_FLAT_DB: PathBuf =
        fs::canonicalize("src/databases/equipment_flattened.db")
            .expect("Failed to get database path");
}

// Slots in which a player can equip gear
#[derive(Debug, PartialEq, Eq, Hash, Default, Deserialize, Clone)]
pub enum GearSlot {
    #[default]
    None,
    Head,
    Neck,
    Body,
    Legs,
    Hands,
    Feet,
    Ring,
    Ammo,
    Weapon,
    Shield,
    Cape,
}

// Combat types, e.g., stab, slash, crush, magic, etc.
#[derive(Debug, PartialEq, Eq, Hash, Default, Copy, Clone, EnumIter, Deserialize)]
pub enum CombatType {
    None,
    Stab,
    Slash,
    #[default]
    Crush, // Default because unarmed (punch) uses crush
    Light,
    Standard,
    Heavy,
    Magic,
    Ranged,
}

// Combat stance (determines stance bonus)
#[derive(Debug, PartialEq, Eq, Hash, Default, Copy, Clone, Deserialize)]
pub enum CombatStance {
    None,
    #[default]
    Accurate, // Default because punch uses accurate stance
    Aggressive,
    Defensive,
    Controlled,
    Rapid,
    Longrange,
    ShortFuse,
    MediumFuse,
    LongFuse,
    DefensiveAutocast,
    Autocast,
    ManualCast,
}

// Name of the combat style as seen in the weapon interface
#[derive(Debug, PartialEq, Eq, Hash, Default, Deserialize, Clone)]
pub enum CombatStyle {
    Chop,
    Slash,
    Smash,
    Block,
    Hack,
    Lunge,
    Swipe,
    Pound,
    Pummel,
    Spike,
    Impale,
    Stab,
    Jab,
    Fend,
    Bash,
    Reap,
    #[default]
    Punch,
    Kick,
    Flick,
    Lash,
    Deflect,
    Accurate,
    Rapid,
    Longrange,
    ShortFuse,
    MediumFuse,
    LongFuse,
    DefensiveSpell,
    ManualCast,
    Spell,
    Scorch,
    Flare,
    Blaze,
}

// Contains the type and stance, to be associated with a CombatStyle
#[derive(Debug, PartialEq, Eq, Hash, Default, Deserialize, Clone)]
pub struct CombatOption {
    pub combat_type: CombatType,
    pub stance: CombatStance,
}

// Equipment stat bonuses for each combat style (generally used for accuracy/defense bonuses)
#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Deserialize)]
pub struct StyleBonus {
    pub stab: i32,
    pub slash: i32,
    pub crush: i32,
    pub ranged: i32,
    pub magic: i32,
}

impl StyleBonus {
    pub fn add_bonuses(&mut self, other: &StyleBonus) {
        // Add another set of bonuses to the totals
        self.stab += other.stab;
        self.slash += other.slash;
        self.crush += other.crush;
        self.ranged += other.ranged;
        self.magic += other.magic;
    }
}

// Equipment strength bonuses for each primary style
#[derive(Debug, PartialEq, Default, Deserialize, Clone)]
pub struct StrengthBonus {
    pub melee: i32,
    pub ranged: i32,
    pub magic: f32,
}

impl StrengthBonus {
    pub fn add_bonuses(&mut self, other: &StrengthBonus) {
        // Add another set of bonuses to the totals
        self.melee += other.melee;
        self.ranged += other.ranged;
        self.magic += other.magic;
    }
}

// Collection of all equipment bonuses for an item
#[derive(Debug, Default, PartialEq, Deserialize, Clone)]
pub struct EquipmentBonuses {
    pub attack: StyleBonus,
    pub defence: StyleBonus,
    pub strength: StrengthBonus,
    pub prayer: i32,
}

impl EquipmentBonuses {
    pub fn add_bonuses(&mut self, other: &EquipmentBonuses) {
        // Add another set of bonuses to the totals of each type of bonus
        self.attack.add_bonuses(&other.attack);
        self.defence.add_bonuses(&other.defence);
        self.strength.add_bonuses(&other.strength);
        self.prayer += other.prayer;
    }
}

// Equipment trait to provide common method for Armor and Weapon structs
pub trait Equipment {
    fn set_info(
        &mut self,
        item_name: &str,
        version: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Retrieve item data from the SQLite database in a JSON format
        let conn = Connection::open(EQUIPMENT_DB.as_path())?;
        let json: String = if version.is_some() {
            conn.query_row(
                "SELECT data FROM equipment WHERE name = ?1 AND version = ?2",
                params![item_name, version.unwrap_or("")],
                |row| row.get(0),
            )?
        } else {
            conn.query_row(
                "SELECT data FROM equipment WHERE name = ? AND version IS NULL",
                params![item_name],
                |row| row.get(0),
            )?
        };

        // Pass it to the set_fields_from_json method to set the item stats
        self.set_fields_from_json(&json)
    }

    fn set_fields_from_json(&mut self, json: &str) -> Result<(), Box<dyn std::error::Error>>;
}

// Any equippable item that is not a weapon
#[derive(Debug, PartialEq, Default, Deserialize, Clone)]
pub struct Armor {
    pub name: String,
    pub version: Option<String>,
    pub bonuses: EquipmentBonuses,
    #[serde(deserialize_with = "parse_gear_slot")]
    pub slot: GearSlot,
}

impl Equipment for Armor {
    fn set_fields_from_json(&mut self, json: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Can deserialize directly into an Armor struct from JSON data
        let armor = serde_json::from_str(json)?;

        *self = armor;
        Ok(())
    }
}

impl Armor {
    pub fn new(name: &str, version: Option<&str>) -> Self {
        // Create a new Armor struct from item name and version (optional)
        let mut armor = Armor::default();
        armor.set_info(name, version).unwrap();
        armor
    }

    pub fn is_valid_ranged_ammo(&self) -> bool {
        // Check if an ammo slot item can be used as ranged ammo
        !self.name.contains("blessing")
            && !["Ghommal's lucky penny", "Mith grapple", "Hallowed grapple"]
                .contains(&self.name.as_str())
            && self.slot != GearSlot::Ammo
    }

    pub fn is_bolt_or_arrow(&self) -> bool {
        self.is_bolt() || self.is_arrow()
    }

    pub fn is_bolt(&self) -> bool {
        self.name.contains("bolts")
    }

    pub fn is_arrow(&self) -> bool {
        self.name.contains("arrow")
    }

    pub fn matches_version(&self, version: &str) -> bool {
        self.version.as_ref().map_or(false, |v| v.contains(version))
    }
}

fn parse_gear_slot<'de, D>(deserializer: D) -> Result<GearSlot, D::Error>
where
    D: Deserializer<'de>,
{
    // Translate a gear slot string into an enum

    let s: String = String::deserialize(deserializer)?;

    let trimmed = s.replace('\"', "");

    match trimmed.as_str() {
        "head" => Ok(GearSlot::Head),
        "neck" => Ok(GearSlot::Neck),
        "cape" => Ok(GearSlot::Cape),
        "body" => Ok(GearSlot::Body),
        "legs" => Ok(GearSlot::Legs),
        "shield" => Ok(GearSlot::Shield),
        "feet" => Ok(GearSlot::Feet),
        "hands" => Ok(GearSlot::Hands),
        "ring" => Ok(GearSlot::Ring),
        "ammo" => Ok(GearSlot::Ammo),
        "weapon" => Err(serde::de::Error::custom(
            "Tried to create armor from a weapon name",
        )),
        _ => Err(serde::de::Error::custom(format!("Unknown slot: {}", s))),
    }
}

// Needs to be a separate struct from Armor because of additional fields
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Weapon {
    pub name: String,
    pub version: Option<String>,
    pub bonuses: EquipmentBonuses,
    #[serde(skip)]
    pub slot: GearSlot, // Can skip deserializing because it's always a weapon
    pub speed: i32,
    #[serde(skip)]
    pub base_speed: i32, // Will be set during new() method
    pub attack_range: i8,
    pub is_two_handed: bool,
    #[serde(default)]
    pub spec_cost: u8, // Not implemented for anything yet
    #[serde(default)]
    pub poison_severity: u8, // May be restructured to use Poison/Venom struct, or removed
    #[serde(rename(deserialize = "category"))]
    #[serde(deserialize_with = "deserialize_combat_styles")]
    pub combat_styles: HashMap<CombatStyle, CombatOption>,
    #[serde(default)]
    pub is_staff: bool, // Will be set in new() method
}

impl Equipment for Weapon {
    fn set_fields_from_json(&mut self, json: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Deserialize the JSON data into a Weapon struct
        let mut weapon: Weapon = serde_json::from_str(json)?;

        // Check if the item is a staff that can cast spells
        if weapon.combat_styles.contains_key(&CombatStyle::Spell) {
            weapon.is_staff = true;
        }

        // Set base speed and item slot
        weapon.base_speed = weapon.speed;
        weapon.slot = GearSlot::Weapon;

        *self = weapon;

        Ok(())
    }
}

impl Default for Weapon {
    fn default() -> Weapon {
        // Default case is unarmed
        Weapon {
            name: String::new(),
            version: None,
            bonuses: EquipmentBonuses::default(),
            slot: GearSlot::Weapon,
            speed: 0,
            base_speed: 0,
            attack_range: 0,
            is_two_handed: false,
            spec_cost: 0,
            poison_severity: 0,
            combat_styles: Weapon::get_styles_from_weapon_type("Unarmed"),
            is_staff: false,
        }
    }
}

pub fn deserialize_combat_styles<'de, D>(
    deserializer: D,
) -> Result<HashMap<CombatStyle, CombatOption>, D::Error>
where
    D: Deserializer<'de>,
{
    let weapon_type = String::deserialize(deserializer)?;
    Ok(Weapon::get_styles_from_weapon_type(weapon_type.as_str()))
}

impl Weapon {
    pub fn new(name: &str, version: Option<&str>) -> Self {
        let mut weapon = Weapon::default();
        weapon.set_info(name, version).unwrap();
        weapon
    }

    pub fn uses_bolts_or_arrows(&self) -> bool {
        // Check if the weapon fires bolts or arrows (used for determining quiver bonuses)
        !NON_BOLT_OR_ARROW_AMMO
            .iter()
            .any(|(name, _)| name == &self.name)
            && self.combat_styles.contains_key(&CombatStyle::Rapid)
    }

    pub fn matches_version(&self, version: &str) -> bool {
        self.version.as_ref().map_or(false, |v| v.contains(version))
    }

    pub fn get_styles_from_weapon_type(weapon_type: &str) -> HashMap<CombatStyle, CombatOption> {
        // Translate a weapon type string into a set of combat styles that it can use
        match weapon_type {
            "2h Sword" => HashMap::from([
                (
                    CombatStyle::Chop,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Slash,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Smash,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Axe" => HashMap::from([
                (
                    CombatStyle::Chop,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Hack,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Smash,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Banner" => HashMap::from([
                (
                    CombatStyle::Lunge,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Swipe,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Pound,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Controlled,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Blunt" => HashMap::from([
                (
                    CombatStyle::Pound,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Pummel,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Bludgeon" => HashMap::from([
                (
                    CombatStyle::Pound,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Pummel,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Smash,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
            ]),
            "Bulwark" => HashMap::from([
                (
                    CombatStyle::Pummel,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::None,
                        stance: CombatStance::None,
                    },
                ),
            ]),
            "Claw" => HashMap::from([
                (
                    CombatStyle::Chop,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Slash,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Lunge,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Partisan" => HashMap::from([
                (
                    CombatStyle::Stab,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Lunge,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Pound,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Pickaxe" => HashMap::from([
                (
                    CombatStyle::Spike,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Impale,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Smash,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Polearm" => HashMap::from([
                (
                    CombatStyle::Jab,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Controlled,
                    },
                ),
                (
                    CombatStyle::Swipe,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Fend,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Polestaff" => HashMap::from([
                (
                    CombatStyle::Bash,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Pound,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Scythe" => HashMap::from([
                (
                    CombatStyle::Reap,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Chop,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Jab,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Slash Sword" => HashMap::from([
                (
                    CombatStyle::Chop,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Slash,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Lunge,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Controlled,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Spear" => HashMap::from([
                (
                    CombatStyle::Lunge,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Controlled,
                    },
                ),
                (
                    CombatStyle::Swipe,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Controlled,
                    },
                ),
                (
                    CombatStyle::Pound,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Controlled,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Spiked" => HashMap::from([
                (
                    CombatStyle::Pound,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Pummel,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Spike,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Controlled,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Stab Sword" => HashMap::from([
                (
                    CombatStyle::Stab,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Slash,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Lunge,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Unarmed" => HashMap::from([
                (
                    CombatStyle::Punch,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Kick,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Block,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Whip" => HashMap::from([
                (
                    CombatStyle::Flick,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Lash,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Controlled,
                    },
                ),
                (
                    CombatStyle::Deflect,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            "Bow" => HashMap::from([
                (
                    CombatStyle::Accurate,
                    CombatOption {
                        combat_type: CombatType::Standard,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Rapid,
                    CombatOption {
                        combat_type: CombatType::Standard,
                        stance: CombatStance::Rapid,
                    },
                ),
                (
                    CombatStyle::Longrange,
                    CombatOption {
                        combat_type: CombatType::Standard,
                        stance: CombatStance::Longrange,
                    },
                ),
            ]),
            "Crossbow" => HashMap::from([
                (
                    CombatStyle::Accurate,
                    CombatOption {
                        combat_type: CombatType::Heavy,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Rapid,
                    CombatOption {
                        combat_type: CombatType::Heavy,
                        stance: CombatStance::Rapid,
                    },
                ),
                (
                    CombatStyle::Longrange,
                    CombatOption {
                        combat_type: CombatType::Heavy,
                        stance: CombatStance::Longrange,
                    },
                ),
            ]),
            "Thrown" => HashMap::from([
                (
                    CombatStyle::Accurate,
                    CombatOption {
                        combat_type: CombatType::Light,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Rapid,
                    CombatOption {
                        combat_type: CombatType::Light,
                        stance: CombatStance::Rapid,
                    },
                ),
                (
                    CombatStyle::Longrange,
                    CombatOption {
                        combat_type: CombatType::Light,
                        stance: CombatStance::Longrange,
                    },
                ),
            ]),
            "Chinchompa" => HashMap::from([
                (
                    CombatStyle::ShortFuse,
                    CombatOption {
                        combat_type: CombatType::Heavy,
                        stance: CombatStance::ShortFuse,
                    },
                ),
                (
                    CombatStyle::MediumFuse,
                    CombatOption {
                        combat_type: CombatType::Heavy,
                        stance: CombatStance::MediumFuse,
                    },
                ),
                (
                    CombatStyle::LongFuse,
                    CombatOption {
                        combat_type: CombatType::Heavy,
                        stance: CombatStance::LongFuse,
                    },
                ),
            ]),
            "Bladed Staff" => HashMap::from([
                (
                    CombatStyle::Jab,
                    CombatOption {
                        combat_type: CombatType::Stab,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Swipe,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Fend,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Defensive,
                    },
                ),
                (
                    CombatStyle::DefensiveSpell,
                    CombatOption {
                        combat_type: CombatType::Magic,
                        stance: CombatStance::DefensiveAutocast,
                    },
                ),
                (
                    CombatStyle::Spell,
                    CombatOption {
                        combat_type: CombatType::Magic,
                        stance: CombatStance::Autocast,
                    },
                ),
            ]),
            "Powered Staff" => HashMap::from([
                (
                    CombatStyle::Accurate,
                    CombatOption {
                        combat_type: CombatType::Magic,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Longrange,
                    CombatOption {
                        combat_type: CombatType::Magic,
                        stance: CombatStance::Longrange,
                    },
                ),
            ]),
            "Staff" => HashMap::from([
                (
                    CombatStyle::Bash,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Pound,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Fend,
                    CombatOption {
                        combat_type: CombatType::Crush,
                        stance: CombatStance::Defensive,
                    },
                ),
                (
                    CombatStyle::DefensiveSpell,
                    CombatOption {
                        combat_type: CombatType::Magic,
                        stance: CombatStance::DefensiveAutocast,
                    },
                ),
                (
                    CombatStyle::Spell,
                    CombatOption {
                        combat_type: CombatType::Magic,
                        stance: CombatStance::Autocast,
                    },
                ),
            ]),
            "Salamander" => HashMap::from([
                (
                    CombatStyle::Scorch,
                    CombatOption {
                        combat_type: CombatType::Slash,
                        stance: CombatStance::Aggressive,
                    },
                ),
                (
                    CombatStyle::Flare,
                    CombatOption {
                        combat_type: CombatType::Ranged,
                        stance: CombatStance::Accurate,
                    },
                ),
                (
                    CombatStyle::Blaze,
                    CombatOption {
                        combat_type: CombatType::Magic,
                        stance: CombatStance::Defensive,
                    },
                ),
            ]),
            _ => HashMap::new(),
        }
    }
}

pub fn get_slot_name(item_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Get the name of the slot for an item (by name)
    let conn = Connection::open(EQUIPMENT_FLAT_DB.as_path())?;
    let slot: String = conn.query_row(
        "SELECT slot FROM equipment WHERE name = ?",
        params![item_name],
        |row| row.get(0),
    )?;

    Ok(slot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_weapon() {
        let weapon = Weapon::default();
        assert_eq!(weapon.name, "");
        assert_eq!(weapon.bonuses, EquipmentBonuses::default());
        assert_eq!(weapon.speed, 0);
        assert_eq!(weapon.base_speed, 0);
        assert_eq!(weapon.attack_range, 0);
        assert!(!weapon.is_two_handed);
        assert_eq!(weapon.spec_cost, 0);
        assert_eq!(weapon.slot, GearSlot::Weapon);
        let combat_style = weapon.combat_styles.get(&CombatStyle::Punch).unwrap();
        assert_eq!(combat_style.combat_type, CombatType::Crush);
        assert_eq!(combat_style.stance, CombatStance::Accurate);
    }

    #[test]
    fn test_default_armor() {
        let armor = Armor::default();
        assert_eq!(armor.name, "");
        assert_eq!(armor.bonuses, EquipmentBonuses::default());
        assert_eq!(armor.slot, GearSlot::None);
    }

    #[test]
    fn test_set_weapon_info() {
        let weapon = Weapon::new("Abyssal whip", None);
        assert_eq!(weapon.name, "Abyssal whip");
        assert_eq!(weapon.slot, GearSlot::Weapon);
        assert_eq!(weapon.bonuses.attack.slash, 82);
        assert_eq!(weapon.bonuses.strength.melee, 82);
        let combat_style = weapon.combat_styles.get(&CombatStyle::Flick).unwrap();
        assert_eq!(combat_style.combat_type, CombatType::Slash);
        assert_eq!(combat_style.stance, CombatStance::Accurate);
    }

    #[test]
    fn test_set_armor_info() {
        let armor = Armor::new("Rune platebody", None);
        assert_eq!(armor.name, "Rune platebody");
        assert_eq!(armor.slot, GearSlot::Body);
        assert_eq!(
            armor.bonuses.defence,
            StyleBonus {
                stab: 82,
                slash: 80,
                crush: 72,
                magic: -6,
                ranged: 80,
            }
        );
    }
}
