use lazy_static::lazy_static;

use crate::constants::*;
use crate::equipment::{CombatType, StyleBonus};
use rusqlite::{Connection, Result, Row};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use strum::IntoEnumIterator;

lazy_static! {
    static ref MONSTER_DB: PathBuf = {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        current_dir.join("src/databases/monsters.db")
    };
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

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct MonsterStats {
    pub hitpoints: u16,
    pub attack: u16,
    pub strength: u16,
    pub defence: u16,
    pub ranged: u16,
    pub magic: u16,
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
    pub flat_armour: i8,
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
    pub version: String,
    pub combat_level: u16,
    pub size: u8,
    pub xpbonus: f32,
    pub slayer_xp: u32,
    pub attributes: Vec<Attribute>,
    pub attack_speed: u8,
    pub aggressive: bool,
    pub poisonous: bool,
    pub poison_severity: u8,
    pub toa_level: u16,
    pub toa_path_level: u16,
}

#[derive(Debug, PartialEq)]
pub struct Monster {
    pub info: MonsterInfo,
    pub stats: MonsterStats,
    pub live_stats: MonsterStats,
    pub bonuses: MonsterBonuses,
    pub immunities: Immunities,
    pub def_rolls: HashMap<CombatType, i32>,
    pub base_def_rolls: HashMap<CombatType, i32>,
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
    pub fn new(name: &str) -> Self {
        let mut monster = Self::default();
        monster.info.name = name.to_string();
        monster
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
        self.info.name = row.get::<_, String>("name")?;
        self.info.version = row.get::<_, String>("version")?;
        self.info.combat_level = row.get::<_, u16>("combat")?;
        self.info.size = row.get::<_, u8>("size")?;
        self.info.xpbonus = row.get::<_, f32>("xpbonus")?;
        self.info.slayer_xp = row.get::<_, u32>("slayerxp")?;
        self.info.attributes =
            parse_attributes(row.get::<_, String>("attributes")?.split(',').collect());
        self.info.attack_speed = row.get::<_, u8>("speed")?;
        self.info.aggressive = row.get::<_, bool>("aggressive")?;
        self.info.poisonous = row.get::<_, bool>("poisonous")?;
        self.info.poison_severity = row.get::<_, u8>("poison")?;
        self.stats.attack = row.get::<_, u16>("attack")?;
        self.stats.strength = row.get::<_, u16>("strength")?;
        self.stats.defence = row.get::<_, u16>("defence")?;
        self.stats.ranged = row.get::<_, u16>("ranged")?;
        self.stats.magic = row.get::<_, u16>("magic")?;
        self.stats.hitpoints = row.get::<_, u16>("hitpoints")?;
        self.live_stats = self.stats.clone();
        self.bonuses.attack.melee = row.get::<_, i32>("attbns")?;
        self.bonuses.attack.ranged = row.get::<_, i32>("arange")?;
        self.bonuses.attack.magic = row.get::<_, i32>("amagic")?;
        self.bonuses.strength.melee = row.get::<_, i32>("strbns")?;
        self.bonuses.strength.ranged = row.get::<_, i32>("rngbns")?;
        self.bonuses.strength.magic = row.get::<_, i32>("mbns")?;
        self.bonuses.defence.stab = row.get::<_, i32>("dstab")?;
        self.bonuses.defence.slash = row.get::<_, i32>("dslash")?;
        self.bonuses.defence.crush = row.get::<_, i32>("dcrush")?;
        self.bonuses.defence.ranged = row.get::<_, i32>("drange")?;
        self.bonuses.defence.magic = row.get::<_, i32>("dmagic")?;
        self.bonuses.flat_armour = row.get::<_, i8>("armour")?;
        self.immunities.poison = row.get::<_, bool>("immunepoison")?;
        self.immunities.venom = row.get::<_, bool>("immunevenom")?;
        self.immunities.cannon = row.get::<_, bool>("immunecannon")?;
        self.immunities.thrall = row.get::<_, bool>("immunethrall")?;
        self.immunities.freeze = row.get::<_, u8>("freezeresistance")?;
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
        let toa_level_bonus = 1000 + self.info.toa_level as i32 * 4;
        for defence_type in CombatType::iter() {
            self.def_rolls.insert(
                defence_type,
                self.base_def_rolls[&defence_type] * toa_level_bonus / 1000,
            );
        }
    }
}

fn round_toa_hp(hp: u16) -> u16 {
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
