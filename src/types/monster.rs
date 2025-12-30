use lazy_static::lazy_static;

use crate::calc::rolls;
use crate::combat::attacks::effects::{BurnType, CombatEffect};
use crate::combat::attacks::standard::Hit;
use crate::combat::thralls::Thrall;
use crate::constants::*;
use crate::error::MonsterError;
use crate::types::equipment::{CombatStyle, CombatType};
use crate::types::player::Player;
use crate::types::stats::MonsterStats;
use rand::Rng;
use serde::{Deserialize, de::Error};
use std::cmp::{max, min};
use std::fs;
use std::path::PathBuf;
use strum_macros::Display;

lazy_static! {
    static ref MONSTER_JSON: PathBuf =
        fs::canonicalize("src/databases/monsters.json").expect("Failed to get database path");
}

// Enum for combat stats
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum CombatStat {
    Attack,
    Strength,
    Defence,
    Ranged,
    Magic,
}

// Struct for stat drain
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct StatDrain {
    pub stat: CombatStat,
    pub cap: Option<u32>,
}

impl StatDrain {
    pub fn new(stat: CombatStat, cap: Option<u32>) -> StatDrain {
        StatDrain { stat, cap }
    }
}

// Enum for monster attributes
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Attribute {
    Demon,
    Draconic,
    Fiery,
    Flying,
    Golem,
    Icy,
    Kalphite,
    Leafy,
    Penance,
    Rat,
    Shade,
    Spectral,
    Undead,
    Vampyre(u8), // Value is the vampyre tier (1, 2, 3)
    Xerician,
}

// Offensive bonus for a each primary combat style
#[derive(Debug, Eq, PartialEq, Hash, Default, Clone, Deserialize)]
pub struct AttackBonus {
    pub melee: i32,
    pub ranged: i32,
    pub magic: i32,
}

// Defensive bonuses for all combat styles
#[derive(Debug, Eq, PartialEq, Hash, Default, Clone, Deserialize)]
pub struct MonsterDefBonuses {
    pub stab: i32,
    pub slash: i32,
    pub crush: i32,
    pub light: i32,
    pub standard: i32,
    pub heavy: i32,
    pub magic: i32,
    #[serde(skip)]
    pub magic_base: i32,
}

type MonsterStrengthBonus = AttackBonus; // Uses the same fields as AttackBonus

// All offensive and defensive bonuses for a monster
#[derive(Debug, PartialEq, Default, Clone, Deserialize)]
pub struct MonsterBonuses {
    pub attack: AttackBonus,
    pub strength: MonsterStrengthBonus,
    pub defence: MonsterDefBonuses,
    #[serde(default)]
    pub flat_armour: i32,
}

// Damage sources from which the monster is immune
#[derive(Debug, Eq, PartialEq, Hash, Default, Clone, Deserialize)]
pub struct Immunities {
    // Only poison and venom are in the database right now
    pub poison: bool,
    pub venom: bool,
    #[serde(default)]
    pub cannon: bool,
    #[serde(default)]
    pub thrall: bool,
    pub freeze: u32,
    #[serde(deserialize_with = "deserialize_burn_immunity")]
    pub burn: Option<BurnType>,
    #[serde(default)]
    pub melee: bool,
    #[serde(default)]
    pub ranged: bool,
    #[serde(default)]
    pub magic: bool,
}

// Enum for monster attack types
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Display)]
pub enum AttackType {
    Stab,
    Slash,
    Crush,
    Melee,
    Magic,
    Ranged,
    Special,
    None,
}

impl<T: AsRef<str>> From<T> for AttackType {
    fn from(attack_type: T) -> Self {
        match attack_type.as_ref().to_lowercase().as_str() {
            "stab" => Self::Stab,
            "slash" => Self::Slash,
            "crush" => Self::Crush,
            "melee" => Self::Melee,
            "magic" => Self::Magic,
            "ranged" => Self::Ranged,
            "" => Self::None,
            _ => Self::Special,
        }
    }
}

// Maximum hit value for a given style
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct MonsterMaxHit {
    pub value: u32,
    pub style: AttackType,
}

impl MonsterMaxHit {
    pub fn new(value: u32, style: AttackType) -> MonsterMaxHit {
        MonsterMaxHit { value, style }
    }
}

// Contains a variety of information about a monster - may separate into multiple structs later
#[derive(Debug, PartialEq, Default, Clone, Deserialize)]
pub struct MonsterInfo {
    pub id: Option<i32>,
    pub name: String,
    pub version: Option<String>,
    pub combat_level: u32,
    pub size: u32,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_attributes")]
    pub attributes: Option<Vec<Attribute>>,
    #[serde(deserialize_with = "deserialize_attack_styles")]
    pub attack_styles: Option<Vec<AttackType>>,
    pub weakness: Option<ElementalWeakness>,
    #[serde(deserialize_with = "deserialize_attack_speed")]
    pub attack_speed: Option<u32>,
    #[serde(default)]
    pub poison_severity: u32, // Will likely rework this to use CombatEffects
    #[serde(default)]
    pub freeze_duration: u32,
    #[serde(default)]
    pub toa_level: u32,
    #[serde(default)]
    pub toa_path_level: u32,
}

fn deserialize_attributes<'de, D>(deserializer: D) -> Result<Option<Vec<Attribute>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // Translate attributes from strings into equivalent enums
    let attributes: Option<Vec<String>> = Option::deserialize(deserializer)?;
    attributes
        .map(|attrs| {
            attrs
                .into_iter()
                .map(|attr| match attr.as_str() {
                    "demon" => Ok(Attribute::Demon),
                    "dragon" => Ok(Attribute::Draconic),
                    "fiery" => Ok(Attribute::Fiery),
                    "flying" => Ok(Attribute::Flying),
                    "golem" => Ok(Attribute::Golem),
                    "icy" => Ok(Attribute::Icy),
                    "kalphite" => Ok(Attribute::Kalphite),
                    "leafy" => Ok(Attribute::Leafy),
                    "penance" => Ok(Attribute::Penance),
                    "rat" => Ok(Attribute::Rat),
                    "shade" => Ok(Attribute::Shade),
                    "spectral" => Ok(Attribute::Spectral),
                    "undead" => Ok(Attribute::Undead),
                    "vampyre1" => Ok(Attribute::Vampyre(1)),
                    "vampyre2" => Ok(Attribute::Vampyre(2)),
                    "vampyre3" => Ok(Attribute::Vampyre(3)),
                    "xerician" => Ok(Attribute::Xerician),
                    _ => Err(D::Error::custom(format!(
                        "Unknown monster attribute: {attr}"
                    ))),
                })
                .collect()
        })
        .transpose()
}

fn deserialize_attack_speed<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let attack_speed = i32::deserialize(deserializer)?;

    if attack_speed <= 0 {
        Ok(None)
    } else {
        Ok(Some(attack_speed as u32))
    }
}

fn deserialize_max_hits<'de, D>(deserializer: D) -> Result<Option<Vec<MonsterMaxHit>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let max_hits: Option<Vec<String>> = Option::deserialize(deserializer)?;

    Ok(max_hits.and_then(|hits| {
        let parsed: Vec<_> = hits
            .into_iter()
            .filter_map(|hit| {
                let mut parts = hit.split('(');
                let value_str = parts.next()?;
                let value = value_str.trim().parse::<u32>().ok()?;
                let style = parts
                    .next()
                    .unwrap_or_default()
                    .to_string()
                    .replace(')', "");

                Some(MonsterMaxHit::new(value, AttackType::from(style)))
            })
            .collect();

        if parsed.is_empty() {
            None
        } else {
            Some(parsed)
        }
    }))
}

fn deserialize_attack_styles<'de, D>(deserializer: D) -> Result<Option<Vec<AttackType>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let attack_styles: Option<Vec<String>> = Option::deserialize(deserializer)?;
    let mut attack_types: Option<Vec<AttackType>> = None;
    if let Some(attack_styles) = attack_styles {
        attack_types = Some(Vec::new());
        for style in attack_styles.iter() {
            let attack_type = AttackType::from(style);
            attack_types.as_mut().unwrap().push(attack_type);
        }
    }

    Ok(attack_types)
}

fn deserialize_burn_immunity<'de, D>(deserializer: D) -> Result<Option<BurnType>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let burn_immunity: Option<String> = Option::deserialize(deserializer)?;
    burn_immunity
        .map(|s| {
            s.parse::<BurnType>()
                .map_err(|e| D::Error::custom(e.to_string()))
        })
        .transpose()
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct MonsterAttRolls {
    stab: i32,
    slash: i32,
    crush: i32,
    ranged: i32,
    magic: i32,
}

impl MonsterAttRolls {
    pub fn get(&self, combat_type: CombatType) -> i32 {
        match combat_type {
            CombatType::Stab => self.stab,
            CombatType::Slash => self.slash,
            CombatType::Crush => self.crush,
            CombatType::Light => self.ranged,
            CombatType::Standard => self.ranged,
            CombatType::Heavy => self.ranged,
            CombatType::Ranged => self.ranged,
            CombatType::Magic => self.magic,
            CombatType::None => 0,
        }
    }

    pub fn set(&mut self, combat_type: CombatType, value: i32) {
        match combat_type {
            CombatType::Stab => self.stab = value,
            CombatType::Slash => self.slash = value,
            CombatType::Crush => self.crush = value,
            CombatType::Light => self.ranged = value,
            CombatType::Standard => self.ranged = value,
            CombatType::Heavy => self.ranged = value,
            CombatType::Ranged => self.ranged = value,
            CombatType::Magic => self.magic = value,
            CombatType::None => {}
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct MonsterDefRolls {
    stab: i32,
    slash: i32,
    crush: i32,
    ranged: i32,
    light: i32,
    standard: i32,
    heavy: i32,
    magic: i32,
}

impl MonsterDefRolls {
    pub fn get(&self, combat_type: CombatType) -> i32 {
        match combat_type {
            CombatType::Stab => self.stab,
            CombatType::Slash => self.slash,
            CombatType::Crush => self.crush,
            CombatType::Light => self.light,
            CombatType::Standard => self.standard,
            CombatType::Heavy => self.heavy,
            CombatType::Ranged => self.ranged,
            CombatType::Magic => self.magic,
            CombatType::None => 0,
        }
    }

    pub fn set(&mut self, combat_type: CombatType, value: i32) {
        match combat_type {
            CombatType::Stab => self.stab = value,
            CombatType::Slash => self.slash = value,
            CombatType::Crush => self.crush = value,
            CombatType::Light => self.light = value,
            CombatType::Standard => self.standard = value,
            CombatType::Heavy => self.heavy = value,
            CombatType::Ranged => self.ranged = value,
            CombatType::Magic => self.magic = value,
            CombatType::None => {}
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct ElementalWeakness {
    pub element: String,
    pub severity: i64,
}

/// Entry in an HP scaling table containing precomputed values for a given HP level.
/// Currently only used for Vardorvis, but can add other stats if necessary
/// in the future
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct HpScalingEntry {
    pub strength: u32,
    pub defence: u32,
    pub max_hit: u32,
    pub def_rolls: MonsterDefRolls,
}

/// Precomputed scaling table for monsters whose stats scale with HP.

#[derive(Debug, PartialEq, Clone, Default)]
pub struct HpScalingTable {
    table: Vec<HpScalingEntry>,
}

impl HpScalingTable {
    pub fn new(table: Vec<HpScalingEntry>) -> Self {
        Self { table }
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    #[inline]
    pub fn get(&self, hp: usize) -> &HpScalingEntry {
        &self.table[hp]
    }

    #[inline]
    pub fn apply(&self, monster: &mut Monster) {
        let hp = monster.stats.hitpoints.current as usize;
        let entry = &self.table[hp];

        monster.stats.strength.current = entry.strength;
        monster.stats.defence.current = entry.defence;

        if let Some(hits) = &mut monster.max_hits {
            hits[0].value = entry.max_hit;
        }

        monster.def_rolls = entry.def_rolls;
    }
}

// Overall monster struct
#[derive(Debug, PartialEq, Clone, Deserialize, Default)]
pub struct Monster {
    pub info: MonsterInfo,
    pub stats: MonsterStats,
    pub bonuses: MonsterBonuses,
    pub immunities: Immunities,
    #[serde(skip)]
    pub def_rolls: MonsterDefRolls,
    #[serde(skip)]
    pub base_def_rolls: MonsterDefRolls,
    #[serde(default)]
    #[serde(rename(deserialize = "max_hit"))]
    #[serde(deserialize_with = "deserialize_max_hits")]
    pub max_hits: Option<Vec<MonsterMaxHit>>,
    #[serde(skip)]
    pub base_att_rolls: MonsterAttRolls,
    #[serde(skip)]
    pub att_rolls: MonsterAttRolls,
    #[serde(skip)]
    pub active_effects: Vec<CombatEffect>, // Will move poison/venom here
    #[serde(skip)]
    pub hp_scaling_table: Option<HpScalingTable>,
}

impl Monster {
    pub fn from_json_str(
        name: &str,
        version: Option<&str>,
        json_str: &str,
    ) -> Result<Monster, MonsterError> {
        // Create a monster by name and version (optional)

        let string_version = version.map(|v| v.to_string());
        let all_monsters: Vec<Monster> = serde_json::from_str(json_str)?;

        // Find the monster matching the given name and version
        let mut monster = all_monsters
            .into_iter()
            .find(|m| m.info.name == name && m.info.version == string_version)
            .ok_or(MonsterError::MonsterNotFound(name.to_string()))?;
        // Set defence level floor
        monster.set_defence_floor();

        // Set base magic def bonus (to allow it to be drained by the eye of ayak)
        monster.bonuses.defence.magic_base = monster.bonuses.defence.magic;

        // Calculate base defence rolls and copy to live defence rolls
        monster.base_def_rolls = rolls::monster_def_rolls(&monster);
        monster.def_rolls.clone_from(&monster.base_def_rolls);

        // Calculate base attack rolls and copy to live attack rolls
        monster.base_att_rolls = rolls::monster_att_rolls(&monster);
        monster.att_rolls.clone_from(&monster.base_att_rolls);

        if let (Some(max_hits), Some(attack_styles)) =
            (&mut monster.max_hits, &monster.info.attack_styles)
        {
            if max_hits.len() == 1 && attack_styles.len() == 1 {
                max_hits[0].style = attack_styles[0];
            } else {
                for hit in max_hits.iter_mut() {
                    if hit.style == AttackType::Melee
                        && let Some(&melee_style) = attack_styles.iter().find(|&x| {
                            matches!(x, AttackType::Stab | AttackType::Slash | AttackType::Crush)
                        })
                    {
                        hit.style = melee_style;
                    }
                }
            }
        }

        Ok(monster)
    }
    pub fn new(name: &str, version: Option<&str>) -> Result<Monster, MonsterError> {
        let json_content = fs::read_to_string(MONSTER_JSON.as_path())?;
        Self::from_json_str(name, version, json_content.as_str())
    }

    pub fn attack(
        &mut self,
        player: &mut Player,
        attack_type: Option<AttackType>,
        rng: &mut rand::rngs::SmallRng,
        cap_hit: bool,
    ) -> Result<Hit, MonsterError> {
        // Perform an attack on a player

        let attack_type = if let Some(att_type) = attack_type {
            att_type
        } else if let Some(attack_styles) = &self.info.attack_styles
            && attack_styles.len() == 1
        {
            attack_styles[0]
        } else {
            return Err(MonsterError::AttackTypeNotSpecified(self.info.name.clone()));
        };

        let max_hit = self
            .max_hits
            .as_ref()
            .and_then(|hits| hits.iter().find(|h| h.style == attack_type))
            .ok_or_else(|| MonsterError::MaxHitNotFound {
                monster_name: self.info.name.clone(),
                attack_type: attack_type.to_string(),
            })?;

        let max_att_roll = match attack_type {
            AttackType::Stab => Ok(self.att_rolls.get(CombatType::Stab)),
            AttackType::Slash => Ok(self.att_rolls.get(CombatType::Slash)),
            AttackType::Crush => Ok(self.att_rolls.get(CombatType::Crush)),
            AttackType::Ranged => Ok(self.att_rolls.get(CombatType::Ranged)),
            AttackType::Magic => Ok(self.att_rolls.get(CombatType::Magic)),
            AttackType::Melee => Ok({
                (self.att_rolls.get(CombatType::Stab)
                    + self.att_rolls.get(CombatType::Slash)
                    + self.att_rolls.get(CombatType::Crush))
                    / 3
            }),
            AttackType::Special => Err(MonsterError::SpecialAttackNotSupported),
            AttackType::None => Err(MonsterError::NoneAttackNotSupported),
        }?;

        let att_roll = rng.random_range(0..max_att_roll + 1);

        let max_def_roll = match attack_type {
            AttackType::Stab => Ok(player.def_rolls.get(CombatType::Stab)),
            AttackType::Slash => Ok(player.def_rolls.get(CombatType::Slash)),
            AttackType::Crush => Ok(player.def_rolls.get(CombatType::Crush)),
            AttackType::Ranged => Ok(player.def_rolls.get(CombatType::Ranged)),
            AttackType::Magic => Ok(player.def_rolls.get(CombatType::Magic)),
            AttackType::Melee => Ok({
                (player.def_rolls.get(CombatType::Stab)
                    + player.def_rolls.get(CombatType::Slash)
                    + player.def_rolls.get(CombatType::Crush))
                    / 3
            }),
            AttackType::Special => Err(MonsterError::SpecialAttackNotSupported),
            AttackType::None => Err(MonsterError::NoneAttackNotSupported),
        }?;

        let def_roll = rng.random_range(0..max_def_roll + 1);

        let success = att_roll > def_roll;

        let mut damage = if success {
            rng.random_range(0..max_hit.value + 1)
        } else {
            0
        };

        if success {
            if player.is_wearing("Elysian spirit shield", None) && rng.random::<f64>() <= 0.7 {
                let reduction = max(1, damage / 4);
                damage -= reduction;
            } else if player.is_wearing("Dinh's bulwark", None)
                && player.attrs.active_style == CombatStyle::Block
            {
                damage = damage * 8 / 10;
            }

            if player.set_effects.full_justiciar {
                let defensive_bonus = match attack_type {
                    AttackType::Stab => Ok(player.bonuses.defence.stab),
                    AttackType::Slash => Ok(player.bonuses.defence.slash),
                    AttackType::Crush => Ok(player.bonuses.defence.crush),
                    AttackType::Ranged => Ok(player.bonuses.defence.ranged),
                    AttackType::Magic => Ok(player.bonuses.defence.magic),
                    AttackType::Melee => Ok({
                        (player.bonuses.defence.stab
                            + player.bonuses.defence.slash
                            + player.bonuses.defence.crush)
                            / 3
                    }),
                    AttackType::Special => Err(MonsterError::SpecialAttackNotSupported),
                    AttackType::None => Err(MonsterError::NoneAttackNotSupported),
                }?;

                damage -= damage * defensive_bonus as u32 / 3000;
            }
        }

        if cap_hit {
            damage = min(damage, player.stats.hitpoints.current);
        }

        Ok(Hit::new(damage, success))
    }

    pub fn scale_toa(&mut self) {
        // Scale the HP and defence rolls based on the toa_level field of the monster
        if TOA_MONSTERS.contains(&self.info.id.unwrap_or(0)) {
            self.scale_toa_hp();
            self.scale_toa_defence();
        }
    }
    fn scale_toa_hp(&mut self) {
        // Scale the HP based on the toa_level field of the monster
        let level_factor = if self.info.name.as_str() == "Core (Wardens)" {
            1 // Core's HP scaling is 75% lower than other monsters in ToA
        } else {
            4
        };

        // Every 5 levels increases the HP by 2% (0.4% per level), or 0.5% (0.1%) for Core
        let toa_level_bonus = 100 + (self.info.toa_level * level_factor / 10);

        // First path level increases the HP by 8%, then 5% per path level
        let toa_path_level_bonus = if self.info.toa_path_level == 0 {
            100
        } else {
            108 + (self.info.toa_path_level - 1) * 5
        };

        // Apply level scaling
        let level_scaled_hp = self.stats.hitpoints.base * toa_level_bonus / 100;

        // If the NPC is affected by path scaling, apply it
        self.stats.hitpoints.current = if TOA_PATH_MONSTERS.contains(&self.info.id.unwrap_or(0)) {
            let path_scaled_hp = level_scaled_hp * toa_path_level_bonus / 100;
            round_toa_hp(path_scaled_hp)
        } else {
            round_toa_hp(level_scaled_hp)
        };
    }

    fn scale_toa_defence(&mut self) {
        // Every 5 levels increases the defence rolls by 2% (0.4% per level)
        let toa_level_bonus = 1000 + self.info.toa_level * 4;
        for defence_type in [
            CombatType::Stab,
            CombatType::Slash,
            CombatType::Crush,
            CombatType::Light,
            CombatType::Standard,
            CombatType::Heavy,
            CombatType::Magic,
        ] {
            self.def_rolls.set(
                defence_type,
                self.base_def_rolls.get(defence_type) * toa_level_bonus as i32 / 1000,
            );
        }
    }

    pub fn tbow_bonuses(&self) -> (i32, i32) {
        // Calculate twisted bow attack and damage multipliers
        let magic_limit = if self
            .info
            .attributes
            .as_ref()
            .is_some_and(|attrs| attrs.contains(&Attribute::Xerician))
        {
            350 // Inside CoX
        } else {
            250 // Outside CoX
        };

        // Take the higher of the magic level and magic attack bonus, capped at the limit
        let highest_magic = min(
            magic_limit,
            max(self.stats.magic.current as i32, self.bonuses.attack.magic),
        );

        let tbow_m = highest_magic * 3 / 10; // Intermediate value
        let acc_bonus = min(
            TBOW_ACC_CAP,
            TBOW_ACC_CAP + (tbow_m * 10 - 10) / 100 - (tbow_m - 100).pow(2) / 100,
        );
        let dmg_bonus = min(
            TBOW_DMG_CAP,
            TBOW_DMG_CAP + (tbow_m * 10 - 14) / 100 - (tbow_m - 140).pow(2) / 100,
        );

        (acc_bonus, dmg_bonus)
    }

    fn has_attribute(&self, attr: Attribute) -> bool {
        self.info
            .attributes
            .as_ref()
            .is_some_and(|attrs| attrs.contains(&attr))
    }

    pub fn is_dragon(&self) -> bool {
        self.has_attribute(Attribute::Draconic)
    }

    pub fn is_demon(&self) -> bool {
        self.has_attribute(Attribute::Demon)
    }

    pub fn is_undead(&self) -> bool {
        self.has_attribute(Attribute::Undead)
    }

    pub fn is_kalphite(&self) -> bool {
        self.has_attribute(Attribute::Kalphite)
    }

    pub fn is_leafy(&self) -> bool {
        self.has_attribute(Attribute::Leafy)
    }

    pub fn is_golem(&self) -> bool {
        self.has_attribute(Attribute::Golem)
    }

    pub fn is_rat(&self) -> bool {
        self.has_attribute(Attribute::Rat)
    }

    pub fn is_fiery(&self) -> bool {
        self.has_attribute(Attribute::Fiery)
    }

    pub fn is_shade(&self) -> bool {
        self.has_attribute(Attribute::Shade)
    }

    pub fn vampyre_tier(&self) -> Option<u8> {
        if let Some(attrs) = self.info.attributes.as_ref() {
            for attr in attrs {
                if let Attribute::Vampyre(tier) = attr {
                    return Some(*tier);
                }
            }
        }
        None
    }

    pub fn is_in_wilderness(&self) -> bool {
        WILDERNESS_MONSTERS.contains(&self.info.name.as_str())
    }

    pub fn is_revenant(&self) -> bool {
        self.info.name.contains("Revenant")
    }

    pub fn is_toa_monster(&self) -> bool {
        TOA_MONSTERS.contains(&self.info.id.unwrap_or(0))
    }

    pub fn is_toa_path_monster(&self) -> bool {
        TOA_PATH_MONSTERS.contains(&self.info.id.unwrap_or(0))
    }

    pub fn heal(&mut self, amount: u32) {
        self.stats.hitpoints.restore(amount, None);
    }

    pub fn take_damage(&mut self, amount: u32) {
        self.stats.hitpoints.drain(amount);
    }

    pub fn is_freezable(&self) -> bool {
        self.immunities.freeze != 100 && !self.info.freeze_duration == 0
    }

    pub fn is_immune_to_weak_burn(&self) -> bool {
        self.immunities.burn.is_some()
    }

    pub fn is_immune_to_normal_burn(&self) -> bool {
        self.immunities.burn == Some(BurnType::Normal)
            || self.immunities.burn == Some(BurnType::Strong)
    }

    pub fn is_immune_to_strong_burn(&self) -> bool {
        self.immunities.burn == Some(BurnType::Strong)
    }

    pub fn regen_stats(&mut self) {
        self.stats.attack.restore(1, None);
        self.stats.strength.restore(1, None);
        self.stats.defence.restore(1, None);
        self.stats.ranged.restore(1, None);
        self.stats.magic.restore(1, None);
    }

    pub fn drain_stat(&mut self, stat: CombatStat, amount: u32, cap: Option<u32>) -> u32 {
        let mut amount = amount;
        let mut remainder = 0;

        if let Some(cap) = cap {
            amount = min(cap, amount);
        }
        match stat {
            CombatStat::Attack => {
                // Mod Ash tweet indicates that stats drain down to 1, not 0
                if self.stats.attack.current.saturating_sub(amount) < 1 {
                    remainder = amount - self.stats.attack.current + 1;
                    self.stats.attack.current = 1;
                } else {
                    self.stats.attack.drain(amount);
                }
            }
            CombatStat::Strength => {
                if self.stats.strength.current.saturating_sub(amount) < 1 {
                    remainder = amount - self.stats.strength.current + 1;
                    self.stats.strength.current = 1;
                } else {
                    self.stats.strength.drain(amount);
                }
            }
            CombatStat::Magic => {
                if self.stats.magic.current.saturating_sub(amount) < 1 {
                    remainder = amount - self.stats.magic.current + 1;
                    self.stats.magic.current = 1;
                } else {
                    self.stats.magic.drain(amount);
                }
                self.def_rolls = rolls::monster_def_rolls(self);
            }
            CombatStat::Ranged => {
                if self.stats.ranged.current.saturating_sub(amount) < 1 {
                    remainder = amount - self.stats.ranged.current + 1;
                    self.stats.ranged.current = 1;
                } else {
                    self.stats.ranged.drain(amount);
                }
            }
            CombatStat::Defence => {
                if self.stats.defence.current.saturating_sub(amount) < 1 {
                    remainder = amount - self.stats.defence.current + 1;
                    self.stats.defence.current = 1;
                } else {
                    self.stats.defence.drain(amount);
                }
                self.def_rolls = rolls::monster_def_rolls(self);
            }
        }

        self.base_def_rolls = rolls::monster_def_rolls(self);
        self.def_rolls.clone_from(&self.base_def_rolls);
        self.scale_toa();
        remainder
    }

    pub fn drain_stats_in_order(&mut self, total_amount: u32, drain_order: Vec<StatDrain>) {
        let mut amount = total_amount;
        for drain in drain_order {
            if amount == 0 {
                break;
            }

            amount = self.drain_stat(drain.stat, amount, drain.cap);
        }
    }

    pub fn reset(&mut self) {
        // Reset live stats, status effects, and defence rolls
        self.stats.reset_all();
        self.bonuses.defence.magic = self.bonuses.defence.magic_base;
        self.info.poison_severity = 0;
        self.info.freeze_duration = 0;
        self.base_def_rolls = rolls::monster_def_rolls(self);
        self.def_rolls = self.base_def_rolls;
        self.scale_toa();
        self.active_effects = Vec::new();
    }

    pub fn is_immune(&self, player: &Player) -> bool {
        // Determine if the monster is immune to the player's current attacks
        let combat_type = &player.combat_type();

        if combat_type == &CombatType::Magic
            && IMMUNE_TO_MAGIC_MONSTERS.contains(&self.info.id.unwrap_or(0))
            || (player.is_using_ranged()
                && IMMUNE_TO_RANGED_MONSTERS.contains(&self.info.id.unwrap_or(0)))
            || (player.is_using_melee()
                && (IMMUNE_TO_MELEE_MONSTERS.contains(&self.info.id.unwrap_or(0))
                    || (self.info.name == "Zulrah" && player.gear.weapon.attack_range < 2)))
        {
            return true;
        }

        if player.is_using_melee()
            && player.gear.weapon.attack_range < 2
            && IMMUNE_TO_NON_HALBERD_MELEE_DAMAGE_MONSTERS.contains(&self.info.id.unwrap_or(0))
        {
            return true;
        }

        if self.vampyre_tier() == Some(3) && !player.is_wearing_ivandis_weapon() {
            return true;
        }

        if !self.is_demon()
            && (player.is_wearing("Holy water", None) || player.is_using_demonbane_spell())
        {
            return true;
        }

        if self.info.name.contains("Guardian (Chambers")
            && (!player.is_using_melee() || !player.gear.weapon.name.contains("pickaxe"))
        {
            return true;
        }

        if self.is_leafy() && !player.is_wearing_leaf_bladed_weapon() {
            return true;
        }

        if !self.is_rat() && player.is_wearing_ratbone_weapon() {
            return true;
        }

        if self.info.name.as_str() == "Fire Warrior of Lesarkus"
            && (!player.is_using_ranged() || !player.is_wearing("Ice arrows", None))
        {
            return true;
        }

        // Fareed can only take damage from water spells or any ranged weapon that fires arrows
        if self.info.name.contains("Fareed")
            && (!player.is_using_water_spell()
                || (player.is_using_ranged()
                    && !player
                        .gear
                        .ammo
                        .as_ref()
                        .is_some_and(|ammo| ammo.name.contains("arrow"))))
        {
            return true;
        }

        if !self.info.name.contains("Verzik") && player.is_wearing("Dawnbringer", None) {
            // Dawnbringer is only usable inside the Verzik room (should check if usable on crabs)
            return true;
        }

        false
    }

    pub fn is_immune_to_thrall(&self, thrall: Thrall) -> bool {
        if thrall.attack_type() == AttackType::Melee
            && IMMUNE_TO_MELEE_MONSTERS.contains(&self.info.id.unwrap_or(0))
            || thrall.attack_type() == AttackType::Ranged
                && IMMUNE_TO_RANGED_MONSTERS.contains(&self.info.id.unwrap_or(0))
            || thrall.attack_type() == AttackType::Magic
                && IMMUNE_TO_MAGIC_MONSTERS.contains(&self.info.id.unwrap_or(0))
        {
            return true;
        }

        false
    }

    pub fn matches_version(&self, version: &str) -> bool {
        self.info
            .version
            .as_ref()
            .is_some_and(|v| v.contains(version))
    }

    pub fn add_burn_stack(&mut self, burn_ticks: u32) {
        // Add one burn effect stack, up to a maximum of 5 concurrent stacks
        match self
            .active_effects
            .iter_mut()
            .find(|effect| matches!(effect, CombatEffect::Burn { .. }))
        {
            Some(CombatEffect::Burn { stacks, .. }) => {
                if stacks.len() < 5 {
                    // New stacks are ignored when there are already 5 stacks
                    stacks.push(burn_ticks); // 10 hits per stack
                }
            }
            _ => self.active_effects.push(CombatEffect::Burn {
                tick_counter: Some(0),
                stacks: vec![burn_ticks],
            }),
        }
    }

    pub fn clear_inactive_effects(&mut self) {
        // Clear all combat effects that are no longer active
        self.active_effects.retain(|event| match event {
            CombatEffect::Poison { tick_counter, .. } => tick_counter.is_some(),
            CombatEffect::Venom { tick_counter, .. } => tick_counter.is_some(),
            CombatEffect::Burn { tick_counter, .. } => tick_counter.is_some(),
            CombatEffect::DelayedAttack { tick_delay, .. } => tick_delay.is_some(),
            CombatEffect::DelayedHeal { tick_counter, .. } => tick_counter.is_some(),
            CombatEffect::DamageOverTime { tick_counter, .. } => tick_counter.is_some(),
        })
    }

    pub fn freeze(&mut self, duration: u32) {
        // Freeze the monster for the specified duration, reduced by freeze resistance
        self.info.freeze_duration = duration - duration * self.immunities.freeze / 100;
    }

    pub fn set_toa_level(&mut self, level: u32, path: u32) {
        self.info.toa_level = level;
        self.info.toa_path_level = path;
        self.scale_toa();
    }

    fn set_defence_floor(&mut self) {
        let floor = match self.info.name.as_str() {
            "Verzik Vitur" => self.stats.defence.base,
            "Soteseg" => 100,
            "The Nightmare" | "Phosani's Nightmare" => 120,
            "Akkha" => 70,
            "Ba-Ba" => 60,
            "Kephri" => 60,
            "Zebak" => 50,
            "Tumeken's Warden" | "Elidinis' Warden" => {
                match self
                    .info
                    .version
                    .as_ref()
                    .expect("No version found")
                    .as_str()
                {
                    "Active" => self.stats.defence.base,
                    "Damaged" | "Enraged" => 120,
                    _ => 0,
                }
            }
            "Obelisk (Tombs of Amascut)" => 60,
            "Nex" => 250,
            "Araxxor" => 90,
            "The Hueycoatl" => 120,
            "Yama" => 145,
            _ => 0,
        };

        self.stats.defence.min_cap = floor;
    }
}

fn round_toa_hp(hp: u32) -> u32 {
    if hp < 100 {
        // Unrounded if scaled HP is below 100 HP
        hp
    } else if hp < 300 {
        // Scaled hp between 100 and 300 HP is rounded to nearest multiple of 5
        (hp + 2) / 5 * 5
    } else {
        // Scaled hp above 300 HP is rounded to nearest multiple of 10
        (hp + 5) / 10 * 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_info() {
        let vorkath = Monster::new("Vorkath", Some("Post-quest")).expect("Error creating monster.");
        assert_eq!(vorkath.info.name, "Vorkath".to_string());
        assert_eq!(vorkath.info.combat_level, 732)
    }

    #[test]
    fn test_toa_scaling() {
        let mut baba = Monster::new("Ba-Ba", None).expect("Error creating monster.");
        baba.set_toa_level(400, 0);
        assert_eq!(baba.stats.hitpoints.current, 990);
        assert_eq!(baba.def_rolls.get(CombatType::Stab), 33321);
    }

    #[test]
    fn test_is_dragon() {
        let vorkath = Monster::new("Vorkath", Some("Post-quest")).expect("Error creating monster.");
        assert!(vorkath.is_dragon());
    }

    #[test]
    fn test_is_demon() {
        let kril = Monster::new("K'ril Tsutsaroth", None).expect("Error creating monster.");
        assert!(kril.is_demon());
    }

    #[test]
    fn test_is_undead() {
        let vorkath = Monster::new("Vorkath", Some("Post-quest")).expect("Error creating monster.");
        assert!(vorkath.is_undead());
    }

    #[test]
    fn test_is_in_wilderness() {
        let spindel = Monster::new("Spindel", None).expect("Error creating monster.");
        assert!(spindel.is_in_wilderness());
    }

    #[test]
    fn test_is_revenant() {
        let revenant = Monster::new("Revenant demon", None).expect("Error creating monster.");
        assert!(revenant.is_revenant());
    }

    #[test]
    fn test_tbow_olm() {
        let olm =
            Monster::new("Great Olm", Some("Head (Normal)")).expect("Error creating monster.");
        let (tbow_acc_bonus, tbow_dmg_bonus) = olm.tbow_bonuses();
        assert_eq!(tbow_acc_bonus, 140);
        assert_eq!(tbow_dmg_bonus, 215);
    }

    #[test]
    fn test_max_hit_parsing_one_style() {
        let sergeant_steelwill =
            Monster::new("Sergeant Steelwill", None).expect("Error creating monster.");
        assert!(sergeant_steelwill.max_hits.is_some());
        assert_eq!(sergeant_steelwill.max_hits.as_ref().unwrap().len(), 1);
        assert_eq!(sergeant_steelwill.max_hits.as_ref().unwrap()[0].value, 15);
        assert_eq!(
            sergeant_steelwill.max_hits.unwrap()[0].style,
            AttackType::Magic
        );
    }

    #[test]
    fn test_max_hit_parsing_two_styles() {
        let graardor = Monster::new("General Graardor", None).expect("Error creating monster.");
        assert!(graardor.max_hits.is_some());
        assert_eq!(graardor.max_hits.as_ref().unwrap().len(), 2);
        assert_eq!(graardor.max_hits.as_ref().unwrap()[0].value, 60);
        assert_eq!(
            graardor.max_hits.as_ref().unwrap()[0].style,
            AttackType::Crush
        );
        assert_eq!(graardor.max_hits.as_ref().unwrap()[1].value, 35);
        assert_eq!(graardor.max_hits.unwrap()[1].style, AttackType::Ranged);
    }

    #[test]
    fn test_drain_stat() {
        let mut zebak = Monster::new("Zebak", Some("Normal")).expect("Error creating monster.");
        zebak.stats.defence.drain(20);
        assert_eq!(zebak.stats.defence.current, 50);
    }

    #[test]
    fn test_drain_stat_min_cap() {
        let mut zebak = Monster::new("Zebak", Some("Normal")).expect("Error creating monster.");
        zebak.stats.defence.drain(30);
        assert_eq!(zebak.stats.defence.current, 50);
    }

    #[test]
    fn test_toa_scaling_with_drain() {
        let mut zebak = Monster::new("Zebak", Some("Normal")).expect("Error creating monster.");
        let initial_def_roll = zebak.def_rolls.get(CombatType::Standard);
        zebak.drain_stat(CombatStat::Defence, 20, None);
        assert_ne!(zebak.def_rolls.get(CombatType::Ranged), initial_def_roll);
        let drained_roll = zebak.def_rolls.get(CombatType::Standard);
        zebak.info.toa_level = 400;
        zebak.scale_toa();
        assert_ne!(drained_roll, zebak.def_rolls.get(CombatType::Standard));
    }

    #[test]
    fn test_burn_immunity() {
        let cerberus = Monster::new("Cerberus", None).expect("Error creating monster.");
        assert_eq!(cerberus.immunities.burn.unwrap(), BurnType::Normal);
    }
}
