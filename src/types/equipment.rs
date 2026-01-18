use crate::constants::*;
use crate::error::GearError;
use serde::{Deserialize, Deserializer};
use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use strum_macros::{Display, EnumIter};

const EQUIPMENT_JSON_STR: &str = include_str!("../databases/equipment.json");

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
    pub fn into_weapon(self) -> Result<Weapon, GearError> {
        if self.slot != "weapon" {
            return Err(GearError::NotAWeapon {
                item_name: self.name,
                slot: self.slot,
            });
        }

        let combat_styles = match self.category {
            Some(category) => Weapon::get_styles_from_weapon_type(&category),
            None => return Err(GearError::MissingWeaponCategory(self.name)),
        };

        let speed = self
            .speed
            .ok_or(GearError::MissingWeaponSpeed(self.name.clone()))?;
        let attack_range = self
            .attack_range
            .ok_or(GearError::MissingAttackRange(self.name.clone()))?;
        let is_two_handed = self
            .is_two_handed
            .ok_or(GearError::MissingTwoHandedField(self.name.clone()))?;

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

    pub fn into_armor(self) -> Result<Armor, GearError> {
        if self.slot == "weapon" {
            return Err(GearError::NotArmor(self.name));
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

    pub fn builder() -> GearBuilder {
        GearBuilder::default()
    }
}

/// Builder for constructing `Gear` instances by item name and version.
///
/// # Example
/// ```
/// use osrs::types::equipment::Gear;
///
/// let gear = Gear::builder()
///     .head("Torva full helm", None)
///     .body("Torva platebody", None)
///     .legs("Torva platelegs", None)
///     .weapon("Osmumten's fang", None)
///     .build()?;
///
/// Ok(())
/// ```
#[derive(Debug, Clone, Default)]
pub struct GearBuilder {
    head: Option<(String, Option<String>)>,
    neck: Option<(String, Option<String>)>,
    cape: Option<(String, Option<String>)>,
    ammo: Option<(String, Option<String>)>,
    second_ammo: Option<(String, Option<String>)>,
    weapon: Option<(String, Option<String>)>,
    shield: Option<(String, Option<String>)>,
    body: Option<(String, Option<String>)>,
    legs: Option<(String, Option<String>)>,
    hands: Option<(String, Option<String>)>,
    feet: Option<(String, Option<String>)>,
    ring: Option<(String, Option<String>)>,
}

impl GearBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the head slot item.
    pub fn head(mut self, name: &str, version: Option<&str>) -> Self {
        self.head = Some((name.to_string(), version.map(|v| v.to_string())));
        self
    }

    /// Set the neck slot item.
    pub fn neck(mut self, name: &str, version: Option<&str>) -> Self {
        self.neck = Some((name.to_string(), version.map(|v| v.to_string())));
        self
    }

    /// Set the cape slot item.
    pub fn cape(mut self, name: &str, version: Option<&str>) -> Self {
        self.cape = Some((name.to_string(), version.map(|v| v.to_string())));
        self
    }

    /// Set the ammo slot item.
    pub fn ammo(mut self, name: &str, version: Option<&str>) -> Self {
        self.ammo = Some((name.to_string(), version.map(|v| v.to_string())));
        self
    }

    /// Set the second ammo slot item (for quiver).
    pub fn second_ammo(mut self, name: &str, version: Option<&str>) -> Self {
        self.second_ammo = Some((name.to_string(), version.map(|v| v.to_string())));
        self
    }

    /// Set the weapon slot item.
    pub fn weapon(mut self, name: &str, version: Option<&str>) -> Self {
        self.weapon = Some((name.to_string(), version.map(|v| v.to_string())));
        self
    }

    /// Set the shield slot item.
    pub fn shield(mut self, name: &str, version: Option<&str>) -> Self {
        self.shield = Some((name.to_string(), version.map(|v| v.to_string())));
        self
    }

    /// Set the body slot item.
    pub fn body(mut self, name: &str, version: Option<&str>) -> Self {
        self.body = Some((name.to_string(), version.map(|v| v.to_string())));
        self
    }

    /// Set the legs slot item.
    pub fn legs(mut self, name: &str, version: Option<&str>) -> Self {
        self.legs = Some((name.to_string(), version.map(|v| v.to_string())));
        self
    }

    /// Set the hands slot item.
    pub fn hands(mut self, name: &str, version: Option<&str>) -> Self {
        self.hands = Some((name.to_string(), version.map(|v| v.to_string())));
        self
    }

    /// Set the feet slot item.
    pub fn feet(mut self, name: &str, version: Option<&str>) -> Self {
        self.feet = Some((name.to_string(), version.map(|v| v.to_string())));
        self
    }

    /// Set the ring slot item.
    pub fn ring(mut self, name: &str, version: Option<&str>) -> Self {
        self.ring = Some((name.to_string(), version.map(|v| v.to_string())));
        self
    }

    /// Build the `Gear` instance.
    pub fn build(self) -> Result<Gear, GearError> {
        let mut gear = Gear::default();

        if let Some((name, version)) = self.head {
            gear.head = Some(Armor::new(&name, version.as_deref())?);
        }

        if let Some((name, version)) = self.neck {
            gear.neck = Some(Armor::new(&name, version.as_deref())?);
        }

        if let Some((name, version)) = self.cape {
            gear.cape = Some(Armor::new(&name, version.as_deref())?);
        }

        if let Some((name, version)) = self.ammo {
            gear.ammo = Some(Armor::new(&name, version.as_deref())?);
        }

        if let Some((name, version)) = self.second_ammo {
            gear.second_ammo = Some(Armor::new(&name, version.as_deref())?);
        }

        if let Some((name, version)) = self.weapon {
            gear.weapon = Weapon::new(&name, version.as_deref())?;
        }

        if let Some((name, version)) = self.shield {
            gear.shield = Some(Armor::new(&name, version.as_deref())?);
        }

        if let Some((name, version)) = self.body {
            gear.body = Some(Armor::new(&name, version.as_deref())?);
        }

        if let Some((name, version)) = self.legs {
            gear.legs = Some(Armor::new(&name, version.as_deref())?);
        }

        if let Some((name, version)) = self.hands {
            gear.hands = Some(Armor::new(&name, version.as_deref())?);
        }

        if let Some((name, version)) = self.feet {
            gear.feet = Some(Armor::new(&name, version.as_deref())?);
        }

        if let Some((name, version)) = self.ring {
            gear.ring = Some(Armor::new(&name, version.as_deref())?);
        }

        Ok(gear)
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
#[derive(Debug, PartialEq, Eq, Hash, Default, Deserialize, Clone, Copy, Display)]
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
    fn set_info(&mut self, item_name: &str, version: Option<&str>) -> Result<(), GearError> {
        self.set_fields_from_json(EQUIPMENT_JSON_STR, item_name, version)
    }

    fn set_fields_from_json(
        &mut self,
        json: &str,
        item_name: &str,
        version: Option<&str>,
    ) -> Result<(), GearError>;
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
    ) -> Result<(), GearError> {
        let all_items: Vec<EquipmentJson> = serde_json::from_str(json)?;
        let version_string = version.map(|v| v.to_string());
        let matched_item = all_items
            .into_iter()
            .find(|a| a.name == item_name && a.version == version_string)
            .ok_or(GearError::EquipmentNotFound {
                name: item_name.to_string(),
                version: version_string,
            })?;

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
    pub fn new(name: &str, version: Option<&str>) -> Result<Self, GearError> {
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

fn parse_gear_slot(slot: String) -> Result<GearSlot, GearError> {
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
        "weapon" => unreachable!(),
        _ => Err(GearError::UnknownSlot(slot)),
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
    ) -> Result<(), GearError> {
        let all_items: Vec<EquipmentJson> = serde_json::from_str(json)?;
        let version_string = version.map(|v| v.to_string());
        let matching_item = all_items
            .into_iter()
            .find(|a| a.name == item_name && a.version == version_string)
            .ok_or(GearError::EquipmentNotFound {
                name: self.name.clone(),
                version: self.version.clone(),
            })?;

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
    pub fn new(name: &str, version: Option<&str>) -> Result<Self, GearError> {
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
        let combat_style = weapon
            .combat_styles
            .get(&CombatStyle::Punch)
            .expect("Combat style not found.");
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
        let weapon = Weapon::new("Abyssal whip", None).expect("Error creating equipment.");
        assert_eq!(weapon.name, "Abyssal whip");
        assert_eq!(weapon.slot, GearSlot::Weapon);
        assert_eq!(weapon.bonuses.attack.slash, 82);
        assert_eq!(weapon.bonuses.strength.melee, 82);
        let combat_style = weapon
            .combat_styles
            .get(&CombatStyle::Flick)
            .expect("Combat style not found.");
        assert_eq!(combat_style.combat_type, CombatType::Slash);
        assert_eq!(combat_style.stance, CombatStance::Accurate);
    }

    #[test]
    fn test_set_armor_info() {
        let armor = Armor::new("Rune platebody", None).expect("Error creating equipment.");
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
        let voidwaker = Weapon::new("Voidwaker", None).expect("Error creating equipment.");
        assert_eq!(voidwaker.spec_cost, Some(50));
    }

    #[test]
    fn test_gear_builder_default() {
        let gear = Gear::builder().build().expect("Error building gear.");
        assert_eq!(gear, Gear::default());
    }

    #[test]
    fn test_gear_builder_with_weapon() {
        let gear = Gear::builder()
            .weapon("Abyssal whip", None)
            .build()
            .expect("Error building gear.");

        assert_eq!(gear.weapon.name, "Abyssal whip");
        assert_eq!(gear.weapon.bonuses.attack.slash, 82);
    }

    #[test]
    fn test_gear_builder_with_armor() {
        let gear = Gear::builder()
            .head("Torva full helm", None)
            .body("Torva platebody", None)
            .legs("Torva platelegs", None)
            .build()
            .expect("Error building gear.");

        assert!(gear.head.is_some());
        assert_eq!(gear.head.as_ref().unwrap().name, "Torva full helm");
        assert!(gear.body.is_some());
        assert_eq!(gear.body.as_ref().unwrap().name, "Torva platebody");
        assert!(gear.legs.is_some());
        assert_eq!(gear.legs.as_ref().unwrap().name, "Torva platelegs");
    }

    #[test]
    fn test_gear_builder_full_setup() {
        let gear = Gear::builder()
            .head("Torva full helm", None)
            .neck("Amulet of torture", None)
            .cape("Infernal cape", None)
            .ammo("Rada's blessing 4", None)
            .body("Torva platebody", None)
            .legs("Torva platelegs", None)
            .hands("Ferocious gloves", None)
            .feet("Primordial boots", None)
            .ring("Ultor ring", None)
            .weapon("Osmumten's fang", None)
            .shield("Avernic defender", None)
            .build()
            .expect("Error building gear.");

        assert!(gear.is_wearing("Torva full helm", None));
        assert!(gear.is_wearing("Amulet of torture", None));
        assert!(gear.is_wearing("Infernal cape", None));
        assert!(gear.is_wearing("Rada's blessing 4", None));
        assert!(gear.is_wearing("Torva platebody", None));
        assert!(gear.is_wearing("Torva platelegs", None));
        assert!(gear.is_wearing("Ferocious gloves", None));
        assert!(gear.is_wearing("Primordial boots", None));
        assert!(gear.is_wearing("Ultor ring", None));
        assert!(gear.is_wearing("Osmumten's fang", None));
        assert!(gear.is_wearing("Avernic defender", None));
    }

    #[test]
    fn test_gear_builder_invalid_item() {
        let result = Gear::builder().head("Not a real item", None).build();

        assert!(result.is_err());
    }
}
