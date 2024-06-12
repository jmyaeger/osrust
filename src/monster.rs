use lazy_static::lazy_static;

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
    Vampyre(u8),
    Xerician,
}

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

#[derive(Debug, Eq, PartialEq, Hash, Default, Clone, Deserialize)]
pub struct AttackBonus {
    pub melee: i32,
    pub ranged: i32,
    pub magic: i32,
}

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

type MonsterStrengthBonus = AttackBonus;

#[derive(Debug, PartialEq, Default, Clone, Deserialize)]
pub struct MonsterBonuses {
    pub attack: AttackBonus,
    pub strength: MonsterStrengthBonus,
    pub defence: MonsterDefBonuses,
    #[serde(default)]
    pub flat_armour: u32,
}

#[derive(Debug, Eq, PartialEq, Hash, Default, Clone, Deserialize)]
pub struct Immunities {
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

#[derive(Debug, Eq, PartialEq, Hash, Default, Clone)]
pub struct MonsterMaxHit {
    pub value: u32,
    pub style: String,
}

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
    pub poison_severity: u32,
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
}

impl Default for Monster {
    fn default() -> Self {
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
        }
    }
}

impl Monster {
    pub fn new(name: &str, version: Option<&str>) -> Result<Monster, Box<dyn std::error::Error>> {
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

        let mut monster: Monster = serde_json::from_str(row.as_str())?;

        monster.live_stats = monster.stats;
        monster.base_def_rolls = rolls::monster_def_rolls(&monster);
        monster.def_rolls = monster.base_def_rolls.clone();

        monster.bonuses.flat_armour = monster.info.id.map_or(0, |id| {
            FLAT_ARMOUR.iter().find(|x| x.0 == id).unwrap_or(&(0, 0)).1 as u32
        });

        Ok(monster)
    }

    pub fn scale_toa(&mut self) {
        if TOA_MONSTERS.contains(&self.info.id.unwrap_or(0)) {
            self.scale_toa_hp();
            self.scale_toa_defence();
        }
    }
    fn scale_toa_hp(&mut self) {
        let level_factor = if self.info.name.as_str() == "Core (Wardens)" {
            1
        } else {
            4
        };
        let toa_level_bonus = 100 + (self.info.toa_level * level_factor / 10);
        let toa_path_level_bonus = if self.info.toa_path_level == 0 {
            100
        } else {
            108 + (self.info.toa_path_level - 1) * 5
        };
        let level_scaled_hp = self.stats.hitpoints * toa_level_bonus / 100;

        self.stats.hitpoints = if TOA_PATH_MONSTERS.contains(&self.info.id.unwrap_or(0)) {
            let path_scaled_hp = level_scaled_hp * toa_path_level_bonus / 100;
            round_toa_hp(path_scaled_hp)
        } else {
            round_toa_hp(level_scaled_hp)
        };
        self.live_stats.hitpoints = self.stats.hitpoints;
    }

    fn scale_toa_defence(&mut self) {
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
        let magic_limit = if self
            .info
            .attributes
            .as_ref()
            .map_or(false, |attrs| attrs.contains(&Attribute::Xerician))
        {
            350
        } else {
            250
        };
        let highest_magic = min(
            magic_limit,
            max(self.stats.magic as i32, self.bonuses.attack.magic),
        );
        let tbow_m = highest_magic * 3 / 10;
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

    pub fn drain_defence(&mut self, amount: u32) {
        self.live_stats.defence = self.live_stats.defence.saturating_sub(amount);
        rolls::monster_def_rolls(self);
    }

    pub fn drain_strength(&mut self, amount: u32) {
        self.live_stats.strength = self.live_stats.strength.saturating_sub(amount);
    }

    pub fn drain_magic(&mut self, amount: u32) {
        self.live_stats.magic = self.live_stats.magic.saturating_sub(amount);
        rolls::monster_def_rolls(self);
    }

    pub fn drain_ranged(&mut self, amount: u32) {
        self.live_stats.ranged = self.live_stats.ranged.saturating_sub(amount);
    }

    pub fn drain_attack(&mut self, amount: u32) {
        self.live_stats.attack = self.live_stats.attack.saturating_sub(amount);
    }

    pub fn reset(&mut self) {
        self.live_stats = self.stats;
        self.info.poison_severity = 0;
        self.info.freeze_duration = 0;
        rolls::monster_def_rolls(self);
    }

    pub fn is_immune(&self, player: &Player) -> bool {
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

        false
    }

    pub fn matches_version(&self, version: &str) -> bool {
        self.info
            .version
            .as_ref()
            .map_or(false, |v| v.contains(version))
    }
}

fn round_toa_hp(hp: u32) -> u32 {
    if hp < 100 {
        hp
    } else if hp < 300 {
        (hp + 2) / 5 * 5
    } else {
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
