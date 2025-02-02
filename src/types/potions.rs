use crate::types::stats::Stat;

// All types of potions or combat level boosting items
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum Potion {
    #[default]
    None,
    Attack,
    Strength,
    Defence,
    Ranging,
    Magic,
    SuperAttack,
    SuperStrength,
    SuperDefence,
    SuperCombat,
    SuperRanging,
    SuperMagic,
    OverloadMinus,
    Overload,
    OverloadPlus,
    ZamorakBrewAtt,
    ZamorakBrewStr,
    SmellingSalts,
    DragonBattleaxe,
    SaradominBrew,
    AncientBrew,
    ForgottenBrew,
    ImbuedHeart,
    SaturatedHeart,
    RubyHarvest,
    BlackWarlock,
    SapphireGlacialis,
    Moonlight,
}

impl Potion {
    pub fn boosts_attack(&self) -> bool {
        self == &Potion::Attack
            || self == &Potion::SuperAttack
            || self == &Potion::ZamorakBrewAtt
            || self == &Potion::RubyHarvest
    }

    pub fn boosts_strength(&self) -> bool {
        self == &Potion::Strength
            || self == &Potion::SuperStrength
            || self == &Potion::ZamorakBrewStr
            || self == &Potion::DragonBattleaxe
            || self == &Potion::BlackWarlock
    }

    pub fn boosts_defence(&self) -> bool {
        self == &Potion::Defence
            || self == &Potion::SuperDefence
            || self == &Potion::SaradominBrew
            || self == &Potion::SapphireGlacialis
    }

    pub fn boosts_ranged(&self) -> bool {
        self == &Potion::Ranging || self == &Potion::SuperRanging
    }

    pub fn boosts_magic(&self) -> bool {
        self == &Potion::Magic
            || self == &Potion::SuperMagic
            || self == &Potion::ImbuedHeart
            || self == &Potion::SaturatedHeart
            || self == &Potion::AncientBrew
            || self == &Potion::ForgottenBrew
    }

    pub fn boosts_all_melee(&self) -> bool {
        self == &Potion::SuperCombat || self == &Potion::Moonlight
    }

    pub fn boosts_all(&self) -> bool {
        self == &Potion::SmellingSalts
            || self == &Potion::OverloadMinus
            || self == &Potion::Overload
            || self == &Potion::OverloadPlus
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PotionStat {
    Attack,
    Strength,
    Defence,
    Ranging,
    Magic,
}

// Contains the type of potion, the boost calc formula, and the calced boost
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PotionBoost {
    pub potion_type: Potion,
    pub factor: u32,
    pub constant: u32,
    pub boost: u32,
}

impl PotionBoost {
    pub fn new(potion: &Potion) -> Self {
        match potion {
            Potion::Attack | Potion::Strength | Potion::Defence => PotionBoost {
                potion_type: *potion,
                factor: 10,
                constant: 3,
                boost: 0, // All boosts initialized to 0, calculated based on player stats
            },
            Potion::Ranging => PotionBoost {
                potion_type: *potion,
                factor: 10,
                constant: 4,
                boost: 0,
            },
            Potion::Magic => PotionBoost {
                potion_type: *potion,
                factor: 0,
                constant: 4,
                boost: 0,
            },
            Potion::SuperAttack
            | Potion::SuperStrength
            | Potion::SuperDefence
            | Potion::SuperRanging
            | Potion::SuperMagic
            | Potion::SuperCombat => PotionBoost {
                potion_type: *potion,
                factor: 15,
                constant: 5,
                boost: 0,
            },
            Potion::OverloadMinus => PotionBoost {
                potion_type: *potion,
                factor: 10,
                constant: 4,
                boost: 0,
            },
            Potion::Overload => PotionBoost {
                potion_type: *potion,
                factor: 13,
                constant: 5,
                boost: 0,
            },
            Potion::OverloadPlus => PotionBoost {
                potion_type: *potion,
                factor: 16,
                constant: 6,
                boost: 0,
            },
            Potion::ZamorakBrewAtt => PotionBoost {
                potion_type: *potion,
                factor: 20,
                constant: 2,
                boost: 0,
            },
            Potion::ZamorakBrewStr => PotionBoost {
                potion_type: *potion,
                factor: 12,
                constant: 2,
                boost: 0,
            },
            Potion::DragonBattleaxe => PotionBoost {
                potion_type: *potion,
                factor: 0,
                constant: 0,
                boost: 0, // Custom implementation
            },
            Potion::SaradominBrew => PotionBoost {
                potion_type: *potion,
                factor: 20,
                constant: 2,
                boost: 0,
            },
            Potion::AncientBrew => PotionBoost {
                potion_type: *potion,
                factor: 5,
                constant: 2,
                boost: 0,
            },
            Potion::ForgottenBrew => PotionBoost {
                potion_type: *potion,
                factor: 8,
                constant: 3,
                boost: 0,
            },
            Potion::ImbuedHeart => PotionBoost {
                potion_type: *potion,
                factor: 10,
                constant: 1,
                boost: 0,
            },
            Potion::SaturatedHeart => PotionBoost {
                potion_type: *potion,
                factor: 10,
                constant: 4,
                boost: 0,
            },
            Potion::SmellingSalts => PotionBoost {
                potion_type: *potion,
                factor: 16,
                constant: 11,
                boost: 0,
            },
            Potion::RubyHarvest => PotionBoost {
                potion_type: *potion,
                factor: 15,
                constant: 4,
                boost: 0,
            },
            Potion::SapphireGlacialis => PotionBoost {
                potion_type: *potion,
                factor: 15,
                constant: 4,
                boost: 0,
            },
            Potion::BlackWarlock => PotionBoost {
                potion_type: *potion,
                factor: 15,
                constant: 4,
                boost: 0,
            },
            Potion::Moonlight => PotionBoost {
                potion_type: *potion,
                factor: 0,
                constant: 0,
                boost: 0, // Custom implementation
            },
            _ => PotionBoost {
                potion_type: *potion,
                factor: 0,
                constant: 0,
                boost: 0,
            },
        }
    }

    pub fn calc_boost(&mut self, level: Stat) {
        // Calculate the level boost based on the player's base level
        self.boost = self.factor * level.base / 100 + self.constant;
    }

    pub fn calc_dragon_battleaxe_boost(
        &mut self,
        att_level: Stat,
        def_level: Stat,
        ranged_level: Stat,
        magic_level: Stat,
    ) {
        // DBA boost gets its own function
        let stats = [att_level, def_level, ranged_level, magic_level];
        let sum: u32 = stats.iter().map(|&n| n.current / 10).sum();
        self.boost = 10 + (sum / 4);
    }

    pub fn calc_moonlight_boost(
        &mut self,
        combat_level: Stat,
        herblore_level: Stat,
        skill: PotionStat,
    ) {
        match skill {
            PotionStat::Attack => {
                if herblore_level.base >= 45 {
                    self.boost = 5 + combat_level.base * 15 / 100;
                } else if herblore_level.base >= 3 {
                    self.boost = 3 + combat_level.base / 10;
                }
            }
            PotionStat::Strength => {
                if herblore_level.base >= 55 {
                    self.boost = 5 + combat_level.base * 15 / 100;
                } else if herblore_level.base >= 12 {
                    self.boost = 3 + combat_level.base / 10;
                }
            }
            PotionStat::Defence => {
                if herblore_level.base >= 70 {
                    self.boost = 7 + combat_level.base / 5;
                } else if herblore_level.base >= 65 {
                    self.boost = 5 + combat_level.base * 15 / 100;
                } else if herblore_level.base >= 30 {
                    self.boost = 3 + combat_level.base / 10;
                }
            }
            _ => {}
        }
    }
}

// Collection of active potion boosts, separated by combat type
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PotionBoosts {
    pub attack: Option<PotionBoost>,
    pub strength: Option<PotionBoost>,
    pub defence: Option<PotionBoost>,
    pub ranged: Option<PotionBoost>,
    pub magic: Option<PotionBoost>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::stats::Stat;
    #[test]
    fn test_dragon_battleaxe_boost() {
        let mut potion = PotionBoost::new(&Potion::DragonBattleaxe);
        potion.calc_dragon_battleaxe_boost(
            Stat::from(120),
            Stat::from(118),
            Stat::from(112),
            Stat::from(103),
        );
        assert_eq!(potion.boost, 21);
    }
}
