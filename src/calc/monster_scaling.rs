// Adapted from the wiki DPS calc - credit to the wiki team

use crate::calc::rolls::{calc_max_hit, monster_def_rolls};
use crate::types::monster::{AttackType, Monster};
use crate::utils::math::lerp;

pub fn scale_monster_hp_only(monster: &mut Monster) {
    // Currently only used for Vardorvis, but this allows for future expansion
    if monster.info.name.contains("Vardorvis") {
        apply_vard_scaling(monster);
    }
}

fn apply_vard_scaling(monster: &mut Monster) {
    // Scale Vardorvis' strength and defence based on current hp
    let vard_ranges = VardNumbers::get(monster);
    let current_hp = monster.stats.hitpoints.current as i32;
    monster.stats.strength.current = lerp(
        current_hp,
        vard_ranges.max_hp,
        0,
        vard_ranges.str[0],
        vard_ranges.str[1],
    ) as u32;
    monster.stats.defence.current = lerp(
        current_hp,
        vard_ranges.max_hp,
        0,
        vard_ranges.def[0],
        vard_ranges.def[1],
    ) as u32;

    // Recalculate Vardorvis' max hit (Note: must be initialized first)
    monster.max_hits.as_mut().and_then(|hits| {
        hits.iter_mut()
            .find(|h| h.style == AttackType::Slash)
            .map(|h| {
                h.value = calc_max_hit(
                    monster.stats.strength.current + 9,
                    monster.bonuses.strength.melee,
                )
            })
    });
    monster.def_rolls = monster_def_rolls(monster);
}

struct VardNumbers {
    pub max_hp: i32,
    // Strength and defence bounds for a given version of Vardorvis
    pub str: [i32; 2],
    pub def: [i32; 2],
}

impl VardNumbers {
    pub fn get(monster: &Monster) -> Self {
        match monster.info.version.as_deref() {
            Some("Quest") => Self {
                max_hp: 500,
                str: [210, 280],
                def: [180, 130],
            },
            Some("Awakened") => Self {
                max_hp: 1400,
                str: [391, 522],
                def: [268, 181],
            },
            _ => Self {
                max_hp: 700,
                str: [270, 360],
                def: [215, 145],
            },
        }
    }
}
