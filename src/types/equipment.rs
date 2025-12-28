use lazy_static::lazy_static;

use crate::constants::*;
use serde::{Deserialize, Deserializer};
use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use strum_macros::{Display, EnumIter};

lazy_static! {
    static ref EQUIPMENT_JSON: PathBuf =
        fs::canonicalize("src/databases/equipment.json").expect("Failed to get database path");
}

// Intermediate struct for JSON deserialization
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct EquipmentJson {
    pub name: String,
    pub version: Option<String>,
    pub slot: String,
    pub image: String,
    pub speed: Option<i32>,
    pub category: Option<String>,
    pub bonuses: EquipmentBonuses,
    pub is_two_handed: Option<bool>,
    pub attack_range: Option<i8>,
}

impl EquipmentJson {
    pub fn into_weapon(self) -> Result<Weapon, Box<dyn std::error::Error>> {
        if self.slot != "weapon" {
            return Err(
                format!("Item '{}' is not a weapon (slot: {})", self.name, self.slot).into(),
            );
        }

        let combat_styles = match self.category {
            Some(category) => Weapon::get_styles_from_weapon_type(&category),
            None => return Err("weapon missing category field".into()),
        };

        let speed = self.speed.ok_or("Weapon missing speed field")?;
        let attack_range = self
            .attack_range
            .ok_or("Weapon missing attack_range field")?;
        let is_two_handed = self
            .is_two_handed
            .ok_or("Weapon missing is_two_handed field")?;

        let weapon = Weapon {
            name: self.name,
            version: self.version,
            bonuses: self.bonuses,
            slot: GearSlot::Weapon,
            speed,
            base_speed: speed,
            attack_range,
            is_two_handed,
            spec_cost: None,
            poison_severity: 0,
            combat_styles,
            is_staff: false,
            image: self.image,
        };

        Ok(weapon)
    }

    pub fn into_armor(self) -> Result<Armor, Box<dyn std::error::Error>> {
        if self.slot == "weapon" {
            return Err(format!("Item '{}' is a weapon, not armor", self.name).into());
        }

        Ok(Armor {
            name: self.name,
            version: self.version,
            bonuses: self.bonuses,
            slot: parse_gear_slot(self.slot)?,
            image: self.image,
        })
    }
}

// Slots in which a player can equip gear
#[derive(Debug, PartialEq, Eq, Hash, Default, Deserialize, Clone, Display, Copy)]
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

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Gear {
    pub head: Option<Armor>,
    pub neck: Option<Armor>,
    pub cape: Option<Armor>,
    pub ammo: Option<Armor>,
    pub second_ammo: Option<Armor>,
    pub weapon: Weapon, // Default to unarmed, which is still a weapon
    pub shield: Option<Armor>,
    pub body: Option<Armor>,
    pub legs: Option<Armor>,
    pub hands: Option<Armor>,
    pub feet: Option<Armor>,
    pub ring: Option<Armor>,
}

impl Gear {
    pub fn is_wearing(&self, gear_name: &str, version: Option<&str>) -> bool {
        // Check if the player is wearing the specified piece of gear

        let matches = |opt: &Option<Armor>| -> bool {
            opt.as_ref()
                .is_some_and(|a| a.name == gear_name && a.version.as_deref() == version)
        };

        self.weapon.name == gear_name && self.weapon.version.as_deref() == version
            || matches(&self.head)
            || matches(&self.neck)
            || matches(&self.cape)
            || matches(&self.ammo)
            || matches(&self.second_ammo)
            || matches(&self.shield)
            || matches(&self.body)
            || matches(&self.legs)
            || matches(&self.feet)
            || matches(&self.hands)
            || matches(&self.ring)
    }

    pub fn is_wearing_any_version(&self, gear_name: &str) -> bool {
        // Same as is_wearing() but allows for any version to match
        self.weapon.name == gear_name
            || [
                &self.head,
                &self.neck,
                &self.cape,
                &self.ammo,
                &self.shield,
                &self.body,
                &self.legs,
                &self.feet,
                &self.hands,
                &self.ring,
            ]
            .iter()
            .filter_map(|slot| slot.as_ref())
            .any(|armor| armor.name == gear_name)
    }

    pub fn is_wearing_any<I>(&self, gear_names: I) -> bool
    where
        I: IntoIterator<Item = (&'static str, Option<&'static str>)>,
    {
        // Check if the player is wearing any item in the provided Vec
        gear_names
            .into_iter()
            .any(|gear_name| self.is_wearing(gear_name.0, gear_name.1))
    }

    pub fn is_wearing_all<I>(&self, gear_names: I) -> bool
    where
        I: IntoIterator<Item = (&'static str, Option<&'static str>)>,
    {
        // Check if the player is wearing all items in the provided Vec
        gear_names
            .into_iter()
            .all(|gear_name| self.is_wearing(gear_name.0, gear_name.1))
    }

    pub fn is_quiver_bonus_valid(&self) -> bool {
        // Check if the player is wearing a quiver and using a weapon with bolts or arrows
        self.cape.as_ref().is_some_and(|cape| {
            cape.name == "Dizana's quiver"
                && cape.matches_version("Charged")
                && self.weapon.uses_bolts_or_arrows()
                && (self
                    .ammo
                    .as_ref()
                    .is_some_and(|ammo| ammo.is_bolt_or_arrow())
                    || self
                        .second_ammo
                        .as_ref()
                        .is_some_and(|ammo| ammo.is_bolt_or_arrow()))
        })
    }
}

// Combat types, e.g., stab, slash, crush, magic, etc.
#[derive(Debug, PartialEq, Eq, Hash, Default, Copy, Clone, EnumIter, Deserialize, Display)]
pub enum CombatType {
    None,
    Stab,
    Slash,
    #[default]
    Crush, // Default because unarmed (punch) uses crush
    #[strum(to_string = "Light Ranged")]
    Light,
    #[strum(to_string = "Standard Ranged")]
    Standard,
    #[strum(to_string = "Heavy Ranged")]
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
#[derive(Debug, PartialEq, Eq, Hash, Default, Deserialize, Clone, Copy)]
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

impl CombatOption {
    pub fn new(combat_type: CombatType, stance: CombatStance) -> Self {
        CombatOption {
            combat_type,
            stance,
        }
    }
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
pub trait Equipment: Any {
    fn set_info(
        &mut self,
        item_name: &str,
        version: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Retrieve item data from the SQLite database in a JSON format
        let json_content = fs::read_to_string(EQUIPMENT_JSON.as_path())?;

        // Pass it to the set_fields_from_json method to set the item stats
        self.set_fields_from_json(&json_content, item_name, version)
    }

    fn set_fields_from_json(
        &mut self,
        json: &str,
        item_name: &str,
        version: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn as_any(&self) -> &dyn Any;
    fn name(&self) -> &str;
    fn get_image_path(&self) -> &str;
    fn slot(&self) -> GearSlot;
}

// Any equippable item that is not a weapon
#[derive(Debug, PartialEq, Default, Deserialize, Clone)]
pub struct Armor {
    pub name: String,
    pub version: Option<String>,
    pub bonuses: EquipmentBonuses,
    pub slot: GearSlot,
    pub image: String,
}

impl Equipment for Armor {
    fn set_fields_from_json(
        &mut self,
        json: &str,
        item_name: &str,
        version: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let all_items: Vec<EquipmentJson> = serde_json::from_str(json)?;
        let version_string = version.map(|v| v.to_string());
        let matched_item = all_items
            .into_iter()
            .find(|a| a.name == item_name && a.version == version_string)
            .ok_or("Equipment not found")?;

        *self = matched_item.into_armor()?;

        Ok(())
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_image_path(&self) -> &str {
        self.image.as_str()
    }

    fn slot(&self) -> GearSlot {
        self.slot
    }
}

impl fmt::Display for Armor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(version) = &self.version {
            write!(f, "{} ({})", self.name, version)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

impl Armor {
    pub fn new(name: &str, version: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        // Create a new Armor struct from item name and version (optional)
        let mut armor = Armor::default();
        armor.set_info(name, version)?;
        Ok(armor)
    }

    pub fn is_valid_ranged_ammo(&self) -> bool {
        // Check if an ammo slot item can be used as ranged ammo
        !self.name.contains("blessing")
            && !["Ghommal's lucky penny", "Mith grapple", "Hallowed grapple"]
                .contains(&self.name.as_str())
            && self.slot == GearSlot::Ammo
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
        self.version.as_ref().is_some_and(|v| v.contains(version))
    }
}

fn parse_gear_slot(slot: String) -> Result<GearSlot, Box<dyn std::error::Error>> {
    // Translate a gear slot string into an enum

    let trimmed = slot.replace('\"', "");

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
        "weapon" => Err("Tried to create armor from a weapon name".into()),
        _ => Err(format!("Unknown slot: {slot}").into()),
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
    pub spec_cost: Option<u8>, // Not implemented for anything yet
    #[serde(default)]
    pub poison_severity: u8, // May be restructured to use Poison/Venom struct, or removed
    #[serde(rename(deserialize = "category"))]
    #[serde(deserialize_with = "deserialize_combat_styles")]
    pub combat_styles: HashMap<CombatStyle, CombatOption>,
    #[serde(default)]
    pub is_staff: bool, // Will be set in new() method
    pub image: String,
}

impl Equipment for Weapon {
    fn set_fields_from_json(
        &mut self,
        json: &str,
        item_name: &str,
        version: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let all_items: Vec<EquipmentJson> = serde_json::from_str(json)?;
        let version_string = version.map(|v| v.to_string());
        let matching_item = all_items
            .into_iter()
            .find(|a| a.name == item_name && a.version == version_string)
            .ok_or("Equipment not found")?;

        let mut weapon = matching_item.into_weapon()?;

        // Check if the item is a staff that can cast spells
        if weapon.combat_styles.contains_key(&CombatStyle::Spell) {
            weapon.is_staff = true;
        }

        // Set base speed and item slot
        weapon.base_speed = weapon.speed;
        weapon.slot = GearSlot::Weapon;

        // Set spec cost, if applicable
        let spec_cost = SPEC_COSTS.iter().find(|w| w.0 == weapon.name);
        if let Some(cost) = spec_cost {
            weapon.spec_cost = Some(cost.1);
        }

        *self = weapon;

        Ok(())
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_image_path(&self) -> &str {
        self.image.as_str()
    }

    fn slot(&self) -> GearSlot {
        self.slot
    }
}

impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(version) = &self.version {
            write!(f, "{} ({})", self.name, version)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

impl Default for Weapon {
    fn default() -> Weapon {
        // Default case is unarmed
        Weapon {
            name: String::from("Unarmed"),
            version: None,
            bonuses: EquipmentBonuses::default(),
            slot: GearSlot::Weapon,
            speed: 5,
            base_speed: 5,
            attack_range: 0,
            is_two_handed: false,
            spec_cost: None,
            poison_severity: 0,
            combat_styles: Weapon::get_styles_from_weapon_type("Unarmed"),
            is_staff: false,
            image: String::new(),
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
    pub fn new(name: &str, version: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut weapon = Weapon::default();
        weapon.set_info(name, version)?;
        Ok(weapon)
    }

    pub fn uses_bolts_or_arrows(&self) -> bool {
        // Check if the weapon fires bolts or arrows (used for determining quiver bonuses)
        !NON_BOLT_OR_ARROW_AMMO
            .iter()
            .any(|(name, _)| name == &self.name)
            && self.combat_styles.contains_key(&CombatStyle::Rapid)
    }

    pub fn matches_version(&self, version: &str) -> bool {
        self.version.as_ref().is_some_and(|v| v.contains(version))
    }

    pub fn get_styles_from_weapon_type(weapon_type: &str) -> HashMap<CombatStyle, CombatOption> {
        // Translate a weapon type string into a set of combat styles that it can use
        match weapon_type {
            "2h Sword" => HashMap::from([
                (
                    CombatStyle::Chop,
                    CombatOption::new(CombatType::Slash, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Slash,
                    CombatOption::new(CombatType::Slash, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Smash,
                    CombatOption::new(CombatType::Crush, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Block,
                    CombatOption::new(CombatType::Slash, CombatStance::Defensive),
                ),
            ]),
            "Axe" => HashMap::from([
                (
                    CombatStyle::Chop,
                    CombatOption::new(CombatType::Slash, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Hack,
                    CombatOption::new(CombatType::Slash, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Smash,
                    CombatOption::new(CombatType::Crush, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Block,
                    CombatOption::new(CombatType::Slash, CombatStance::Defensive),
                ),
            ]),
            "Banner" => HashMap::from([
                (
                    CombatStyle::Lunge,
                    CombatOption::new(CombatType::Stab, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Swipe,
                    CombatOption::new(CombatType::Slash, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Pound,
                    CombatOption::new(CombatType::Crush, CombatStance::Controlled),
                ),
                (
                    CombatStyle::Block,
                    CombatOption::new(CombatType::Stab, CombatStance::Defensive),
                ),
            ]),
            "Blunt" => HashMap::from([
                (
                    CombatStyle::Pound,
                    CombatOption::new(CombatType::Crush, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Pummel,
                    CombatOption::new(CombatType::Crush, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Block,
                    CombatOption::new(CombatType::Crush, CombatStance::Defensive),
                ),
            ]),
            "Bludgeon" => HashMap::from([
                (
                    CombatStyle::Pound,
                    CombatOption::new(CombatType::Crush, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Pummel,
                    CombatOption::new(CombatType::Crush, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Smash,
                    CombatOption::new(CombatType::Crush, CombatStance::Aggressive),
                ),
            ]),
            "Bulwark" => HashMap::from([
                (
                    CombatStyle::Pummel,
                    CombatOption::new(CombatType::Crush, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Block,
                    CombatOption::new(CombatType::None, CombatStance::None),
                ),
            ]),
            "Claw" => HashMap::from([
                (
                    CombatStyle::Chop,
                    CombatOption::new(CombatType::Slash, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Slash,
                    CombatOption::new(CombatType::Slash, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Lunge,
                    CombatOption::new(CombatType::Stab, CombatStance::Controlled),
                ),
                (
                    CombatStyle::Block,
                    CombatOption::new(CombatType::Slash, CombatStance::Defensive),
                ),
            ]),
            "Partisan" => HashMap::from([
                (
                    CombatStyle::Stab,
                    CombatOption::new(CombatType::Stab, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Lunge,
                    CombatOption::new(CombatType::Stab, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Pound,
                    CombatOption::new(CombatType::Crush, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Block,
                    CombatOption::new(CombatType::Stab, CombatStance::Defensive),
                ),
            ]),
            "Pickaxe" => HashMap::from([
                (
                    CombatStyle::Spike,
                    CombatOption::new(CombatType::Stab, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Impale,
                    CombatOption::new(CombatType::Stab, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Smash,
                    CombatOption::new(CombatType::Crush, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Block,
                    CombatOption::new(CombatType::Stab, CombatStance::Defensive),
                ),
            ]),
            "Polearm" => HashMap::from([
                (
                    CombatStyle::Jab,
                    CombatOption::new(CombatType::Stab, CombatStance::Controlled),
                ),
                (
                    CombatStyle::Swipe,
                    CombatOption::new(CombatType::Slash, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Fend,
                    CombatOption::new(CombatType::Stab, CombatStance::Defensive),
                ),
            ]),
            "Polestaff" => HashMap::from([
                (
                    CombatStyle::Bash,
                    CombatOption::new(CombatType::Crush, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Pound,
                    CombatOption::new(CombatType::Crush, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Block,
                    CombatOption::new(CombatType::Crush, CombatStance::Defensive),
                ),
            ]),
            "Scythe" => HashMap::from([
                (
                    CombatStyle::Reap,
                    CombatOption::new(CombatType::Slash, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Chop,
                    CombatOption::new(CombatType::Slash, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Jab,
                    CombatOption::new(CombatType::Crush, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Block,
                    CombatOption::new(CombatType::Slash, CombatStance::Defensive),
                ),
            ]),
            "Slash Sword" => HashMap::from([
                (
                    CombatStyle::Chop,
                    CombatOption::new(CombatType::Slash, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Slash,
                    CombatOption::new(CombatType::Slash, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Lunge,
                    CombatOption::new(CombatType::Stab, CombatStance::Controlled),
                ),
                (
                    CombatStyle::Block,
                    CombatOption::new(CombatType::Slash, CombatStance::Defensive),
                ),
            ]),
            "Spear" => HashMap::from([
                (
                    CombatStyle::Lunge,
                    CombatOption::new(CombatType::Stab, CombatStance::Controlled),
                ),
                (
                    CombatStyle::Swipe,
                    CombatOption::new(CombatType::Slash, CombatStance::Controlled),
                ),
                (
                    CombatStyle::Pound,
                    CombatOption::new(CombatType::Crush, CombatStance::Controlled),
                ),
                (
                    CombatStyle::Block,
                    CombatOption::new(CombatType::Stab, CombatStance::Defensive),
                ),
            ]),
            "Spiked" => HashMap::from([
                (
                    CombatStyle::Pound,
                    CombatOption::new(CombatType::Crush, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Pummel,
                    CombatOption::new(CombatType::Crush, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Spike,
                    CombatOption::new(CombatType::Stab, CombatStance::Controlled),
                ),
                (
                    CombatStyle::Block,
                    CombatOption::new(CombatType::Crush, CombatStance::Defensive),
                ),
            ]),
            "Stab Sword" => HashMap::from([
                (
                    CombatStyle::Stab,
                    CombatOption::new(CombatType::Stab, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Slash,
                    CombatOption::new(CombatType::Slash, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Lunge,
                    CombatOption::new(CombatType::Stab, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Block,
                    CombatOption::new(CombatType::Stab, CombatStance::Defensive),
                ),
            ]),
            "Unarmed" => HashMap::from([
                (
                    CombatStyle::Punch,
                    CombatOption::new(CombatType::Crush, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Kick,
                    CombatOption::new(CombatType::Crush, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Block,
                    CombatOption::new(CombatType::Crush, CombatStance::Defensive),
                ),
            ]),
            "Whip" => HashMap::from([
                (
                    CombatStyle::Flick,
                    CombatOption::new(CombatType::Slash, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Lash,
                    CombatOption::new(CombatType::Slash, CombatStance::Controlled),
                ),
                (
                    CombatStyle::Deflect,
                    CombatOption::new(CombatType::Slash, CombatStance::Defensive),
                ),
            ]),
            "Bow" => HashMap::from([
                (
                    CombatStyle::Accurate,
                    CombatOption::new(CombatType::Standard, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Rapid,
                    CombatOption::new(CombatType::Standard, CombatStance::Rapid),
                ),
                (
                    CombatStyle::Longrange,
                    CombatOption::new(CombatType::Standard, CombatStance::Longrange),
                ),
            ]),
            "Crossbow" => HashMap::from([
                (
                    CombatStyle::Accurate,
                    CombatOption::new(CombatType::Heavy, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Rapid,
                    CombatOption::new(CombatType::Heavy, CombatStance::Rapid),
                ),
                (
                    CombatStyle::Longrange,
                    CombatOption::new(CombatType::Heavy, CombatStance::Longrange),
                ),
            ]),
            "Thrown" => HashMap::from([
                (
                    CombatStyle::Accurate,
                    CombatOption::new(CombatType::Light, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Rapid,
                    CombatOption::new(CombatType::Light, CombatStance::Rapid),
                ),
                (
                    CombatStyle::Longrange,
                    CombatOption::new(CombatType::Light, CombatStance::Longrange),
                ),
            ]),
            "Chinchompa" => HashMap::from([
                (
                    CombatStyle::ShortFuse,
                    CombatOption::new(CombatType::Heavy, CombatStance::ShortFuse),
                ),
                (
                    CombatStyle::MediumFuse,
                    CombatOption::new(CombatType::Heavy, CombatStance::MediumFuse),
                ),
                (
                    CombatStyle::LongFuse,
                    CombatOption::new(CombatType::Heavy, CombatStance::LongFuse),
                ),
            ]),
            "Bladed Staff" => HashMap::from([
                (
                    CombatStyle::Jab,
                    CombatOption::new(CombatType::Stab, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Swipe,
                    CombatOption::new(CombatType::Slash, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Fend,
                    CombatOption::new(CombatType::Crush, CombatStance::Defensive),
                ),
                (
                    CombatStyle::DefensiveSpell,
                    CombatOption::new(CombatType::Magic, CombatStance::DefensiveAutocast),
                ),
                (
                    CombatStyle::Spell,
                    CombatOption::new(CombatType::Magic, CombatStance::Autocast),
                ),
            ]),
            "Powered Staff" => HashMap::from([
                (
                    CombatStyle::Accurate,
                    CombatOption::new(CombatType::Magic, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Longrange,
                    CombatOption::new(CombatType::Magic, CombatStance::Longrange),
                ),
            ]),
            "Staff" => HashMap::from([
                (
                    CombatStyle::Bash,
                    CombatOption::new(CombatType::Crush, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Pound,
                    CombatOption::new(CombatType::Crush, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Fend,
                    CombatOption::new(CombatType::Crush, CombatStance::Defensive),
                ),
                (
                    CombatStyle::DefensiveSpell,
                    CombatOption::new(CombatType::Magic, CombatStance::DefensiveAutocast),
                ),
                (
                    CombatStyle::Spell,
                    CombatOption::new(CombatType::Magic, CombatStance::Autocast),
                ),
            ]),
            "Salamander" => HashMap::from([
                (
                    CombatStyle::Scorch,
                    CombatOption::new(CombatType::Slash, CombatStance::Aggressive),
                ),
                (
                    CombatStyle::Flare,
                    CombatOption::new(CombatType::Standard, CombatStance::Accurate),
                ),
                (
                    CombatStyle::Blaze,
                    CombatOption::new(CombatType::Magic, CombatStance::Defensive),
                ),
            ]),
            _ => HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_weapon() {
        let weapon = Weapon::default();
        assert_eq!(weapon.name, "Unarmed");
        assert_eq!(weapon.bonuses, EquipmentBonuses::default());
        assert_eq!(weapon.speed, 5);
        assert_eq!(weapon.base_speed, 5);
        assert_eq!(weapon.attack_range, 0);
        assert!(!weapon.is_two_handed);
        assert_eq!(weapon.spec_cost, None);
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
        let weapon = Weapon::new("Abyssal whip", None).unwrap();
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
        let armor = Armor::new("Rune platebody", None).unwrap();
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

    #[test]
    fn test_spec_cost() {
        let voidwaker = Weapon::new("Voidwaker", None).unwrap();
        assert_eq!(voidwaker.spec_cost, Some(50));
    }
}
