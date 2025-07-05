use serde::Deserialize;

use crate::constants::*;
use std::cmp::{max, min};
use std::collections::HashMap;

// Stats of the player (both base stats and current stats)
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Default)]
pub struct PlayerStats {
    pub hitpoints: Stat,
    pub attack: Stat,
    pub strength: Stat,
    pub defence: Stat,
    pub ranged: Stat,
    pub magic: Stat,
    pub prayer: Stat,
    pub mining: Stat,
    pub herblore: Stat,
    pub spec: SpecEnergy,
}

impl PlayerStats {
    pub fn min_stats() -> Self {
        Self {
            hitpoints: Stat::from(MIN_HITPOINTS),
            attack: Stat::min_level(),
            strength: Stat::min_level(),
            defence: Stat::min_level(),
            ranged: Stat::min_level(),
            magic: Stat::min_level(),
            prayer: Stat::min_level(),
            mining: Stat::min_level(),
            herblore: Stat::min_level(),
            spec: SpecEnergy::default(),
        }
    }

    pub fn reset_all(&mut self) {
        self.hitpoints.reset();
        self.attack.reset();
        self.strength.reset();
        self.defence.reset();
        self.ranged.reset();
        self.magic.reset();
        self.prayer.reset();
        self.mining.reset();
        self.herblore.reset();
        self.spec.reset();
    }
}

impl TryFrom<&HashMap<&str, u32>> for PlayerStats {
    type Error = &'static str;

    fn try_from(stats_map: &HashMap<&str, u32>) -> Result<Self, Self::Error> {
        let mut stats = Self::default();
        for stat_name in STAT_NAMES {
            if let Some(&value) = stats_map.get(stat_name) {
                match stat_name {
                    "hitpoints" => stats.hitpoints = Stat::from(value),
                    "attack" => stats.attack = Stat::from(value),
                    "strength" => stats.strength = Stat::from(value),
                    "defence" => stats.defence = Stat::from(value),
                    "ranged" => stats.ranged = Stat::from(value),
                    "magic" => stats.magic = Stat::from(value),
                    "prayer" => stats.prayer = Stat::from(value),
                    "mining" => stats.mining = Stat::from(value),
                    "herblore" => stats.herblore = Stat::from(value),
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

    pub fn drain(&mut self, amount: u8) {
        self.0 = max(0, self.0.saturating_sub(amount));
    }

    pub fn reset(&mut self) {
        self.regen_full();
    }
}

impl Default for SpecEnergy {
    fn default() -> Self {
        Self(FULL_SPEC)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Stat {
    pub base: u32,
    pub current: u32,
    pub min_cap: u32,
}

impl Stat {
    pub fn new(base: u32, min_cap: Option<u32>) -> Self {
        Self {
            base,
            current: base,
            min_cap: min_cap.unwrap_or_default(),
        }
    }
    pub fn min_level() -> Self {
        Self::new(MIN_LEVEL, None)
    }

    pub fn restore(&mut self, amount: u32, overboost: Option<u32>) {
        let level_cap = match overboost {
            Some(over) => self.base + over,
            None => self.base,
        };
        self.current = min(level_cap, self.current + amount);
    }

    pub fn drain(&mut self, amount: u32) {
        self.current = max(self.min_cap, self.current.saturating_sub(amount));
    }

    pub fn boost(&mut self, amount: u32) {
        self.current = min(self.current + amount, self.base + amount);
    }

    pub fn reset(&mut self) {
        self.current = self.base;
    }
}

impl From<u32> for Stat {
    fn from(value: u32) -> Self {
        Self::new(value, None)
    }
}

impl Default for Stat {
    fn default() -> Self {
        Self::new(MAX_LEVEL, None)
    }
}

impl<'de> Deserialize<'de> for Stat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Can handle both object form and direct number form
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StatValue {
            Number(u32),
            Object {
                base: u32,
                #[serde(default)]
                current: u32,
            },
        }

        let value = StatValue::deserialize(deserializer)?;
        match value {
            StatValue::Number(base) => Ok(Stat {
                base,
                current: base,
                min_cap: 0,
            }),
            StatValue::Object { base, current } => Ok(Stat {
                base,
                current,
                min_cap: 0,
            }),
        }
    }
}

// Base stats of a monster
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize)]
pub struct MonsterStats {
    pub hitpoints: Stat,
    pub attack: Stat,
    pub strength: Stat,
    pub defence: Stat,
    pub ranged: Stat,
    pub magic: Stat,
}

impl Default for MonsterStats {
    fn default() -> Self {
        Self {
            hitpoints: Stat::new(MIN_HITPOINTS, None),
            attack: Stat::min_level(),
            strength: Stat::min_level(),
            defence: Stat::min_level(),
            ranged: Stat::min_level(),
            magic: Stat::min_level(),
        }
    }
}

impl MonsterStats {
    pub fn reset_all(&mut self) {
        self.hitpoints.reset();
        self.attack.reset();
        self.strength.reset();
        self.defence.reset();
        self.ranged.reset();
        self.magic.reset();
    }
}
