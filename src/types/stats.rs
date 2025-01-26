use crate::constants::*;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::ops::{Add, AddAssign, Sub, SubAssign};

// Stats of the player (both base stats and current stats)
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Default)]
pub struct PlayerStats {
    pub hitpoints: PlayerStat,
    pub attack: PlayerStat,
    pub strength: PlayerStat,
    pub defence: PlayerStat,
    pub ranged: PlayerStat,
    pub magic: PlayerStat,
    pub prayer: PlayerStat,
    pub mining: PlayerStat,
    pub herblore: PlayerStat,
    pub spec: SpecEnergy,
}

impl PlayerStats {
    pub fn min_stats() -> Self {
        Self {
            hitpoints: PlayerStat::from(MIN_HITPOINTS),
            attack: PlayerStat::min_level(),
            strength: PlayerStat::min_level(),
            defence: PlayerStat::min_level(),
            ranged: PlayerStat::min_level(),
            magic: PlayerStat::min_level(),
            prayer: PlayerStat::min_level(),
            mining: PlayerStat::min_level(),
            herblore: PlayerStat::min_level(),
            spec: SpecEnergy::default(),
        }
    }
}

impl TryFrom<&HashMap<&str, u32>> for PlayerStats {
    type Error = &'static str;

    fn try_from(stats_map: &HashMap<&str, u32>) -> Result<Self, Self::Error> {
        let mut stats = Self::default();
        for stat_name in STAT_NAMES {
            if let Some(&value) = stats_map.get(stat_name) {
                match stat_name {
                    "hitpoints" => stats.hitpoints = PlayerStat::from(value),
                    "attack" => stats.attack = PlayerStat::from(value),
                    "strength" => stats.strength = PlayerStat::from(value),
                    "defence" => stats.defence = PlayerStat::from(value),
                    "ranged" => stats.ranged = PlayerStat::from(value),
                    "magic" => stats.magic = PlayerStat::from(value),
                    "prayer" => stats.prayer = PlayerStat::from(value),
                    "mining" => stats.mining = PlayerStat::from(value),
                    "herblore" => stats.herblore = PlayerStat::from(value),
                    _ => return Err("Invalid stat name"),
                }
            }
        }
        Ok(stats)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpecEnergy(u8);

impl SpecEnergy {
    pub fn new(value: u8) -> Self {
        if value <= FULL_SPEC {
            Self(value)
        } else {
            Self(FULL_SPEC)
        }
    }

    pub fn regen_full(&mut self) {
        self.0 = FULL_SPEC;
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn is_full(&self) -> bool {
        self.0 == FULL_SPEC
    }

    pub fn has_enough(&self, amount: u8) -> bool {
        self.0 >= amount
    }

    pub fn regen(&mut self) {
        self.0 = min(self.0 + SPEC_REGEN, FULL_SPEC);
    }

    pub fn death_charge(&mut self) {
        self.0 = min(self.0 + DEATH_CHARGE, FULL_SPEC);
    }
}

impl Default for SpecEnergy {
    fn default() -> Self {
        Self(FULL_SPEC)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerStat {
    pub base: u32,
    pub current: u32,
}

impl PlayerStat {
    pub fn new(base: u32) -> Self {
        Self {
            base,
            current: base,
        }
    }
    pub fn min_level() -> Self {
        Self::new(MIN_LEVEL)
    }

    pub fn restore(&mut self, amount: u32, overboost: Option<u32>) {
        let level_cap = match overboost {
            Some(over) => self.base + over,
            None => self.base,
        };
        self.current = min(level_cap, self.current + amount);
    }

    pub fn drain(&mut self, amount: u32, min_cap: Option<u32>) {
        let min_level = min_cap.unwrap_or(MIN_LEVEL);
        self.current = max(min_level, self.current.saturating_sub(amount));
    }

    pub fn boost(&mut self, amount: u32) {
        self.current = min(self.current + amount, self.base + amount);
    }
}

impl From<u32> for PlayerStat {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl Default for PlayerStat {
    fn default() -> Self {
        Self::new(MAX_LEVEL)
    }
}
