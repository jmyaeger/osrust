use crate::monster::Monster;

pub fn scale_monster_hp_only(monster: &mut Monster) {
    if monster.info.name.contains("Vardorvis") {
        apply_vard_scaling(monster);
    }
}

fn apply_vard_scaling(monster: &mut Monster) {
    let vard_ranges = VardNumbers::get(monster);
    let current_hp = monster.live_stats.hitpoints;
    monster.live_stats.strength = lerp(
        current_hp,
        vard_ranges.max_hp,
        0,
        vard_ranges.str[0],
        vard_ranges.str[1],
    );
    monster.live_stats.defence = lerp(
        current_hp,
        vard_ranges.max_hp,
        0,
        vard_ranges.def[0],
        vard_ranges.def[1],
    );
}

struct VardNumbers {
    pub max_hp: u32,
    pub str: [u32; 2],
    pub def: [u32; 2],
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
    current: u32,
    source_start: u32,
    source_end: u32,
    target_start: u32,
    target_end: u32,
) -> u32 {
    target_start
        + (current - source_start) * (target_end - target_start) / (source_end - source_start)
}