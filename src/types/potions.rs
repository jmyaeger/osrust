use crate::types::stats::Stat;
use strum_macros::{Display, EnumIter};

// All types of potions or combat level boosting items
#[derive(Debug, Default, PartialEq, Copy, Clone, Display, EnumIter)]
pub enum Potion {
    #[default]
    #[strum(to_string = "None")]
    None,
    #[strum(to_string = "Attack")]
    Attack,
    #[strum(to_string = "Strength")]
    Strength,
    #[strum(to_string = "Defence")]
    Defence,
    #[strum(to_string = "Ranging")]
    Ranging,
    #[strum(to_string = "Magic")]
    Magic,
    #[strum(to_string = "Super attack")]
    SuperAttack,
    #[strum(to_string = "Super strength")]
    SuperStrength,
    #[strum(to_string = "Super defence")]
    SuperDefence,
    #[strum(to_string = "Super combat")]
    SuperCombat,
    #[strum(to_string = "Super ranging")]
    SuperRanging,
    #[strum(to_string = "Super magic")]
    SuperMagic,
    #[strum(to_string = "Overload (-)")]
    OverloadMinus,
    #[strum(to_string = "Overload")]
    Overload,
    #[strum(to_string = "Overload (+)")]
    OverloadPlus,
    #[strum(to_string = "Zamorak brew")]
    ZamorakBrew,
    #[strum(to_string = "Smelling salts")]
    SmellingSalts,
    #[strum(to_string = "Dragon battleaxe")]
    DragonBattleaxe,
    #[strum(to_string = "Saradomin brew")]
    SaradominBrew,
    #[strum(to_string = "Ancient brew")]
    AncientBrew,
    #[strum(to_string = "Forgotten brew")]
    ForgottenBrew,
    #[strum(to_string = "Imbued heart")]
    ImbuedHeart,
    #[strum(to_string = "Saturated heart")]
    SaturatedHeart,
    #[strum(to_string = "Ruby Harvest")]
    RubyHarvest,
    #[strum(to_string = "Black Warlock")]
    BlackWarlock,
    #[strum(to_string = "Sapphire Glacialis")]
    SapphireGlacialis,
    #[strum(to_string = "Moonlight")]
    Moonlight,
}

impl Potion {
    pub fn boosts_attack(&self) -> bool {
        self == &Potion::Attack
            || self == &Potion::SuperAttack
            || self == &Potion::ZamorakBrew
            || self == &Potion::RubyHarvest
    }

    pub fn boosts_strength(&self) -> bool {
        self == &Potion::Strength
            || self == &Potion::SuperStrength
            || self == &Potion::ZamorakBrew
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

macro_rules! potion_boost {
    (
        $potion_var:ident;
        $(
            $($potion:ident)|+ => { $($field:ident: $value:expr),* $(,)? }
        ),*
        $(,)?
    ) => {
        match $potion_var {
            $(
                $(Potion::$potion)|+ => PotionBoost {
                    potion_type: *$potion_var,
                    $($field: $value,)*
                    ..Default::default()
                },
            )*
            _ => PotionBoost {
                potion_type: *$potion_var,
                ..Default::default()
            }
        }
    };
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
        potion_boost!(potion;
            Attack | Strength | Defence => { factor: 10, constant: 3 },
            Ranging | OverloadMinus | SaturatedHeart => { factor: 10, constant: 4 },
            Magic => { factor: 0, constant: 4 },
            SuperAttack | SuperStrength | SuperDefence | SuperRanging | SuperMagic | SuperCombat => { factor: 15, constant: 5 },
            Overload => { factor: 13, constant: 5 },
            OverloadPlus => { factor: 16, constant: 6 },
            SaradominBrew => { factor: 20, constant: 2 },
            AncientBrew => { factor: 5, constant: 2 },
            ForgottenBrew => { factor: 8, constant: 3 },
            ImbuedHeart => { factor: 10, constant: 1 },
            SmellingSalts => { factor: 16, constant: 11 },
            RubyHarvest | SapphireGlacialis | BlackWarlock => { factor: 15, constant: 4 }
        )
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
        skill: &PotionStat,
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

    pub fn calc_zamorak_brew_boost(&mut self, combat_level: Stat, skill: &PotionStat) {
        match skill {
            PotionStat::Attack => self.boost = 2 + combat_level.base * 20 / 100,
            PotionStat::Strength => self.boost = 2 + combat_level.base * 12 / 100,
            _ => {}
        }
    }
}

// Collection of active potion boosts, separated by combat type
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PotionBoosts {
    pub attack: Option<Vec<PotionBoost>>,
    pub strength: Option<Vec<PotionBoost>>,
    pub defence: Option<Vec<PotionBoost>>,
    pub ranged: Option<Vec<PotionBoost>>,
    pub magic: Option<Vec<PotionBoost>>,
}

impl PotionBoosts {
    pub fn remove_potion(&mut self, potion: Potion) {
        if let Some(ref mut attack) = self.attack {
            attack.retain(|p| p.potion_type != potion);
        }
        if let Some(ref mut strength) = self.strength {
            strength.retain(|p| p.potion_type != potion);
        }
        if let Some(ref mut defence) = self.defence {
            defence.retain(|p| p.potion_type != potion);
        }
        if let Some(ref mut ranged) = self.ranged {
            ranged.retain(|p| p.potion_type != potion);
        }
        if let Some(ref mut magic) = self.magic {
            magic.retain(|p| p.potion_type != potion);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::stats::Stat;

    #[test]
    fn test_smelling_salt_boost() {
        let mut potion = PotionBoost::new(&Potion::SmellingSalts);
        potion.calc_boost(Stat::from(99));
        assert_eq!(potion.boost, 26);
    }

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
