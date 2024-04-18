#[derive(Debug, Default, PartialEq)]
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

#[derive(Debug, Default, PartialEq)]
pub struct PotionBoost {
    pub potion_type: Potion,
    pub factor: u16,
    pub constant: u16,
    pub boost: u16,
}

impl PotionBoost {
    pub fn new(potion: Potion) -> Self {
        match potion {
            Potion::Attack | Potion::Strength | Potion::Defence => PotionBoost {
                potion_type: potion,
                factor: 10,
                constant: 3,
                boost: 0,
            },
            Potion::Ranging => PotionBoost {
                potion_type: potion,
                factor: 10,
                constant: 4,
                boost: 0,
            },
            Potion::Magic => PotionBoost {
                potion_type: potion,
                factor: 0,
                constant: 4,
                boost: 0,
            },
            Potion::SuperAttack
            | Potion::SuperStrength
            | Potion::SuperDefence
            | Potion::SuperRanging
            | Potion::SuperMagic => PotionBoost {
                potion_type: potion,
                factor: 15,
                constant: 5,
                boost: 0,
            },
            Potion::OverloadMinus => PotionBoost {
                potion_type: potion,
                factor: 10,
                constant: 4,
                boost: 0,
            },
            Potion::Overload => PotionBoost {
                potion_type: potion,
                factor: 13,
                constant: 5,
                boost: 0,
            },
            Potion::OverloadPlus => PotionBoost {
                potion_type: potion,
                factor: 16,
                constant: 6,
                boost: 0,
            },
            Potion::ZamorakBrewAtt => PotionBoost {
                potion_type: potion,
                factor: 20,
                constant: 2,
                boost: 0,
            },
            Potion::ZamorakBrewStr => PotionBoost {
                potion_type: potion,
                factor: 12,
                constant: 2,
                boost: 0,
            },
            Potion::DragonBattleaxe => PotionBoost {
                potion_type: potion,
                factor: 0,
                constant: 0,
                boost: 0, // Custom implementation
            },
            Potion::SaradominBrew => PotionBoost {
                potion_type: potion,
                factor: 20,
                constant: 2,
                boost: 0,
            },
            Potion::AncientBrew => PotionBoost {
                potion_type: potion,
                factor: 5,
                constant: 2,
                boost: 0,
            },
            Potion::ForgottenBrew => PotionBoost {
                potion_type: potion,
                factor: 8,
                constant: 3,
                boost: 0,
            },
            Potion::ImbuedHeart => PotionBoost {
                potion_type: potion,
                factor: 10,
                constant: 1,
                boost: 0,
            },
            Potion::SaturatedHeart => PotionBoost {
                potion_type: potion,
                factor: 10,
                constant: 4,
                boost: 0,
            },
            _ => PotionBoost {
                potion_type: potion,
                factor: 0,
                constant: 0,
                boost: 0,
            },
        }
    }

    pub fn calc_boost(&mut self, level: u16) {
        self.boost = self.factor * level / 100 + self.constant;
    }

    pub fn calc_dragon_battleaxe_boost(
        &mut self,
        att_level: u16,
        def_level: u16,
        ranged_level: u16,
        magic_level: u16,
    ) {
        let stats = [att_level, def_level, ranged_level, magic_level];
        let sum: u16 = stats.iter().map(|&n| n / 10).sum();
        self.boost = 10 + (sum / 4);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dragon_battleaxe_boost() {
        let mut potion = PotionBoost::new(Potion::DragonBattleaxe);
        potion.calc_dragon_battleaxe_boost(120, 118, 112, 103);
        assert_eq!(potion.boost, 21);
    }
}
