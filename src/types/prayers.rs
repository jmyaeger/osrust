use std::fmt;
use strum_macros::Display;

// Most combat-related prayers (excluding protection prayers)
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Display)]
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
    #[strum(to_string = "Deadeye")]
    Deadeye,
    #[strum(to_string = "Rigour")]
    Rigour,
    #[strum(to_string = "Mystic Will")]
    MysticWill,
    #[strum(to_string = "Mystic Lore")]
    MysticLore,
    #[strum(to_string = "Mystic Might")]
    MysticMight,
    #[strum(to_string = "Mystic Vigour")]
    MysticVigour,
    #[strum(to_string = "Augury")]
    Augury,
}

// Contains the type of prayer, and the percentage boost to each style
#[derive(Debug, Default, PartialEq, Clone, Copy, Eq)]
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
            Prayer::Deadeye => PrayerBoost {
                prayer_type: prayer,
                attack: 0,
                strength: 0,
                defence: 5,
                ranged_att: 18,
                ranged_str: 18,
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
            Prayer::MysticVigour => PrayerBoost {
                prayer_type: prayer,
                attack: 0,
                strength: 0,
                defence: 5,
                ranged_att: 0,
                ranged_str: 0,
                magic_att: 18,
                magic_str: 3,
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

        // Deadeye and mystic vigour defence stacks with steel skin, etc.
        let stacks_defence = self.prayer_type == Prayer::Deadeye
            || p2.prayer_type == Prayer::Deadeye
            || self.prayer_type == Prayer::MysticVigour
            || p2.prayer_type == Prayer::MysticVigour;

        self.attack > 0 && (p2.attack > 0 || p2.ranged_att > 0 || p2.magic_att > 0)
            || self.strength > 0 && (p2.strength > 0 || p2.ranged_str > 0 || p2.magic_str > 0)
            || !stacks_defence && self.defence > 0 && p2.defence > 0
            || self.ranged_att > 0 && (p2.attack > 0 || p2.ranged_att > 0 || p2.magic_att > 0)
            || self.ranged_str > 0 && (p2.strength > 0 || p2.ranged_str > 0 || p2.magic_str > 0)
            || self.magic_att > 0 && (p2.attack > 0 || p2.ranged_att > 0 || p2.magic_att > 0)
            || self.magic_str > 0 && (p2.strength > 0 || p2.ranged_str > 0 || p2.magic_str > 0)
    }
}

// Collection of active prayers and their cumulative boosts
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PrayerBoosts {
    pub active_prayers: Option<Vec<PrayerBoost>>,
    pub attack: u32,
    pub strength: u32,
    pub defence: u32,
    pub ranged_att: u32,
    pub ranged_str: u32,
    pub magic_att: u32,
    pub magic_str: u32,
}

impl PrayerBoosts {
    pub fn add(&mut self, prayer: Prayer) {
        let prayer_boost = PrayerBoost::new(prayer);
        match &mut self.active_prayers {
            Some(active_prayers) => {
                // Remove any conflicting prayer boosts first
                active_prayers.retain(|p| !p.conflicts_with(&prayer_boost));
                active_prayers.push(prayer_boost);
            }
            None => {
                self.active_prayers = Some(vec![prayer_boost]);
            }
        }

        self.update_prayer_boosts();
    }

    pub fn remove(&mut self, prayer: Prayer) {
        let prayer_boost = PrayerBoost::new(prayer);
        if let Some(active_prayers) = &mut self.active_prayers {
            active_prayers.retain(|p| p != &prayer_boost);

            self.update_prayer_boosts();
        }
    }

    fn update_prayer_boosts(&mut self) {
        // Search through active prayers and returns the highest boost values for all stats
        if let Some(prayers) = &mut self.active_prayers {
            self.attack = prayers.iter().map(|p| p.attack).max().unwrap_or(0);
            self.strength = prayers.iter().map(|p| p.strength).max().unwrap_or(0);

            // Defence boosts can now be additive between Deadeye/Mystic Vigour and Skin prayers
            // TODO: Write a test to verify that add() is properly eliminating conflicting prayers, as
            //       otherwise this will result in a higher-than-expected defence boost
            // TODO: Consider changing all of these to sum() after checking that the above works
            self.defence = prayers.iter().map(|p| p.defence).sum::<u32>();
            self.ranged_att = prayers.iter().map(|p| p.ranged_att).max().unwrap_or(0);
            self.ranged_str = prayers.iter().map(|p| p.ranged_str).max().unwrap_or(0);
            self.magic_att = prayers.iter().map(|p| p.magic_att).max().unwrap_or(0);
            self.magic_str = prayers.iter().map(|p| p.magic_str).max().unwrap_or(0);
        }
    }

    pub fn contains_prayer(&self, prayer: Prayer) -> bool {
        self.active_prayers
            .as_ref()
            .is_some_and(|prayers| prayers.iter().any(|p| p.prayer_type == prayer))
    }
}

mod test {
    #![allow(unused)]
    use super::*;

    #[test]
    fn test_contains_prayer() {
        let mut prayers = PrayerBoosts::default();
        prayers.add(Prayer::Augury);
        assert!(prayers.contains_prayer(Prayer::Augury));
        prayers.remove(Prayer::Augury);
        assert!(!prayers.contains_prayer(Prayer::Augury));
    }
}
