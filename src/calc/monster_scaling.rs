// Adapted from the wiki DPS calc - credit to the wiki team

use crate::calc::rolls::{calc_max_hit, monster_def_rolls};
use crate::types::monster::{HpScalingEntry, HpScalingTable, Monster};
use crate::utils::math::lerp;

/// Scales monster stats based on current HP if the monster has HP-based scaling.
#[inline]
pub fn scale_monster_hp_only(monster: &mut Monster) {
    if monster.hp_scaling_table.is_some() {
        let hp = monster.stats.hitpoints.current as usize;
        let entry = &monster.hp_scaling_table.as_ref().unwrap().get(hp);

        monster.stats.strength.current = entry.strength;
        monster.stats.defence.current = entry.defence;

        if let Some(hits) = &mut monster.max_hits {
            hits[0].value = entry.max_hit;
        }

        monster.def_rolls = entry.def_rolls;
    } else if monster.info.name.as_str() == "Vardorvis" {
        // let scaling_table = build_vard_scaling_table(monster);
        // monster.hp_scaling_table = Some(scaling_table);
        apply_vard_scaling(monster);
    }
}

#[inline]
pub fn apply_vard_scaling(monster: &mut Monster) {
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
    if let Some(hits) = &mut monster.max_hits {
        hits[0].value = calc_max_hit(
            monster.stats.strength.current + 9,
            monster.bonuses.strength.melee,
        );
    }

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

/// Build a precomputed HP scaling table for Vardorvis.
pub fn build_vard_scaling_table(monster: &Monster) -> HpScalingTable {
    let (max_hp, str_bounds, def_bounds): (i32, [i32; 2], [i32; 2]) =
        match monster.info.version.as_deref() {
            Some("Quest") => (500, [210, 280], [180, 130]),
            Some("Awakened") => (1400, [391, 522], [268, 181]),
            _ => (700, [270, 360], [215, 145]),
        };

    let str_bonus = monster.bonuses.strength.melee;

    let mut table = Vec::with_capacity(max_hp as usize + 1);

    // Create a temporary copy to compute def_rolls at each HP level
    let mut temp_monster = monster.clone();

    for hp in 0..=max_hp {
        let strength = lerp(hp, max_hp, 0, str_bounds[0], str_bounds[1]) as u32;
        let defence = lerp(hp, max_hp, 0, def_bounds[0], def_bounds[1]) as u32;
        let max_hit = calc_max_hit(strength + 9, str_bonus);

        // Set the defence level and compute def_rolls
        temp_monster.stats.defence.current = defence;
        let def_rolls = monster_def_rolls(&temp_monster);

        table.push(HpScalingEntry {
            strength,
            defence,
            max_hit,
            def_rolls,
        });
    }

    HpScalingTable::new(table)
}
