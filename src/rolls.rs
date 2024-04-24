use crate::equipment::{CombatStance, CombatType};
use crate::monster::{Attribute, Monster};
use crate::player::Player;
use crate::{constants::*, monster};
use num::rational::Ratio;
use std::collections::HashMap;

pub fn monster_def_rolls(monster: &Monster) -> HashMap<CombatType, i32> {
    let mut def_rolls = HashMap::new();
    for combat_type in &[
        (CombatType::Stab, monster.bonuses.defence.stab),
        (CombatType::Slash, monster.bonuses.defence.slash),
        (CombatType::Crush, monster.bonuses.defence.crush),
        (CombatType::Ranged, monster.bonuses.defence.ranged),
    ] {
        def_rolls.insert(
            combat_type.0,
            calc_roll(9 + monster.live_stats.defence as i32, combat_type.1),
        );
    }
    if !MAGIC_DEF_EXCEPTIONS.contains(&monster.info.name.as_str()) {
        def_rolls.insert(
            CombatType::Magic,
            calc_roll(
                9 + monster.live_stats.magic as i32,
                monster.bonuses.defence.magic,
            ),
        );
    } else {
        def_rolls.insert(
            CombatType::Magic,
            calc_roll(
                9 + monster.live_stats.defence as i32,
                monster.bonuses.defence.magic,
            ),
        );
    }
    def_rolls
}

pub fn calc_player_def_rolls(player: &mut Player) {
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
            calc_roll(effective_level as i32, combat_type.1),
        );
    }
    def_rolls.insert(
        CombatType::Magic,
        calc_roll(
            (effective_magic * 7 / 10 + effective_level * 3 / 10) as i32,
            1,
        ),
    );
    player.def_rolls = def_rolls;
}

fn calc_roll(eff_lvl: i32, bonus: i32) -> i32 {
    eff_lvl * (bonus + 64)
}

fn calc_max_hit(eff_lvl: u16, bonus: u16) -> u16 {
    (eff_lvl * (bonus + 64) + 320) / 640
}

pub fn calc_all_player_rolls(player: &mut Player, monster: &Monster) {
    calc_player_melee_rolls(player, monster);
    calc_player_ranged_rolls(player, monster);
    calc_player_magic_rolls(player, monster);
    calc_player_def_rolls(player);
}

pub fn calc_player_melee_rolls(player: &mut Player, monster: &Monster) {
    let (eff_att, eff_str) = calc_eff_melee_lvls(player);

    let gear_bonus = melee_gear_bonus(player, monster);
    let inquisitor_boost = inquisitor_boost(player);
    let obsidian_boost = obsidian_boost(player);

    let base_max_hit = calc_max_hit(eff_str, player.bonuses.strength.melee as u16);
    let scaled_max_hit = base_max_hit * *gear_bonus.numer() as u16 / *gear_bonus.denom() as u16
        + base_max_hit * *obsidian_boost.numer() as u16 / *obsidian_boost.denom() as u16;

    let mut att_rolls = HashMap::new();
    let mut max_hits = HashMap::new();

    let combat_types = [
        (CombatType::Stab, player.bonuses.attack.stab),
        (CombatType::Slash, player.bonuses.attack.slash),
        (CombatType::Crush, player.bonuses.attack.crush),
    ];

    for (combat_type, bonus) in combat_types.iter() {
        let mut att_roll = calc_roll(eff_att as i32, *bonus);
        att_roll = att_roll * *gear_bonus.numer() as i32 / *gear_bonus.denom() as i32;
        let mut max_hit = scaled_max_hit;

        if monster.is_dragon() && player.is_wearing("Dragon hunter lance") {
            att_roll = att_roll * 6 / 5;
            max_hit = max_hit * 6 / 5;
        }

        att_rolls.insert(*combat_type, att_roll);
        max_hits.insert(*combat_type, max_hit);
    }
    att_rolls.insert(
        CombatType::Crush,
        att_rolls[&CombatType::Crush] * inquisitor_boost / 1000,
    );
    max_hits.insert(
        CombatType::Crush,
        max_hits[&CombatType::Crush] * inquisitor_boost as u16 / 1000,
    );

    player.att_rolls = att_rolls;
    player.max_hits = max_hits;
}

pub fn calc_player_ranged_rolls(player: &mut Player, monster: &Monster) {
    let (eff_att, eff_str) = calc_eff_ranged_lvls(player);

    let crystal_bonus = crystal_bonus(player);
    let mut att_gear_bonus = ranged_gear_bonus(player, monster);
    let mut str_gear_bonus = att_gear_bonus;
    let str_bonus = if player.is_wearing("Eclipse atlatl") {
        player.bonuses.strength.melee
    } else {
        player.bonuses.strength.ranged
    };

    let mut att_roll = calc_roll(eff_att as i32, player.bonuses.attack.ranged);
    att_roll = att_roll * (1000 + 2 * crystal_bonus) / 1000;

    let mut max_hit = calc_max_hit(eff_str, str_bonus as u16);
    max_hit = max_hit * (1000 + crystal_bonus as u16) / 1000;

    if monster.is_dragon() && player.is_wearing("Dragon hunter crossbow") {
        att_gear_bonus += Ratio::new(3, 10);
        str_gear_bonus += Ratio::new(1, 4);
    }

    att_roll = att_roll * *att_gear_bonus.numer() / *att_gear_bonus.denom();
    max_hit = max_hit * *str_gear_bonus.numer() as u16 / *str_gear_bonus.denom() as u16;

    if player.is_wearing("Twisted bow") {
        let (tbow_acc_bonus, tbow_dmg_bonus) = monster.tbow_bonuses();
        att_roll = att_roll * tbow_acc_bonus / 100;
        max_hit = max_hit * tbow_dmg_bonus as u16 / 100;
    }
    player.att_rolls.insert(CombatType::Ranged, att_roll);
    player.max_hits.insert(CombatType::Ranged, max_hit);
}

pub fn calc_player_magic_rolls(player: &mut Player, monster: &Monster) {}

fn calc_eff_melee_lvls(player: &Player) -> (u16, u16) {
    let att_stance_bonus = match player.combat_stance() {
        CombatStance::Accurate => 11,
        CombatStance::Controlled => 9,
        _ => 8,
    };
    let str_stance_bonus = match player.combat_stance() {
        CombatStance::Aggressive => 11,
        CombatStance::Controlled => 9,
        _ => 8,
    };
    let att_pray_boost = player.prayers.attack;
    let str_pray_boost = player.prayers.strength;
    let soulreaper_boost = player.boosts.soulreaper_stacks * player.live_stats.strength * 6 / 100;
    let mut eff_att = player.live_stats.attack * (100 + att_pray_boost) / 100 + att_stance_bonus;
    let mut eff_str = player.live_stats.strength * (100 + str_pray_boost) / 100
        + soulreaper_boost
        + str_stance_bonus;
    if player.set_effects.full_void | player.set_effects.full_elite_void {
        eff_att = eff_att * 11 / 10;
        eff_str = eff_str * 11 / 10;
    }
    (eff_att, eff_str)
}

fn calc_eff_ranged_lvls(player: &Player) -> (u16, u16) {
    let stance_bonus = match player.combat_stance() {
        CombatStance::Accurate => 11,
        _ => 8,
    };

    let range_att_pray_boost = player.prayers.ranged_att;
    let range_str_pray_boost = player.prayers.ranged_str;
    let str_level = if player.is_wearing("Eclipse atlatl") {
        player.live_stats.strength
    } else {
        player.live_stats.ranged
    };

    let mut eff_att = player.live_stats.ranged * (100 + range_att_pray_boost) / 100 + stance_bonus;
    let mut eff_str = str_level * (100 + range_str_pray_boost) / 100 + stance_bonus;

    if player.set_effects.full_elite_void {
        eff_att = eff_att * 11 / 10;
        eff_str = eff_str * 1125 / 1000;
    } else if player.set_effects.full_void {
        eff_att = eff_att * 11 / 10;
        eff_str = eff_str * 11 / 10;
    }
    (eff_att, eff_str)
}

fn melee_gear_bonus(player: &Player, monster: &Monster) -> Ratio<i32> {
    if player.is_wearing("Amulet of avarice") && monster.is_revenant() {
        if player.boosts.forinthry_surge {
            Ratio::new(135, 100)
        } else {
            Ratio::new(6, 5)
        }
    } else if monster.is_undead() && player.is_wearing_salve() {
        Ratio::new(7, 6)
    } else if player.is_wearing_salve_e() {
        Ratio::new(6, 5)
    } else if player.boosts.on_task && player.is_wearing_black_mask() {
        Ratio::new(7, 6)
    } else {
        Ratio::from_integer(1)
    }
}

fn obsidian_boost(player: &Player) -> Ratio<i32> {
    if player.set_effects.full_obsidian
        && player.is_wearing_tzhaar_weapon()
        && player.is_using_melee()
    {
        Ratio::new(1, 10)
    } else {
        Ratio::from_integer(0)
    }
}

fn inquisitor_boost(player: &Player) -> i32 {
    let inquisitor_pieces = [&player.gear.head, &player.gear.body, &player.gear.legs]
        .iter()
        .filter_map(|slot| slot.as_ref())
        .filter(|armor| armor.name.contains("Inquisitor"))
        .count();

    let boost = if player.set_effects.full_inquisitor {
        25
    } else {
        5 * inquisitor_pieces as i32
    };

    1000 + boost
}

fn crystal_bonus(player: &Player) -> i32 {
    let mut crystal_bonus = 0;
    if player.is_wearing_crystal_bow() {
        if player.is_wearing("Crystal helm") {
            crystal_bonus += 25;
        }
        if player.is_wearing("Crystal body") {
            crystal_bonus += 75;
        }
        if player.is_wearing("Crystal legs") {
            crystal_bonus += 50;
        }
    }
    crystal_bonus
}

fn ranged_gear_bonus(player: &Player, monster: &Monster) -> Ratio<i32> {
    if player.is_wearing("Eclipse atlatl") {
        return melee_gear_bonus(player, monster);
    }
    let mut gear_bonus = Ratio::from_integer(1);
    if player.is_wearing("Amulet of avarice") && monster.is_revenant() {
        if player.boosts.forinthry_surge {
            gear_bonus = Ratio::new(135, 100)
        } else {
            gear_bonus = Ratio::new(6, 5)
        }
    } else if monster.is_undead() && player.is_wearing("Salve amulet (ei)") {
        gear_bonus = Ratio::new(6, 5)
    } else if player.is_wearing("Salve amulet (i)") {
        gear_bonus = Ratio::new(7, 6)
    } else if player.boosts.on_task && player.is_wearing_imbued_black_mask() {
        gear_bonus = Ratio::new(115, 100);
        if (player.boosts.in_wilderness || monster.is_in_wilderness())
            && player.is_wearing_wildy_bow()
        {
            gear_bonus += Ratio::new(1, 2);
        }
    }
    gear_bonus
}
