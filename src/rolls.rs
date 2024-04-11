use crate::constants::*;
use crate::equipment::{CombatStance, CombatType};
use crate::monster::Monster;
use crate::player::Player;
use std::collections::HashMap;

pub fn monster_def_rolls(monster: &Monster) -> HashMap<CombatType, i32> {
    fn calc_def_roll(def_lvl: i32, def_bonus: i32) -> i32 {
        (def_lvl + 9) * (def_bonus * 64)
    }

    let mut def_rolls = HashMap::new();
    for combat_type in &[
        (CombatType::Stab, monster.bonuses.defence.stab),
        (CombatType::Slash, monster.bonuses.defence.slash),
        (CombatType::Crush, monster.bonuses.defence.crush),
        (CombatType::Ranged, monster.bonuses.defence.ranged),
    ] {
        def_rolls.insert(
            combat_type.0,
            calc_def_roll(monster.live_stats.defence as i32, combat_type.1),
        );
    }
    if !MAGIC_DEF_EXCEPTIONS.contains(&monster.info.name.as_str()) {
        def_rolls.insert(
            CombatType::Magic,
            calc_def_roll(
                monster.live_stats.magic as i32,
                monster.bonuses.defence.magic,
            ),
        );
    } else {
        def_rolls.insert(
            CombatType::Magic,
            calc_def_roll(
                monster.live_stats.defence as i32,
                monster.bonuses.defence.magic,
            ),
        );
    }
    def_rolls
}

pub fn player_def_rolls(player: &Player) -> HashMap<CombatType, i32> {
    fn calc_def_roll(eff_lvl: i32, def_bonus: i32) -> i32 {
        eff_lvl * (def_bonus * 64)
    }

    let mut def_rolls = HashMap::new();
    let stance_bonus = match player.gear.weapon.combat_styles[&player.attrs.active_style].stance {
        CombatStance::Defensive | CombatStance::Longrange => 11,
        CombatStance::Controlled => 9,
        _ => 8,
    };
    let effective_level = player.live_stats.defence * player.prayers.defence + stance_bonus;
    let effective_magic = player.live_stats.magic * player.prayers.magic;
    for combat_type in &[
        (CombatType::Stab, player.bonuses.defence.stab),
        (CombatType::Slash, player.bonuses.defence.slash),
        (CombatType::Crush, player.bonuses.defence.crush),
        (CombatType::Ranged, player.bonuses.defence.ranged),
    ] {
        def_rolls.insert(
            combat_type.0,
            calc_def_roll(effective_level as i32, combat_type.1),
        );
    }
    def_rolls.insert(
        CombatType::Magic,
        calc_def_roll(
            (effective_magic * 7 / 10 + effective_level * 3 / 10) as i32,
            1,
        ),
    );
    def_rolls
}
