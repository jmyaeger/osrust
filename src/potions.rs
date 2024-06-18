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
            | Potion::SuperMagic => PotionBoost {
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
            _ => PotionBoost {
                potion_type: *potion,
                factor: 0,
                constant: 0,
                boost: 0,
            },
        }
    }

    pub fn calc_boost(&mut self, level: u32) {
        // Calculate the level boost based on the player's base level
        self.boost = self.factor * level / 100 + self.constant;
    }

    pub fn calc_dragon_battleaxe_boost(
        &mut self,
        att_level: u32,
        def_level: u32,
        ranged_level: u32,
        magic_level: u32,
    ) {
        // DBA boost gets its own function
        let stats = [att_level, def_level, ranged_level, magic_level];
        let sum: u32 = stats.iter().map(|&n| n / 10).sum();
        self.boost = 10 + (sum / 4);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dragon_battleaxe_boost() {
        let mut potion = PotionBoost::new(&Potion::DragonBattleaxe);
        potion.calc_dragon_battleaxe_boost(120, 118, 112, 103);
        assert_eq!(potion.boost, 21);
    }
}
