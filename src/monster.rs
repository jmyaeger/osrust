use lazy_static::lazy_static;

use crate::equipment::{CombatType, StyleBonus};
use crate::rolls::monster_def_rolls;
use crate::{constants::*, rolls};
use rusqlite::{Connection, Result, Row};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use strum::IntoEnumIterator;

lazy_static! {
    static ref MONSTER_DB: PathBuf =
        fs::canonicalize("src/databases/monsters.db").expect("Failed to get database path");
}

#[derive(Debug, Eq, PartialEq, Hash)]
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

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
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

#[derive(Debug, Eq, PartialEq, Hash, Default)]
pub struct AttackBonus {
    pub melee: i32,
    pub ranged: i32,
    pub magic: i32,
}

type MonsterStrengthBonus = AttackBonus;

#[derive(Debug, PartialEq, Default)]
pub struct MonsterBonuses {
    pub attack: AttackBonus,
    pub strength: MonsterStrengthBonus,
    pub defence: StyleBonus,
    pub flat_armour: u32,
}

#[derive(Debug, Eq, PartialEq, Hash, Default)]
pub struct Immunities {
    pub poison: bool,
    pub venom: bool,
    pub cannon: bool,
    pub thrall: bool,
    pub freeze: u8,
    pub melee: bool,
    pub ranged: bool,
    pub magic: bool,
}

#[derive(Debug, PartialEq, Default)]
pub struct MonsterInfo {
    pub name: String,
    pub version: Option<String>,
    pub combat_level: u16,
    pub size: u8,
    pub xpbonus: f32,
    pub slayer_xp: f32,
    pub attributes: Option<Vec<Attribute>>,
    pub attack_speed: u8,
    pub aggressive: bool,
    pub poisonous: bool,
    pub poison_severity: u8,
    pub freeze_duration: u8,
    pub toa_level: u32,
    pub toa_path_level: u32,
}

#[derive(Debug, PartialEq)]
pub struct Monster {
    pub info: MonsterInfo,
    pub stats: MonsterStats,
    pub live_stats: MonsterStats,
    pub bonuses: MonsterBonuses,
    pub immunities: Immunities,
    pub def_rolls: HashMap<CombatType, u32>,
    pub base_def_rolls: HashMap<CombatType, u32>,
}

impl Default for Monster {
    fn default() -> Self {
        let mut def_rolls = HashMap::new();
        def_rolls.insert(CombatType::Stab, 0);
        def_rolls.insert(CombatType::Slash, 0);
        def_rolls.insert(CombatType::Crush, 0);
        def_rolls.insert(CombatType::Ranged, 0);
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
    pub fn new(name: &str) -> Result<Self> {
        let mut monster = Self::default();
        monster.set_info(name)?;
        Ok(monster)
    }

    pub fn set_info(&mut self, name: &str) -> Result<()> {
        let conn = Connection::open(MONSTER_DB.as_path())?;
        let mut stmt = conn.prepare("SELECT * FROM monsters WHERE name = ?")?;
        let mut rows = stmt.query([&name])?;
        if let Some(row) = rows.next()? {
            self.set_fields_from_row(row)?;
            Ok(())
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }

    pub fn set_fields_from_row(&mut self, row: &Row) -> Result<()> {
        self.info.name = row.get::<_, Option<String>>("name")?.unwrap_or_default();
        self.info.version = row.get::<_, Option<String>>("version")?;
        self.info.combat_level = row.get::<_, Option<u16>>("combat")?.unwrap_or_default();
        self.info.size = row.get::<_, Option<u8>>("size")?.unwrap_or_default();
        self.info.xpbonus = row.get::<_, Option<f32>>("xpbonus")?.unwrap_or_default();
        self.info.slayer_xp = row.get::<_, Option<f32>>("slayxp")?.unwrap_or_default();
        let attributes = row.get::<_, Option<String>>("attributes")?;
        if let Some(attributes) = attributes {
            self.info.attributes = Some(parse_attributes(attributes.split(',').collect()));
        }
        self.info.attack_speed = row
            .get::<_, Option<u8>>("attack_speed")?
            .unwrap_or_default();
        self.info.aggressive = row
            .get::<_, Option<bool>>("aggressive")?
            .unwrap_or_default();
        self.info.poisonous = row.get::<_, Option<bool>>("poisonous")?.unwrap_or_default();
        self.stats.attack = row.get::<_, Option<u32>>("attack")?.unwrap_or_default();
        self.stats.strength = row.get::<_, Option<u32>>("strength")?.unwrap_or_default();
        self.stats.defence = row.get::<_, Option<u32>>("defence")?.unwrap_or_default();
        self.stats.ranged = row.get::<_, Option<u32>>("ranged")?.unwrap_or_default();
        self.stats.magic = row.get::<_, Option<u32>>("magic")?.unwrap_or_default();
        self.stats.hitpoints = row.get::<_, Option<u32>>("hitpoints")?.unwrap_or_default();
        self.live_stats = self.stats;
        self.bonuses.attack.melee = row.get::<_, Option<i32>>("attbns")?.unwrap_or_default();
        self.bonuses.attack.ranged = row.get::<_, Option<i32>>("arange")?.unwrap_or_default();
        self.bonuses.attack.magic = row.get::<_, Option<i32>>("amagic")?.unwrap_or_default();
        self.bonuses.strength.melee = row.get::<_, Option<i32>>("strbns")?.unwrap_or_default();
        self.bonuses.strength.ranged = row.get::<_, Option<i32>>("rngbns")?.unwrap_or_default();
        self.bonuses.strength.magic = row.get::<_, Option<i32>>("mbns")?.unwrap_or_default();
        self.bonuses.defence.stab = row.get::<_, Option<i32>>("dstab")?.unwrap_or_default();
        self.bonuses.defence.slash = row.get::<_, Option<i32>>("dslash")?.unwrap_or_default();
        self.bonuses.defence.crush = row.get::<_, Option<i32>>("dcrush")?.unwrap_or_default();
        self.bonuses.defence.ranged = row.get::<_, Option<i32>>("drange")?.unwrap_or_default();
        self.bonuses.defence.magic = row.get::<_, Option<i32>>("dmagic")?.unwrap_or_default();
        self.bonuses.flat_armour = row.get::<_, Option<u32>>("armour")?.unwrap_or_default();
        self.immunities.poison = row
            .get::<_, Option<bool>>("immunepoison")?
            .unwrap_or_default();
        self.immunities.venom = row
            .get::<_, Option<bool>>("immunevenom")?
            .unwrap_or_default();
        self.immunities.cannon = row
            .get::<_, Option<bool>>("immunecannon")?
            .unwrap_or_default();
        self.immunities.thrall = row
            .get::<_, Option<bool>>("immunethrall")?
            .unwrap_or_default();
        self.immunities.freeze = row
            .get::<_, Option<u8>>("freezeresistance")?
            .unwrap_or_default();

        self.base_def_rolls = monster_def_rolls(self);
        self.def_rolls = self.base_def_rolls.clone();

        Ok(())
    }

    pub fn scale_toa(&mut self) {
        if TOA_MONSTERS.contains(&self.info.name.as_str()) {
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
        let toa_level_bonus = 1000 + self.info.toa_level * level_factor;
        let toa_path_level_bonus = if self.info.toa_path_level == 0 {
            1000
        } else {
            1080 + (self.info.toa_path_level - 1) * 50
        };
        let level_scaled_hp = self.stats.hitpoints * toa_level_bonus / 1000;

        self.stats.hitpoints = if TOA_PATH_MONSTERS.contains(&self.info.name.as_str()) {
            let path_scaled_hp = level_scaled_hp * toa_path_level_bonus / 1000;
            round_toa_hp(path_scaled_hp)
        } else {
            round_toa_hp(level_scaled_hp)
        };
        self.live_stats.hitpoints = self.stats.hitpoints;
    }

    fn scale_toa_defence(&mut self) {
        let toa_level_bonus = 1000 + self.info.toa_level * 4;
        for defence_type in CombatType::iter() {
            if defence_type == CombatType::None {
                continue;
            }
            self.def_rolls.insert(
                defence_type,
                self.base_def_rolls[&defence_type] * toa_level_bonus / 1000,
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
        TOA_MONSTERS.contains(&self.info.name.as_str())
    }

    pub fn is_toa_path_monster(&self) -> bool {
        TOA_PATH_MONSTERS.contains(&self.info.name.as_str())
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

fn parse_attributes(attributes: Vec<&str>) -> Vec<Attribute> {
    let mut attr_vec = Vec::new();
    for attribute in &attributes {
        match attribute.to_lowercase().as_str() {
            "demon" => attr_vec.push(Attribute::Demon),
            "dragon" | "draconic" => attr_vec.push(Attribute::Draconic),
            "fiery" => attr_vec.push(Attribute::Fiery),
            "golem" => attr_vec.push(Attribute::Golem),
            "icy" => attr_vec.push(Attribute::Icy),
            "kalphite" => attr_vec.push(Attribute::Kalphite),
            "leafy" => attr_vec.push(Attribute::Leafy),
            "penance" => attr_vec.push(Attribute::Penance),
            "rat" => attr_vec.push(Attribute::Rat),
            "shade" => attr_vec.push(Attribute::Shade),
            "spectral" => attr_vec.push(Attribute::Spectral),
            "undead" => attr_vec.push(Attribute::Undead),
            "vampyre1" => attr_vec.push(Attribute::Vampyre(1)),
            "vampyre2" => attr_vec.push(Attribute::Vampyre(2)),
            "vampyre3" => attr_vec.push(Attribute::Vampyre(3)),
            "xerician" => attr_vec.push(Attribute::Xerician),
            _ => {}
        }
    }
    attr_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_info() {
        let vorkath = Monster::new("Vorkath").unwrap();
        assert_eq!(vorkath.info.name, "Vorkath".to_string());
        assert_eq!(vorkath.info.combat_level, 732)
    }

    #[test]
    fn test_toa_scaling() {
        let mut baba = Monster::new("Ba-Ba").unwrap();
        baba.info.toa_level = 400;
        baba.scale_toa();
        assert_eq!(baba.stats.hitpoints, 990);
        assert_eq!(baba.def_rolls[&CombatType::Stab], 33321);
    }

    #[test]
    fn test_is_dragon() {
        let vorkath = Monster::new("Vorkath").unwrap();
        assert!(vorkath.is_dragon());
    }

    #[test]
    fn test_is_demon() {
        let kril = Monster::new("K'ril Tsutsaroth").unwrap();
        assert!(kril.is_demon());
    }

    #[test]
    fn test_is_undead() {
        let vorkath = Monster::new("Vorkath").unwrap();
        assert!(vorkath.is_undead());
    }

    #[test]
    fn test_is_in_wilderness() {
        let spindel = Monster::new("Spindel").unwrap();
        assert!(spindel.is_in_wilderness());
    }

    #[test]
    fn test_is_revenant() {
        let revenant = Monster::new("Revenant demon").unwrap();
        assert!(revenant.is_revenant());
    }

    #[test]
    fn test_tbow_olm() {
        let olm = Monster::new("Great Olm (Head)").unwrap();
        let (tbow_acc_bonus, tbow_dmg_bonus) = olm.tbow_bonuses();
        assert_eq!(tbow_acc_bonus, 140);
        assert_eq!(tbow_dmg_bonus, 215);
    }
}
