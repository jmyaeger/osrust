use crate::types::player::Player;
use std::cmp::min;
use strum_macros::Display;

// pub trait Spell: std::fmt::Debug {
//     fn max_hit(&self, player: &Player) -> u32;
//     fn as_any(&self) -> &dyn std::any::Any;
// }

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Spell {
    Standard(StandardSpell),
    Ancient(AncientSpell),
    Arceuus(ArceuusSpell),
    Special(SpecialSpell),
}

impl std::fmt::Display for Spell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spell::Standard(spell) => write!(f, "{spell}"),
            Spell::Ancient(spell) => write!(f, "{spell}"),
            Spell::Arceuus(spell) => write!(f, "{spell}"),
            Spell::Special(spell) => write!(f, "{spell}"),
        }
    }
}

impl Spell {
    pub fn max_hit(&self, player: &Player) -> u32 {
        match self {
            Spell::Standard(spell) => spell.max_hit(player),
            Spell::Ancient(spell) => spell.max_hit(),
            Spell::Arceuus(spell) => spell.max_hit(),
            Spell::Special(spell) => spell.max_hit(player),
        }
    }

    pub fn required_level(&self) -> u32 {
        match self {
            Spell::Standard(spell) => spell.required_level(),
            Spell::Ancient(spell) => spell.required_level(),
            Spell::Arceuus(spell) => spell.required_level(),
            Spell::Special(_) => 1,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy, Display)]
pub enum StandardSpell {
    #[default]
    None,
    #[strum(to_string = "Wind Strike")]
    WindStrike,
    #[strum(to_string = "Water Strike")]
    WaterStrike,
    #[strum(to_string = "Earth Strike")]
    EarthStrike,
    #[strum(to_string = "Fire Strike")]
    FireStrike,
    #[strum(to_string = "Wind Bolt")]
    WindBolt,
    #[strum(to_string = "Water Bolt")]
    WaterBolt,
    #[strum(to_string = "Earth Bolt")]
    EarthBolt,
    #[strum(to_string = "Fire Bolt")]
    FireBolt,
    #[strum(to_string = "Wind Blast")]
    WindBlast,
    #[strum(to_string = "Water Blast")]
    WaterBlast,
    #[strum(to_string = "Crumble Undead")]
    CrumbleUndead,
    #[strum(to_string = "Earth Blast")]
    EarthBlast,
    #[strum(to_string = "Fire Blast")]
    FireBlast,
    #[strum(to_string = "Wind Wave")]
    WindWave,
    #[strum(to_string = "Water Wave")]
    WaterWave,
    #[strum(to_string = "Earth Wave")]
    EarthWave,
    #[strum(to_string = "Fire Wave")]
    FireWave,
    #[strum(to_string = "Saradomin Strike")]
    SaradominStrike,
    #[strum(to_string = "Claws of Guthix")]
    ClawsOfGuthix,
    #[strum(to_string = "Flames of Zamorak")]
    FlamesOfZamorak,
    #[strum(to_string = "Wind Surge")]
    WindSurge,
    #[strum(to_string = "Water Surge")]
    WaterSurge,
    #[strum(to_string = "Earth Surge")]
    EarthSurge,
    #[strum(to_string = "Fire Surge")]
    FireSurge,
    #[strum(to_string = "Iban Blast")]
    IbanBlast,
    #[strum(to_string = "Magic Dart")]
    MagicDart,
    Bind,
    Snare,
    Entangle,
}

impl StandardSpell {
    pub fn max_hit(&self, player: &Player) -> u32 {
        match self {
            StandardSpell::WindStrike | StandardSpell::WaterStrike | StandardSpell::EarthStrike => {
                strike_spell_max_hit(player)
            }
            StandardSpell::FireStrike => 8,
            StandardSpell::WindBolt | StandardSpell::WaterBolt | StandardSpell::EarthBolt => {
                bolt_spell_max_hit(player)
            }
            StandardSpell::FireBolt => 12,
            StandardSpell::WindBlast | StandardSpell::WaterBlast | StandardSpell::EarthBlast => {
                blast_spell_max_hit(player)
            }
            StandardSpell::CrumbleUndead => 15,
            StandardSpell::FireBlast => 16,
            StandardSpell::WindWave | StandardSpell::WaterWave | StandardSpell::EarthWave => {
                wave_spell_max_hit(player)
            }
            StandardSpell::FireWave
            | StandardSpell::SaradominStrike
            | StandardSpell::ClawsOfGuthix
            | StandardSpell::FlamesOfZamorak => 20,
            StandardSpell::WindSurge | StandardSpell::WaterSurge | StandardSpell::EarthSurge => {
                surge_spell_max_hit(player)
            }
            StandardSpell::FireSurge => 24,
            StandardSpell::IbanBlast => 25,
            StandardSpell::MagicDart => magic_dart_max_hit(player),
            _ => 0,
        }
    }

    pub fn required_level(&self) -> u32 {
        match self {
            StandardSpell::None | StandardSpell::WindStrike => 1,
            StandardSpell::WaterStrike => 5,
            StandardSpell::EarthStrike => 9,
            StandardSpell::FireStrike => 13,
            StandardSpell::WindBolt => 17,
            StandardSpell::WaterBolt => 23,
            StandardSpell::EarthBolt => 29,
            StandardSpell::FireBolt => 35,
            StandardSpell::WindBlast => 41,
            StandardSpell::WaterBlast => 47,
            StandardSpell::EarthBlast => 53,
            StandardSpell::CrumbleUndead => 39,
            StandardSpell::FireBlast => 59,
            StandardSpell::WindWave => 62,
            StandardSpell::WaterWave => 65,
            StandardSpell::EarthWave => 70,
            StandardSpell::FireWave => 75,
            StandardSpell::SaradominStrike
            | StandardSpell::ClawsOfGuthix
            | StandardSpell::FlamesOfZamorak => 60,
            StandardSpell::WindSurge => 81,
            StandardSpell::WaterSurge => 85,
            StandardSpell::EarthSurge => 90,
            StandardSpell::FireSurge => 95,
            StandardSpell::IbanBlast | StandardSpell::MagicDart | StandardSpell::Snare => 50,
            StandardSpell::Bind => 20,
            StandardSpell::Entangle => 79,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Display)]
pub enum AncientSpell {
    #[strum(to_string = "Smoke Rush")]
    SmokeRush,
    #[strum(to_string = "Shadow Rush")]
    ShadowRush,
    #[strum(to_string = "Blood Rush")]
    BloodRush,
    #[strum(to_string = "Ice Rush")]
    IceRush,
    #[strum(to_string = "Smoke Burst")]
    SmokeBurst,
    #[strum(to_string = "Shadow Burst")]
    ShadowBurst,
    #[strum(to_string = "Blood Burst")]
    BloodBurst,
    #[strum(to_string = "Ice Burst")]
    IceBurst,
    #[strum(to_string = "Smoke Blitz")]
    SmokeBlitz,
    #[strum(to_string = "Shadow Blitz")]
    ShadowBlitz,
    #[strum(to_string = "Blood Blitz")]
    BloodBlitz,
    #[strum(to_string = "Ice Blitz")]
    IceBlitz,
    #[strum(to_string = "Smoke Barrage")]
    SmokeBarrage,
    #[strum(to_string = "Shadow Barrage")]
    ShadowBarrage,
    #[strum(to_string = "Blood Barrage")]
    BloodBarrage,
    #[strum(to_string = "Ice Barrage")]
    IceBarrage,
}

impl AncientSpell {
    pub fn max_hit(&self) -> u32 {
        match self {
            AncientSpell::SmokeRush => 13,
            AncientSpell::ShadowRush => 14,
            AncientSpell::BloodRush => 15,
            AncientSpell::IceRush => 16,
            AncientSpell::SmokeBurst => 17,
            AncientSpell::ShadowBurst => 18,
            AncientSpell::BloodBurst => 21,
            AncientSpell::IceBurst => 22,
            AncientSpell::SmokeBlitz => 23,
            AncientSpell::ShadowBlitz => 24,
            AncientSpell::BloodBlitz => 25,
            AncientSpell::IceBlitz => 26,
            AncientSpell::SmokeBarrage => 27,
            AncientSpell::ShadowBarrage => 28,
            AncientSpell::BloodBarrage => 29,
            AncientSpell::IceBarrage => 30,
        }
    }

    pub fn required_level(&self) -> u32 {
        match self {
            AncientSpell::SmokeRush => 50,
            AncientSpell::ShadowRush => 52,
            AncientSpell::BloodRush => 56,
            AncientSpell::IceRush => 58,
            AncientSpell::SmokeBurst => 62,
            AncientSpell::ShadowBurst => 64,
            AncientSpell::BloodBurst => 68,
            AncientSpell::IceBurst => 70,
            AncientSpell::SmokeBlitz => 74,
            AncientSpell::ShadowBlitz => 76,
            AncientSpell::BloodBlitz => 80,
            AncientSpell::IceBlitz => 82,
            AncientSpell::SmokeBarrage => 86,
            AncientSpell::ShadowBarrage => 88,
            AncientSpell::BloodBarrage => 92,
            AncientSpell::IceBarrage => 94,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Display)]
pub enum ArceuusSpell {
    #[strum(to_string = "Ghostly Grasp")]
    GhostlyGrasp,
    #[strum(to_string = "Skeletal Grasp")]
    SkeletalGrasp,
    #[strum(to_string = "Undead Grasp")]
    UndeadGrasp,
    #[strum(to_string = "Inferior Demonbane")]
    InferiorDemonbane,
    #[strum(to_string = "Superior Demonbane")]
    SuperiorDemonbane,
    #[strum(to_string = "Dark Demonbane")]
    DarkDemonbane,
}

impl ArceuusSpell {
    pub fn max_hit(&self) -> u32 {
        match self {
            ArceuusSpell::GhostlyGrasp => 12,
            ArceuusSpell::SkeletalGrasp => 17,
            ArceuusSpell::UndeadGrasp => 24,
            ArceuusSpell::InferiorDemonbane => 16,
            ArceuusSpell::SuperiorDemonbane => 23,
            ArceuusSpell::DarkDemonbane => 30,
        }
    }

    pub fn required_level(&self) -> u32 {
        match self {
            ArceuusSpell::GhostlyGrasp => 35,
            ArceuusSpell::SkeletalGrasp => 56,
            ArceuusSpell::UndeadGrasp => 79,
            ArceuusSpell::InferiorDemonbane => 44,
            ArceuusSpell::SuperiorDemonbane => 62,
            ArceuusSpell::DarkDemonbane => 82,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Display)]
pub enum SpecialSpell {
    Invocate,
    Immolate,
}

impl SpecialSpell {
    pub fn max_hit(&self, player: &Player) -> u32 {
        match self {
            SpecialSpell::Invocate => min((99 + player.stats.magic.current * 44) / 99, 44),
            SpecialSpell::Immolate => min((99 + player.stats.magic.current * 58) / 99, 58),
        }
    }
}

fn magic_dart_max_hit(player: &Player) -> u32 {
    if player.is_wearing("Slayer's staff (e)", None) || player.boosts.on_task {
        13 + player.stats.magic.current / 6
    } else {
        10 + player.stats.magic.current / 10
    }
}

fn strike_spell_max_hit(player: &Player) -> u32 {
    min(8, player.stats.magic.current.div_ceil(4) * 2)
}

fn bolt_spell_max_hit(player: &Player) -> u32 {
    min(12, (player.stats.magic.current + 1) / 6 + 6)
}

fn blast_spell_max_hit(player: &Player) -> u32 {
    min(16, (player.stats.magic.current + 1) / 6 + 6)
}

fn wave_spell_max_hit(player: &Player) -> u32 {
    min(20, player.stats.magic.current / 5 + 5)
}

fn surge_spell_max_hit(player: &Player) -> u32 {
    min(24, player.stats.magic.current / 5 + 5)
}

macro_rules! spell_check {
    (
        $(
            $fn_name:ident => $spellbook:ident: [$($variant:ident),* $(,)?]
        ),*
        $(,)?
    ) => {
        paste::paste! {
            $(
                pub fn $fn_name(spell: &Spell) -> bool {
                    matches!(
                        spell,
                        $(Spell::$spellbook([<$spellbook Spell>]::$variant))|*
                    )
                }
            )*
        }
    };
}

pub fn is_standard_spell(spell: &Spell) -> bool {
    matches!(spell, Spell::Standard(_))
}

pub fn is_ancient_spell(spell: &Spell) -> bool {
    matches!(spell, Spell::Ancient(_))
}

pub fn is_arceuus_spell(spell: &Spell) -> bool {
    matches!(spell, Spell::Arceuus(_))
}

spell_check!(
    is_water_spell => Standard: [WaterStrike, WaterBolt, WaterBlast, WaterWave, WaterSurge],
    is_fire_spell => Standard: [FireStrike, FireBolt, FireBlast, FireWave, FireSurge],
    is_earth_spell => Standard: [EarthStrike, EarthBolt, EarthBlast, EarthWave, EarthSurge],
    is_air_spell => Standard: [WindStrike, WindBolt, WindBlast, WindWave, WindSurge],
    is_smoke_spell => Ancient: [SmokeRush, SmokeBurst, SmokeBlitz, SmokeBarrage],
    is_shadow_spell => Ancient: [ShadowRush, ShadowBurst, ShadowBlitz, ShadowBarrage],
    is_blood_spell => Ancient: [BloodRush, BloodBurst, BloodBlitz, BloodBarrage],
    is_ice_spell => Ancient: [IceRush, IceBurst, IceBlitz, IceBarrage],
    is_demonbane_spell => Arceuus: [InferiorDemonbane, SuperiorDemonbane, DarkDemonbane],
    is_grasp_spell => Arceuus: [GhostlyGrasp, SkeletalGrasp, UndeadGrasp],
    is_bolt_spell => Standard: [WindBolt, EarthBolt, WaterBolt, FireBolt],
    is_blast_spell => Standard: [WindBlast, EarthBlast, WaterBlast, FireBlast],
    is_wave_spell => Standard: [WindWave, EarthWave, WaterWave, FireWave],
);

pub fn is_bind_spell(spell: &Spell) -> bool {
    is_ice_spell(spell)
        || is_grasp_spell(spell)
        || matches!(
            spell,
            Spell::Standard(StandardSpell::Bind | StandardSpell::Snare | StandardSpell::Entangle)
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::player::Player;

    #[test]
    fn test_strike_max_hits() {
        let mut player = Player::new();
        let _ = player.set_spell(Spell::Standard(StandardSpell::WindStrike));
        for level in 1..5 {
            player.stats.magic.current = level;
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 2);
        }

        for level in 5..9 {
            player.stats.magic.current = level;
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 4);
            let _ = player.set_spell(Spell::Standard(StandardSpell::WaterStrike));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 4);
        }

        for level in 9..13 {
            player.stats.magic.current = level;
            let _ = player.set_spell(Spell::Standard(StandardSpell::WindStrike));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 6);
            let _ = player.set_spell(Spell::Standard(StandardSpell::WaterStrike));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 6);
            let _ = player.set_spell(Spell::Standard(StandardSpell::EarthStrike));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 6);
        }

        for level in 13..20 {
            player.stats.magic.current = level;
            let _ = player.set_spell(Spell::Standard(StandardSpell::WindStrike));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 8);
            let _ = player.set_spell(Spell::Standard(StandardSpell::WaterStrike));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 8);
            let _ = player.set_spell(Spell::Standard(StandardSpell::EarthStrike));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 8);
            let _ = player.set_spell(Spell::Standard(StandardSpell::FireStrike));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 8);
        }
    }

    #[test]
    fn test_bolt_max_hits() {
        let mut player = Player::new();
        let _ = player.set_spell(Spell::Standard(StandardSpell::WindBolt));
        for level in 17..23 {
            player.stats.magic.current = level;
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 9);
        }

        for level in 23..29 {
            player.stats.magic.current = level;
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 10);
            let _ = player.set_spell(Spell::Standard(StandardSpell::WaterBolt));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 10);
        }

        for level in 29..35 {
            player.stats.magic.current = level;
            let _ = player.set_spell(Spell::Standard(StandardSpell::WindBolt));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 11);
            let _ = player.set_spell(Spell::Standard(StandardSpell::WaterBolt));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 11);
            let _ = player.set_spell(Spell::Standard(StandardSpell::EarthBolt));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 11);
        }

        for level in 35..42 {
            player.stats.magic.current = level;
            let _ = player.set_spell(Spell::Standard(StandardSpell::WindBolt));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 12);
            let _ = player.set_spell(Spell::Standard(StandardSpell::WaterBolt));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 12);
            let _ = player.set_spell(Spell::Standard(StandardSpell::EarthBolt));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 12);
            let _ = player.set_spell(Spell::Standard(StandardSpell::FireBolt));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 12);
        }
    }

    #[test]
    fn test_blast_max_hits() {
        let mut player = Player::new();
        let _ = player.set_spell(Spell::Standard(StandardSpell::WindBlast));
        for level in 41..47 {
            player.stats.magic.current = level;
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 13);
        }

        for level in 47..53 {
            player.stats.magic.current = level;
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 14);
            let _ = player.set_spell(Spell::Standard(StandardSpell::WaterBlast));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 14);
        }

        for level in 53..59 {
            player.stats.magic.current = level;
            let _ = player.set_spell(Spell::Standard(StandardSpell::WindBlast));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 15);
            let _ = player.set_spell(Spell::Standard(StandardSpell::WaterBlast));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 15);
            let _ = player.set_spell(Spell::Standard(StandardSpell::EarthBlast));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 15);
        }

        for level in 59..66 {
            player.stats.magic.current = level;
            let _ = player.set_spell(Spell::Standard(StandardSpell::WindBlast));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 16);
            let _ = player.set_spell(Spell::Standard(StandardSpell::WaterBlast));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 16);
            let _ = player.set_spell(Spell::Standard(StandardSpell::EarthBlast));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 16);
            let _ = player.set_spell(Spell::Standard(StandardSpell::FireBlast));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 16);
        }
    }

    #[test]
    fn test_wave_max_hits() {
        let mut player = Player::new();
        let _ = player.set_spell(Spell::Standard(StandardSpell::WindWave));
        for level in 62..65 {
            player.stats.magic.current = level;
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 17);
        }

        for level in 65..70 {
            player.stats.magic.current = level;
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 18);
            let _ = player.set_spell(Spell::Standard(StandardSpell::WaterWave));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 18);
        }

        for level in 70..75 {
            player.stats.magic.current = level;
            let _ = player.set_spell(Spell::Standard(StandardSpell::WindWave));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 19);
            let _ = player.set_spell(Spell::Standard(StandardSpell::WaterWave));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 19);
            let _ = player.set_spell(Spell::Standard(StandardSpell::EarthWave));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 19);
        }

        for level in 75..81 {
            player.stats.magic.current = level;
            let _ = player.set_spell(Spell::Standard(StandardSpell::WindWave));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 20);
            let _ = player.set_spell(Spell::Standard(StandardSpell::WaterWave));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 20);
            let _ = player.set_spell(Spell::Standard(StandardSpell::EarthWave));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 20);
            let _ = player.set_spell(Spell::Standard(StandardSpell::FireWave));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 20);
        }
    }

    #[test]
    fn test_surge_max_hits() {
        let mut player = Player::new();
        let _ = player.set_spell(Spell::Standard(StandardSpell::WindSurge));
        for level in 81..85 {
            player.stats.magic.current = level;
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 21);
        }

        for level in 85..90 {
            player.stats.magic.current = level;
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 22);
            let _ = player.set_spell(Spell::Standard(StandardSpell::WaterSurge));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 22);
        }

        for level in 90..95 {
            player.stats.magic.current = level;
            let _ = player.set_spell(Spell::Standard(StandardSpell::WindSurge));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 23);
            let _ = player.set_spell(Spell::Standard(StandardSpell::WaterSurge));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 23);
            let _ = player.set_spell(Spell::Standard(StandardSpell::EarthSurge));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 23);
        }

        for level in 95..101 {
            player.stats.magic.current = level;
            let _ = player.set_spell(Spell::Standard(StandardSpell::WindSurge));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 24);
            let _ = player.set_spell(Spell::Standard(StandardSpell::WaterSurge));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 24);
            let _ = player.set_spell(Spell::Standard(StandardSpell::EarthSurge));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 24);
            let _ = player.set_spell(Spell::Standard(StandardSpell::FireSurge));
            assert_eq!(player.attrs.spell.unwrap().max_hit(&player), 24);
        }
    }
}
