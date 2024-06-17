// Adapted from the wiki DPS calc - credit to the wiki team

use crate::monster::Monster;

pub fn scale_monster_hp_only(monster: &mut Monster) {
    // Currently only used for Vardorvis, but this allows for future expansion
    if monster.info.name.contains("Vardorvis") {
        apply_vard_scaling(monster);
    }
}

fn apply_vard_scaling(monster: &mut Monster) {
    // Scale Vardorvis' strength and defence based on current hp
    let vard_ranges = VardNumbers::get(monster);
    let current_hp = monster.live_stats.hitpoints as i32;
    monster.live_stats.strength = lerp(
        current_hp,
        vard_ranges.max_hp,
        0,
        vard_ranges.str[0],
        vard_ranges.str[1],
    ) as u32;
    monster.live_stats.defence = lerp(
        current_hp,
        vard_ranges.max_hp,
        0,
        vard_ranges.def[0],
        vard_ranges.def[1],
    ) as u32;
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

fn lerp(
    current: i32,
    source_start: i32,
    source_end: i32,
    target_start: i32,
    target_end: i32,
) -> i32 {
    // Linear interpolation function
    target_start
        + (current - source_start) * (target_end - target_start) / (source_end - source_start)
}
