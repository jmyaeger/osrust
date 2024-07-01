use lazy_static::lazy_static;

use crate::effects::CombatEffect;
use crate::equipment::CombatType;
use crate::monster_db::ElementalWeakness;
use crate::player::Player;
use crate::{constants::*, rolls};
use rusqlite::{params, Result};
use serde::Deserialize;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

lazy_static! {
    static ref MONSTER_DB: PathBuf =
        fs::canonicalize("src/databases/monsters.db").expect("Failed to get database path");
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
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Attribute {
    Demon,
    Draconic,
    Fiery,
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

// Base stats of a monster
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize)]
pub struct MonsterStats {
    pub hitpoints: u32,
    pub attack: u32,
    pub strength: u32,
    pub defence: u32,
    pub ranged: u32,
    pub magic: u32,
}

impl Default for MonsterStats {
    fn default() -> Self {
        Self {
            hitpoints: 10,
            attack: 1,
            strength: 1,
            defence: 1,
            ranged: 1,
            magic: 1,
        }
    }
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
}

type MonsterStrengthBonus = AttackBonus; // Uses the same fields as AttackBonus

// All offensive and defensive bonuses for a monster
#[derive(Debug, PartialEq, Default, Clone, Deserialize)]
pub struct MonsterBonuses {
    pub attack: AttackBonus,
    pub strength: MonsterStrengthBonus,
    pub defence: MonsterDefBonuses,
    #[serde(default)]
    pub flat_armour: u32, // Defaults to 0 during deserialization
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
    #[serde(default)]
    pub melee: bool,
    #[serde(default)]
    pub ranged: bool,
    #[serde(default)]
    pub magic: bool,
}

// Maximum hit value for a given style
#[derive(Debug, Eq, PartialEq, Hash, Default, Clone)]
pub struct MonsterMaxHit {
    pub value: u32,
    pub style: String, // Will probably make this an enum later
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
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_max_hits")]
    pub max_hit: Option<Vec<MonsterMaxHit>>,
    pub attack_styles: Option<Vec<String>>,
    pub weakness: Option<ElementalWeakness>,
    pub attack_speed: u32,
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
    let parsed_attributes = attributes.map(|attrs| {
        attrs
            .into_iter()
            .map(|attr| match attr.as_str() {
                "demon" => Attribute::Demon,
                "dragon" => Attribute::Draconic,
                "fiery" => Attribute::Fiery,
                "golem" => Attribute::Golem,
                "icy" => Attribute::Icy,
                "kalphite" => Attribute::Kalphite,
                "leafy" => Attribute::Leafy,
                "penance" => Attribute::Penance,
                "rat" => Attribute::Rat,
                "shade" => Attribute::Shade,
                "spectral" => Attribute::Spectral,
                "undead" => Attribute::Undead,
                "vampyre1" => Attribute::Vampyre(1),
                "vampyre2" => Attribute::Vampyre(2),
                "vampyre3" => Attribute::Vampyre(3),
                "xerician" => Attribute::Xerician,
                _ => panic!("Unknown attribute: {}", attr),
            })
            .collect()
    });
    Ok(parsed_attributes)
}

fn deserialize_max_hits<'de, D>(deserializer: D) -> Result<Option<Vec<MonsterMaxHit>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // Parse max hit strings into values and styles
    let max_hits: Option<Vec<String>> = Option::deserialize(deserializer)?;
    let parsed_max_hits = max_hits.map(|hits| {
        hits.into_iter()
            .map(|hit| {
                let mut parts = hit.split('(');
                let value = parts.next().unwrap().parse().unwrap_or_default();
                let style = parts.next().unwrap_or_default().to_string();
                MonsterMaxHit { value, style }
            })
            .collect()
    });
    Ok(parsed_max_hits)
}

// Overall monster struct
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Monster {
    pub info: MonsterInfo,
    pub stats: MonsterStats,
    #[serde(default)]
    pub live_stats: MonsterStats,
    pub bonuses: MonsterBonuses,
    pub immunities: Immunities,
    #[serde(skip)]
    pub def_rolls: HashMap<CombatType, i32>,
    #[serde(skip)]
    pub base_def_rolls: HashMap<CombatType, i32>,
    #[serde(skip)]
    pub active_effects: Vec<CombatEffect>, // Will move poison/venom here
}

impl Default for Monster {
    fn default() -> Self {
        // Default to zero for all combat rolls
        let mut def_rolls = HashMap::new();
        def_rolls.insert(CombatType::Stab, 0);
        def_rolls.insert(CombatType::Slash, 0);
        def_rolls.insert(CombatType::Crush, 0);
        def_rolls.insert(CombatType::Light, 0);
        def_rolls.insert(CombatType::Standard, 0);
        def_rolls.insert(CombatType::Heavy, 0);
        def_rolls.insert(CombatType::Magic, 0);

        Self {
            info: MonsterInfo::default(),
            stats: MonsterStats::default(),
            live_stats: MonsterStats::default(),
            bonuses: MonsterBonuses::default(),
            immunities: Immunities::default(),
            def_rolls: def_rolls.clone(),
            base_def_rolls: def_rolls.clone(),
            active_effects: Vec::new(),
        }
    }
}

impl Monster {
    pub fn new(name: &str, version: Option<&str>) -> Result<Monster, Box<dyn std::error::Error>> {
        // Create a monster by name and version (optional) via a SQL query and deserialization

        // Get the monster's data in JSON form from the SQLite database
        let conn = rusqlite::Connection::open(MONSTER_DB.as_path())?;
        let row: String = if version.is_some() {
            conn.query_row(
                "SELECT data FROM monsters WHERE name = ?1 AND version = ?2",
                params![name, version.unwrap()],
                |row| row.get(0),
            )?
        } else {
            conn.query_row(
                "SELECT data FROM monsters WHERE name = ?",
                params![name],
                |row| row.get(0),
            )?
        };

        // Deserialize the JSON data into a Monster struct
        let mut monster: Monster = serde_json::from_str(row.as_str())?;

        // Initialize the live stats to the base stats
        monster.live_stats = monster.stats;

        // Calculate base defence rolls and copy to live defence rolls
        monster.base_def_rolls = rolls::monster_def_rolls(&monster);
        monster.def_rolls.clone_from(&monster.base_def_rolls);

        // Set the flat armour bonus if applicable
        monster.bonuses.flat_armour = monster.info.id.map_or(0, |id| {
            FLAT_ARMOUR.iter().find(|x| x.0 == id).unwrap_or(&(0, 0)).1 as u32
        });

        Ok(monster)
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
        let level_scaled_hp = self.stats.hitpoints * toa_level_bonus / 100;

        // If the NPC is affected by path scaling, apply it
        self.stats.hitpoints = if TOA_PATH_MONSTERS.contains(&self.info.id.unwrap_or(0)) {
            let path_scaled_hp = level_scaled_hp * toa_path_level_bonus / 100;
            round_toa_hp(path_scaled_hp)
        } else {
            round_toa_hp(level_scaled_hp)
        };

        // Set the live HP to the scaled base HP
        self.live_stats.hitpoints = self.stats.hitpoints;
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
            self.def_rolls.insert(
                defence_type,
                self.base_def_rolls[&defence_type] * toa_level_bonus as i32 / 1000,
            );
        }
    }

    pub fn tbow_bonuses(&self) -> (i32, i32) {
        // Calculate twisted bow attack and damage multipliers
        let magic_limit = if self
            .info
            .attributes
            .as_ref()
            .map_or(false, |attrs| attrs.contains(&Attribute::Xerician))
        {
            350 // Inside CoX
        } else {
            250 // Outside CoX
        };

        // Take the higher of the magic level and magic attack bonus, capped at the limit
        let highest_magic = min(
            magic_limit,
            max(self.stats.magic as i32, self.bonuses.attack.magic),
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

    pub fn is_dragon(&self) -> bool {
        self.info
            .attributes
            .as_ref()
            .map_or(false, |attrs| attrs.contains(&Attribute::Draconic))
    }

    pub fn is_demon(&self) -> bool {
        self.info
            .attributes
            .as_ref()
            .map_or(false, |attrs| attrs.contains(&Attribute::Demon))
    }

    pub fn is_undead(&self) -> bool {
        self.info
            .attributes
            .as_ref()
            .map_or(false, |attrs| attrs.contains(&Attribute::Undead))
    }

    pub fn is_kalphite(&self) -> bool {
        self.info
            .attributes
            .as_ref()
            .map_or(false, |attrs| attrs.contains(&Attribute::Kalphite))
    }

    pub fn is_leafy(&self) -> bool {
        self.info
            .attributes
            .as_ref()
            .map_or(false, |attrs| attrs.contains(&Attribute::Leafy))
    }

    pub fn is_golem(&self) -> bool {
        self.info
            .attributes
            .as_ref()
            .map_or(false, |attrs| attrs.contains(&Attribute::Golem))
    }

    pub fn is_rat(&self) -> bool {
        self.info
            .attributes
            .as_ref()
            .map_or(false, |attrs| attrs.contains(&Attribute::Rat))
    }

    pub fn is_fiery(&self) -> bool {
        self.info
            .attributes
            .as_ref()
            .map_or(false, |attrs| attrs.contains(&Attribute::Fiery))
    }

    pub fn is_shade(&self) -> bool {
        self.info
            .attributes
            .as_ref()
            .map_or(false, |attrs| attrs.contains(&Attribute::Shade))
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
        self.live_stats.hitpoints = min(self.live_stats.hitpoints + amount, self.stats.hitpoints);
    }

    pub fn take_damage(&mut self, amount: u32) {
        self.live_stats.hitpoints = self.live_stats.hitpoints.saturating_sub(amount);
    }

    pub fn is_freezable(&self) -> bool {
        self.immunities.freeze != 100 && !self.info.freeze_duration == 0
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
                if self.live_stats.attack - amount < 1 {
                    remainder = amount - self.live_stats.attack + 1;
                    self.live_stats.attack = 1;
                } else {
                    self.live_stats.attack -= amount;
                }
            }
            CombatStat::Strength => {
                if self.live_stats.strength - amount < 1 {
                    remainder = amount - self.live_stats.strength + 1;
                    self.live_stats.strength = 1;
                } else {
                    self.live_stats.strength -= amount;
                }
            }
            CombatStat::Magic => {
                if self.live_stats.magic - amount < 1 {
                    remainder = amount - self.live_stats.magic + 1;
                    self.live_stats.magic = 1;
                } else {
                    self.live_stats.magic -= amount;
                }
                rolls::monster_def_rolls(self);
            }
            CombatStat::Ranged => {
                if self.live_stats.ranged - amount < 1 {
                    remainder = amount - self.live_stats.ranged + 1;
                    self.live_stats.ranged = 1;
                } else {
                    self.live_stats.ranged -= amount;
                }
            }
            CombatStat::Defence => {
                if self.live_stats.defence - amount < 1 {
                    remainder = amount - self.live_stats.defence + 1;
                    self.live_stats.defence = 1;
                } else {
                    self.live_stats.defence -= amount;
                }
                rolls::monster_def_rolls(self);
            }
        }

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
        self.live_stats = self.stats;
        self.info.poison_severity = 0;
        self.info.freeze_duration = 0;
        rolls::monster_def_rolls(self);
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
                && IMMUNE_TO_MELEE_MONSTERS.contains(&self.info.id.unwrap_or(0)))
        {
            return true;
        }

        if self.vampyre_tier() == Some(3) && !player.is_wearing_ivandis_weapon() {
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
                        .map_or(false, |ammo| ammo.name.contains("arrow"))))
        {
            return true;
        }

        if !self.info.name.contains("Verzik") && player.is_wearing("Dawnbringer", None) {
            // Dawnbringer is only usable inside the Verzik room (should check if usable on crabs)
            return true;
        }

        false
    }

    pub fn matches_version(&self, version: &str) -> bool {
        self.info
            .version
            .as_ref()
            .map_or(false, |v| v.contains(version))
    }

    pub fn add_burn_stack(&mut self) {
        // Add one burn effect stack, up to a maximum of 5 concurrent stacks
        match self
            .active_effects
            .iter_mut()
            .find(|effect| matches!(effect, CombatEffect::Burn { .. }))
        {
            Some(CombatEffect::Burn { stacks, .. }) => {
                if stacks.len() < 5 {
                    stacks.push(10); // 10 hits per stack
                }
            }
            _ => self.active_effects.push(CombatEffect::Burn {
                tick_counter: Some(0),
                stacks: vec![10],
            }),
        }
    }

    pub fn clear_inactive_effects(&mut self) {
        self.active_effects.retain(|event| match event {
            CombatEffect::Poison { tick_counter, .. } => tick_counter.is_some(),
            CombatEffect::Venom { tick_counter, .. } => tick_counter.is_some(),
            CombatEffect::Burn { tick_counter, .. } => tick_counter.is_some(),
            CombatEffect::DelayedAttack { tick_delay, .. } => tick_delay.is_some(),
            CombatEffect::DelayedHeal { tick_delay, .. } => tick_delay.is_some(),
        })
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
        let vorkath = Monster::new("Vorkath", Some("Post-quest")).unwrap();
        assert_eq!(vorkath.info.name, "Vorkath".to_string());
        assert_eq!(vorkath.info.combat_level, 732)
    }

    #[test]
    fn test_toa_scaling() {
        let mut baba = Monster::new("Ba-Ba", None).unwrap();
        baba.info.toa_level = 400;
        baba.scale_toa();
        assert_eq!(baba.stats.hitpoints, 990);
        assert_eq!(baba.def_rolls[&CombatType::Stab], 33321);
    }

    #[test]
    fn test_is_dragon() {
        let vorkath = Monster::new("Vorkath", Some("Post-quest")).unwrap();
        assert!(vorkath.is_dragon());
    }

    #[test]
    fn test_is_demon() {
        let kril = Monster::new("K'ril Tsutsaroth", None).unwrap();
        assert!(kril.is_demon());
    }

    #[test]
    fn test_is_undead() {
        let vorkath = Monster::new("Vorkath", Some("Post-quest")).unwrap();
        assert!(vorkath.is_undead());
    }

    #[test]
    fn test_is_in_wilderness() {
        let spindel = Monster::new("Spindel", None).unwrap();
        assert!(spindel.is_in_wilderness());
    }

    #[test]
    fn test_is_revenant() {
        let revenant = Monster::new("Revenant demon", None).unwrap();
        assert!(revenant.is_revenant());
    }

    #[test]
    fn test_tbow_olm() {
        let olm = Monster::new("Great Olm", Some("Head")).unwrap();
        let (tbow_acc_bonus, tbow_dmg_bonus) = olm.tbow_bonuses();
        assert_eq!(tbow_acc_bonus, 140);
        assert_eq!(tbow_dmg_bonus, 215);
    }
}
