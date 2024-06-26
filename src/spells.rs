use crate::player::Player;
use std::cmp::min;

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

impl Spell {
    pub fn max_hit(&self, player: &Player) -> u32 {
        match self {
            Spell::Standard(spell) => spell.max_hit(player),
            Spell::Ancient(spell) => spell.max_hit(),
            Spell::Arceuus(spell) => spell.max_hit(),
            Spell::Special(spell) => spell.max_hit(player),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum StandardSpell {
    #[default]
    None,
    WindStrike,
    WaterStrike,
    EarthStrike,
    FireStrike,
    WindBolt,
    WaterBolt,
    EarthBolt,
    FireBolt,
    WindBlast,
    WaterBlast,
    CrumbleUndead,
    EarthBlast,
    FireBlast,
    WindWave,
    WaterWave,
    EarthWave,
    FireWave,
    SaradominStrike,
    ClawsOfGuthix,
    FlamesOfZamorak,
    WindSurge,
    WaterSurge,
    EarthSurge,
    FireSurge,
    IbanBlast,
    MagicDart,
    Bind,
    Snare,
    Entangle,
}

impl StandardSpell {
    pub fn max_hit(&self, player: &Player) -> u32 {
        match self {
            StandardSpell::WindStrike => 2,
            StandardSpell::WaterStrike => 4,
            StandardSpell::EarthStrike => 6,
            StandardSpell::FireStrike => 8,
            StandardSpell::WindBolt => 9,
            StandardSpell::WaterBolt => 10,
            StandardSpell::EarthBolt => 11,
            StandardSpell::FireBolt => 12,
            StandardSpell::WindBlast => 13,
            StandardSpell::WaterBlast => 14,
            StandardSpell::EarthBlast => 15,
            StandardSpell::CrumbleUndead => 15,
            StandardSpell::FireBlast => 16,
            StandardSpell::WindWave => 17,
            StandardSpell::WaterWave => 18,
            StandardSpell::EarthWave => 19,
            StandardSpell::FireWave => 20,
            StandardSpell::SaradominStrike => 20,
            StandardSpell::ClawsOfGuthix => 20,
            StandardSpell::FlamesOfZamorak => 20,
            StandardSpell::WindSurge => 21,
            StandardSpell::WaterSurge => 22,
            StandardSpell::EarthSurge => 23,
            StandardSpell::FireSurge => 24,
            StandardSpell::IbanBlast => 25,
            StandardSpell::MagicDart => magic_dart_max_hit(player),
            _ => 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum AncientSpell {
    SmokeRush,
    ShadowRush,
    BloodRush,
    IceRush,
    SmokeBurst,
    ShadowBurst,
    BloodBurst,
    IceBurst,
    SmokeBlitz,
    ShadowBlitz,
    BloodBlitz,
    IceBlitz,
    SmokeBarrage,
    ShadowBarrage,
    BloodBarrage,
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
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ArceuusSpell {
    GhostlyGrasp,
    SkeletalGrasp,
    UndeadGrasp,
    InferiorDemonbane,
    SuperiorDemonbane,
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
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SpecialSpell {
    Invocate,
    Immolate,
}

impl SpecialSpell {
    pub fn max_hit(&self, player: &Player) -> u32 {
        match self {
            SpecialSpell::Invocate => min(1 + player.live_stats.magic * 44 / 99, 44),
            SpecialSpell::Immolate => min(1 + player.live_stats.magic * 58 / 99, 58),
        }
    }
}

fn magic_dart_max_hit(player: &Player) -> u32 {
    if player.is_wearing("Slayer's staff (e)", None) || player.boosts.on_task {
        13 + player.live_stats.magic / 6
    } else {
        10 + player.live_stats.magic / 10
    }
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

pub fn is_water_spell(spell: &Spell) -> bool {
    matches!(
        spell,
        Spell::Standard(StandardSpell::WaterStrike)
            | Spell::Standard(StandardSpell::WaterBolt)
            | Spell::Standard(StandardSpell::WaterWave)
            | Spell::Standard(StandardSpell::WaterBlast)
            | Spell::Standard(StandardSpell::WaterSurge)
    )
}

pub fn is_fire_spell(spell: &Spell) -> bool {
    matches!(
        spell,
        Spell::Standard(StandardSpell::FireStrike)
            | Spell::Standard(StandardSpell::FireBolt)
            | Spell::Standard(StandardSpell::FireWave)
            | Spell::Standard(StandardSpell::FireBlast)
            | Spell::Standard(StandardSpell::FireSurge)
    )
}

pub fn is_air_spell(spell: &Spell) -> bool {
    matches!(
        spell,
        Spell::Standard(StandardSpell::WindStrike)
            | Spell::Standard(StandardSpell::WindBolt)
            | Spell::Standard(StandardSpell::WindWave)
            | Spell::Standard(StandardSpell::WindBlast)
            | Spell::Standard(StandardSpell::WindSurge)
    )
}

pub fn is_earth_spell(spell: &Spell) -> bool {
    matches!(
        spell,
        Spell::Standard(StandardSpell::EarthBolt)
            | Spell::Standard(StandardSpell::EarthWave)
            | Spell::Standard(StandardSpell::EarthBlast)
            | Spell::Standard(StandardSpell::EarthSurge)
    )
}

pub fn is_smoke_spell(spell: &Spell) -> bool {
    matches!(
        spell,
        Spell::Ancient(AncientSpell::SmokeRush)
            | Spell::Ancient(AncientSpell::SmokeBurst)
            | Spell::Ancient(AncientSpell::SmokeBlitz)
            | Spell::Ancient(AncientSpell::SmokeBarrage)
    )
}

pub fn is_shadow_spell(spell: &Spell) -> bool {
    matches!(
        spell,
        Spell::Ancient(AncientSpell::ShadowRush)
            | Spell::Ancient(AncientSpell::ShadowBurst)
            | Spell::Ancient(AncientSpell::ShadowBlitz)
            | Spell::Ancient(AncientSpell::ShadowBarrage)
    )
}

pub fn is_blood_spell(spell: &Spell) -> bool {
    matches!(
        spell,
        Spell::Ancient(AncientSpell::BloodRush)
            | Spell::Ancient(AncientSpell::BloodBurst)
            | Spell::Ancient(AncientSpell::BloodBlitz)
            | Spell::Ancient(AncientSpell::BloodBarrage)
    )
}

pub fn is_ice_spell(spell: &Spell) -> bool {
    matches!(
        spell,
        Spell::Ancient(AncientSpell::IceRush)
            | Spell::Ancient(AncientSpell::IceBurst)
            | Spell::Ancient(AncientSpell::IceBlitz)
            | Spell::Ancient(AncientSpell::IceBarrage)
    )
}

pub fn is_demonbane_spell(spell: &Spell) -> bool {
    matches!(
        spell,
        Spell::Arceuus(ArceuusSpell::InferiorDemonbane)
            | Spell::Arceuus(ArceuusSpell::SuperiorDemonbane)
            | Spell::Arceuus(ArceuusSpell::DarkDemonbane)
    )
}

pub fn is_bind_spell(spell: &Spell) -> bool {
    is_ice_spell(spell)
        || is_grasp_spell(spell)
        || matches!(
            spell,
            Spell::Standard(StandardSpell::Bind)
                | Spell::Standard(StandardSpell::Snare)
                | Spell::Standard(StandardSpell::Entangle)
        )
}

pub fn is_grasp_spell(spell: &Spell) -> bool {
    matches!(
        spell,
        Spell::Arceuus(ArceuusSpell::GhostlyGrasp)
            | Spell::Arceuus(ArceuusSpell::SkeletalGrasp)
            | Spell::Arceuus(ArceuusSpell::UndeadGrasp)
    )
}
