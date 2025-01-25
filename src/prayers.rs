use std::fmt;
use strum_macros::Display;

// Most combat-related prayers (excluding protection prayers)
#[derive(Debug, Default, PartialEq, Clone, Display)]
pub enum Prayer {
    #[default]
    None,
    #[strum(to_string = "Clarity of Thought")]
    ClarityOfThought,
    #[strum(to_string = "Improved Reflexes")]
    ImprovedReflexes,
    #[strum(to_string = "Incredible Reflexes")]
    IncredibleReflexes,
    #[strum(to_string = "Chivalry")]
    Chivalry,
    #[strum(to_string = "Piety")]
    Piety,
    #[strum(to_string = "Burst of Strength")]
    BurstOfStrength,
    #[strum(to_string = "Superhuman Strength")]
    SuperhumanStrength,
    #[strum(to_string = "Ultimate Strength")]
    UltimateStrength,
    #[strum(to_string = "Thick Skin")]
    ThickSkin,
    #[strum(to_string = "Rock Skin")]
    RockSkin,
    #[strum(to_string = "Steel Skin")]
    SteelSkin,
    #[strum(to_string = "Sharp Eye")]
    SharpEye,
    #[strum(to_string = "Hawk Eye")]
    HawkEye,
    #[strum(to_string = "Eagle Eye")]
    EagleEye,
    #[strum(to_string = "Rigour")]
    Rigour,
    #[strum(to_string = "Mystic Will")]
    MysticWill,
    #[strum(to_string = "Mystic Lore")]
    MysticLore,
    #[strum(to_string = "Mystic Might")]
    MysticMight,
    #[strum(to_string = "Augury")]
    Augury,
}

// Contains the type of prayer, and the percentage boost to each style
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PrayerBoost {
    pub prayer_type: Prayer,
    pub attack: u32,
    pub strength: u32,
    pub defence: u32,
    pub ranged_att: u32,
    pub ranged_str: u32,
    pub magic_att: u32,
    pub magic_str: u32,
}

impl fmt::Display for PrayerBoost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.prayer_type)
    }
}

impl PrayerBoost {
    pub fn new(prayer: Prayer) -> Self {
        match prayer {
            Prayer::ClarityOfThought => PrayerBoost {
                prayer_type: prayer,
                attack: 5,
                strength: 0,
                defence: 0,
                ranged_att: 0,
                ranged_str: 0,
                magic_att: 0,
                magic_str: 0,
            },
            Prayer::ImprovedReflexes => PrayerBoost {
                prayer_type: prayer,
                attack: 10,
                strength: 0,
                defence: 0,
                ranged_att: 0,
                ranged_str: 0,
                magic_att: 0,
                magic_str: 0,
            },
            Prayer::IncredibleReflexes => PrayerBoost {
                prayer_type: prayer,
                attack: 15,
                strength: 0,
                defence: 0,
                ranged_att: 0,
                ranged_str: 0,
                magic_att: 0,
                magic_str: 0,
            },
            Prayer::Chivalry => PrayerBoost {
                prayer_type: prayer,
                attack: 15,
                strength: 18,
                defence: 20,
                ranged_att: 0,
                ranged_str: 0,
                magic_att: 0,
                magic_str: 0,
            },
            Prayer::Piety => PrayerBoost {
                prayer_type: prayer,
                attack: 20,
                strength: 23,
                defence: 25,
                ranged_att: 0,
                ranged_str: 0,
                magic_att: 0,
                magic_str: 0,
            },
            Prayer::BurstOfStrength => PrayerBoost {
                prayer_type: prayer,
                attack: 0,
                strength: 5,
                defence: 0,
                ranged_att: 0,
                ranged_str: 0,
                magic_att: 0,
                magic_str: 0,
            },
            Prayer::SuperhumanStrength => PrayerBoost {
                prayer_type: prayer,
                attack: 0,
                strength: 10,
                defence: 0,
                ranged_att: 0,
                ranged_str: 0,
                magic_att: 0,
                magic_str: 0,
            },
            Prayer::UltimateStrength => PrayerBoost {
                prayer_type: prayer,
                attack: 0,
                strength: 15,
                defence: 0,
                ranged_att: 0,
                ranged_str: 0,
                magic_att: 0,
                magic_str: 0,
            },
            Prayer::ThickSkin => PrayerBoost {
                prayer_type: prayer,
                attack: 0,
                strength: 0,
                defence: 5,
                ranged_att: 0,
                ranged_str: 0,
                magic_att: 0,
                magic_str: 0,
            },
            Prayer::RockSkin => PrayerBoost {
                prayer_type: prayer,
                attack: 0,
                strength: 0,
                defence: 10,
                ranged_att: 0,
                ranged_str: 0,
                magic_att: 0,
                magic_str: 0,
            },
            Prayer::SteelSkin => PrayerBoost {
                prayer_type: prayer,
                attack: 0,
                strength: 0,
                defence: 15,
                ranged_att: 0,
                ranged_str: 0,
                magic_att: 0,
                magic_str: 0,
            },
            Prayer::SharpEye => PrayerBoost {
                prayer_type: prayer,
                attack: 0,
                strength: 0,
                defence: 0,
                ranged_att: 5,
                ranged_str: 5,
                magic_att: 0,
                magic_str: 0,
            },
            Prayer::HawkEye => PrayerBoost {
                prayer_type: prayer,
                attack: 0,
                strength: 0,
                defence: 0,
                ranged_att: 10,
                ranged_str: 10,
                magic_att: 0,
                magic_str: 0,
            },
            Prayer::EagleEye => PrayerBoost {
                prayer_type: prayer,
                attack: 0,
                strength: 0,
                defence: 0,
                ranged_att: 15,
                ranged_str: 15,
                magic_att: 0,
                magic_str: 0,
            },
            Prayer::Rigour => PrayerBoost {
                prayer_type: prayer,
                attack: 0,
                strength: 0,
                defence: 25,
                ranged_att: 20,
                ranged_str: 23,
                magic_att: 0,
                magic_str: 0,
            },
            Prayer::MysticWill => PrayerBoost {
                prayer_type: prayer,
                attack: 0,
                strength: 0,
                defence: 0,
                ranged_att: 0,
                ranged_str: 0,
                magic_att: 5,
                magic_str: 0,
            },
            Prayer::MysticLore => PrayerBoost {
                prayer_type: prayer,
                attack: 0,
                strength: 0,
                defence: 0,
                ranged_att: 0,
                ranged_str: 0,
                magic_att: 10,
                magic_str: 1,
            },
            Prayer::MysticMight => PrayerBoost {
                prayer_type: prayer,
                attack: 0,
                strength: 0,
                defence: 0,
                ranged_att: 0,
                ranged_str: 0,
                magic_att: 15,
                magic_str: 2,
            },
            Prayer::Augury => PrayerBoost {
                prayer_type: prayer,
                attack: 0,
                strength: 0,
                defence: 25,
                ranged_att: 0,
                ranged_str: 0,
                magic_att: 25,
                magic_str: 4,
            },
            _ => PrayerBoost {
                prayer_type: prayer,
                attack: 0,
                strength: 0,
                defence: 0,
                ranged_att: 0,
                ranged_str: 0,
                magic_att: 0,
                magic_str: 0,
            },
        }
    }

    pub fn conflicts_with(&self, p2: &PrayerBoost) -> bool {
        // Check if two prayer boosts conflict on any stats
        self.attack > 0 && (p2.attack > 0 || p2.ranged_att > 0 || p2.magic_att > 0)
            || self.strength > 0 && (p2.strength > 0 || p2.ranged_str > 0 || p2.magic_str > 0)
            || self.defence > 0 && p2.defence > 0
            || self.ranged_att > 0 && (p2.attack > 0 || p2.ranged_att > 0 || p2.magic_att > 0)
            || self.ranged_str > 0 && (p2.strength > 0 || p2.ranged_str > 0 || p2.magic_str > 0)
            || self.magic_att > 0 && (p2.attack > 0 || p2.ranged_att > 0 || p2.magic_att > 0)
            || self.magic_str > 0 && (p2.strength > 0 || p2.ranged_str > 0 || p2.magic_str > 0)
    }
}
