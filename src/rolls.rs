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

pub fn player_def_rolls(player: &Player) -> HashMap<CombatType, i32> {
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
    def_rolls
}

fn calc_roll(eff_lvl: i32, def_bonus: i32) -> i32 {
    eff_lvl * (def_bonus + 64)
}

pub fn player_att_rolls(player: &Player, monster: &Monster) -> HashMap<CombatType, i32> {
    let mut att_rolls = HashMap::new();
    let melee_rolls = player_melee_att_rolls(player, monster);
    let ranged_roll = player_ranged_att_roll(player, monster);
    let magic_roll = player_magic_att_roll(player, monster);
    att_rolls.insert(CombatType::Stab, melee_rolls[&CombatType::Stab]);
    att_rolls.insert(CombatType::Slash, melee_rolls[&CombatType::Slash]);
    att_rolls.insert(CombatType::Crush, melee_rolls[&CombatType::Crush]);
    att_rolls.insert(CombatType::Ranged, ranged_roll);
    att_rolls.insert(CombatType::Magic, magic_roll);

    att_rolls
}

pub fn player_max_hits(player: &Player, monster: &Monster) -> HashMap<CombatType, i32> {
    let mut max_hits = HashMap::new();
    let melee_max_hits = player_melee_max_hit(player, monster);
    let ranged_max_hit = player_ranged_max_hit(player, monster);
    let magic_max_hit = player_magic_max_hit(player, monster);
    max_hits.insert(CombatType::Stab, melee_max_hits[&CombatType::Stab]);
    max_hits.insert(CombatType::Slash, melee_max_hits[&CombatType::Slash]);
    max_hits.insert(CombatType::Crush, melee_max_hits[&CombatType::Crush]);
    max_hits.insert(CombatType::Ranged, ranged_max_hit);
    max_hits.insert(CombatType::Magic, magic_max_hit);
    max_hits
}

fn player_melee_att_rolls(player: &Player, monster: &Monster) -> HashMap<CombatType, i32> {
    let eff_att = calc_eff_att(player) as i32;
    let gear_bonus = melee_gear_bonus(player, monster);
    let inquisitor_boost = inquisitor_boost(player);
    let mut att_rolls = HashMap::new();

    let combat_types = [
        (CombatType::Stab, player.bonuses.attack.stab),
        (CombatType::Slash, player.bonuses.attack.slash),
        (CombatType::Crush, player.bonuses.attack.crush),
    ];

    for (combat_type, bonus) in combat_types.iter() {
        let factor = match combat_type {
            CombatType::Crush => gear_bonus * inquisitor_boost,
            _ => gear_bonus,
        };
        let roll = calc_roll(eff_att, *bonus);
        att_rolls.insert(
            *combat_type,
            roll * *factor.numer() as i32 / *factor.denom() as i32 / 1000,
        );
    }
    att_rolls
}

fn player_ranged_att_roll(player: &Player, monster: &Monster) -> i32 {
    let eff_att = calc_eff_ranged_att(player) as i32;
    let mut att_roll = calc_roll(eff_att, player.bonuses.attack.ranged);
    let crystal_bonus = crystal_bonus(player);
    att_roll = att_roll * (1000 + 2 * crystal_bonus) / 1000;
    let mut gear_bonus = ranged_gear_bonus(player, monster);

    if monster.info.attributes.contains(&Attribute::Draconic)
        && player.is_wearing("Dragon hunter crossbow")
    {
        gear_bonus += Ratio::new(3, 10);
    }

    att_roll = att_roll * *gear_bonus.numer() / *gear_bonus.denom();

    if player.is_wearing("Twisted bow") {
        let (tbow_acc_bonus, _) = monster.tbow_bonuses();
        att_roll = att_roll * tbow_acc_bonus / 100;
    }

    att_roll
}

fn player_magic_att_roll(player: &Player, monster: &Monster) -> i32 {
    0
}

fn player_melee_max_hit(player: &Player, monster: &Monster) -> HashMap<CombatType, i32> {
    let eff_str = calc_eff_str(player) as i32;
    let gear_bonus = melee_gear_bonus(player, monster);
    let obsidian_boost = obsidian_boost(player);
    let inquisitor_boost = inquisitor_boost(player);

    let base_max_hit = (eff_str * (player.bonuses.strength.melee + 64) + 320) / 640;
    let scaled_max_hit = base_max_hit * *gear_bonus.numer() / *gear_bonus.denom()
        + base_max_hit * *obsidian_boost.numer() / *obsidian_boost.denom();

    let mut max_hits = HashMap::new();
    max_hits.insert(CombatType::Stab, scaled_max_hit);
    max_hits.insert(CombatType::Slash, scaled_max_hit);
    max_hits.insert(CombatType::Crush, scaled_max_hit * inquisitor_boost / 1000);
    max_hits
}

fn player_ranged_max_hit(player: &Player, monster: &Monster) -> i32 {
    let eff_str = calc_eff_ranged_str(player) as i32;
    let str_bonus = if player.is_wearing("Eclipse atlatl") {
        player.bonuses.strength.melee
    } else {
        player.bonuses.strength.ranged
    };
    let mut max_hit = (eff_str * (str_bonus + 64) + 320) / 640;
    let crystal_bonus = crystal_bonus(player);
    max_hit = max_hit * (1000 + crystal_bonus) / 1000;

    let mut gear_bonus = ranged_gear_bonus(player, monster);

    if monster.info.attributes.contains(&Attribute::Draconic)
        && player.is_wearing("Dragon hunter crossbow")
    {
        gear_bonus += Ratio::new(1, 4);
    }

    max_hit = max_hit * *gear_bonus.numer() / *gear_bonus.denom();

    if player.is_wearing("Twisted bow") {
        let (_, tbow_dmg_bonus) = monster.tbow_bonuses();
        max_hit = max_hit * tbow_dmg_bonus / 100;
    }

    max_hit
}

fn player_magic_max_hit(player: &Player, monster: &Monster) -> i32 {
    0
}

fn calc_eff_att(player: &Player) -> u16 {
    let stance_bonus = match player.combat_stance() {
        CombatStance::Accurate => 11,
        CombatStance::Controlled => 9,
        _ => 8,
    };
    let att_pray_boost = player.prayers.attack;
    let mut eff_att = player.live_stats.attack * (100 + att_pray_boost) / 100 + stance_bonus;
    if player.set_effects.full_void | player.set_effects.full_elite_void {
        eff_att = eff_att * 11 / 10;
    }
    eff_att
}

fn calc_eff_str(player: &Player) -> u16 {
    let stance_bonus = match player.combat_stance() {
        CombatStance::Aggressive => 11,
        CombatStance::Controlled => 9,
        _ => 8,
    };
    let str_pray_boost = player.prayers.strength;
    let soulreaper_boost = player.boosts.soulreaper_stacks * player.live_stats.strength * 6 / 100;
    let mut eff_str =
        player.live_stats.strength * (100 + str_pray_boost) / 100 + soulreaper_boost + stance_bonus;
    if player.set_effects.full_void | player.set_effects.full_elite_void {
        eff_str = eff_str * 11 / 10;
    }
    eff_str
}

fn calc_eff_ranged_att(player: &Player) -> u16 {
    let stance_bonus = match player.combat_stance() {
        CombatStance::Accurate => 11,
        _ => 8,
    };

    let range_att_pray_boost = player.prayers.ranged_att;
    let mut eff_att = player.live_stats.ranged * (100 + range_att_pray_boost) / 100 + stance_bonus;
    if player.set_effects.full_void | player.set_effects.full_elite_void {
        eff_att = eff_att * 11 / 10;
    }
    eff_att
}

fn calc_eff_ranged_str(player: &Player) -> u16 {
    let stance_bonus = match player.combat_stance() {
        CombatStance::Accurate => 11,
        _ => 8,
    };

    let range_str_pray_boost = player.prayers.ranged_str;
    let str_level = if player.is_wearing("Eclipse atlatl") {
        player.live_stats.strength
    } else {
        player.live_stats.ranged
    };

    let mut eff_str = str_level * (100 + range_str_pray_boost) / 100 + stance_bonus;
    if player.set_effects.full_elite_void {
        eff_str = eff_str * 1125 / 1000;
    } else if player.set_effects.full_void {
        eff_str = eff_str * 11 / 10;
    }

    eff_str
}

fn melee_gear_bonus(player: &Player, monster: &Monster) -> Ratio<i32> {
    if player.is_wearing("Amulet of avarice") && monster.info.name.contains("Revenant") {
        if player.boosts.forinthry_surge {
            Ratio::new(135, 100)
        } else {
            Ratio::new(6, 5)
        }
    } else if monster.info.attributes.contains(&Attribute::Undead) && is_wearing_salve(player) {
        Ratio::new(7, 6)
    } else if is_wearing_salve_e(player) {
        Ratio::new(6, 5)
    } else if player.boosts.on_task && is_wearing_black_mask(player) {
        Ratio::new(7, 6)
    } else {
        Ratio::from_integer(1)
    }
}

fn obsidian_boost(player: &Player) -> Ratio<i32> {
    if player.set_effects.full_obsidian
        && (player.gear.weapon.name.contains("Tzhaar") || player.gear.weapon.name.contains("Toktz"))
        && player.is_using_melee()
    {
        Ratio::new(1, 10)
    } else {
        Ratio::from_integer(0)
    }
}

fn inquisitor_boost(player: &Player) -> i32 {
    let inquisitor_pieces = [
        &player.gear.head.name,
        &player.gear.body.name,
        &player.gear.legs.name,
    ]
    .iter()
    .filter(|slot| slot.contains("Inquisitor"))
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
    if player.is_wearing_any(vec![
        "Crystal bow",
        "Bow of faerdhinen",
        "Bow of faerdhinen (c)",
    ]) {
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
    if player.is_wearing("Amulet of avarice") && monster.info.name.contains("Revenant") {
        if player.boosts.forinthry_surge {
            gear_bonus = Ratio::new(135, 100)
        } else {
            gear_bonus = Ratio::new(6, 5)
        }
    } else if monster.info.attributes.contains(&Attribute::Undead)
        && player.is_wearing("Salve amulet (ei)")
    {
        gear_bonus = Ratio::new(6, 5)
    } else if player.is_wearing("Salve amulet (i)") {
        gear_bonus = Ratio::new(7, 6)
    } else if player.boosts.on_task && is_wearing_imbued_black_mask(player) {
        gear_bonus = Ratio::new(115, 100);
        if player.boosts.in_wilderness && player.is_wearing_any(vec!["Craw's bow", "Webweaver bow"])
        {
            gear_bonus += Ratio::new(1, 2);
        }
    }
    gear_bonus
}

fn is_wearing_black_mask(player: &Player) -> bool {
    player.is_wearing_any(vec![
        "Black mask",
        "Black mask (i)",
        "Slayer helmet",
        "Slayer helmet (i)",
    ])
}

fn is_wearing_imbued_black_mask(player: &Player) -> bool {
    player.is_wearing_any(vec!["Black mask (i)", "Slayer helmet (i)"])
}

fn is_wearing_salve(player: &Player) -> bool {
    player.is_wearing_any(vec!["Salve amulet", "Salve amulet (i)"])
}

fn is_wearing_salve_e(player: &Player) -> bool {
    player.is_wearing_any(vec!["Salve amulet (e)", "Salve amulet (ei)"])
}
