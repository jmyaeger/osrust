use crate::equipment::CombatType;
use crate::monster::Monster;
use crate::player::Player;
use rand::Rng;
use std::cmp::{max, min};

pub fn standard_attack(player: &Player, monster: &Monster, rng: &mut impl Rng) -> (u32, bool) {
    let combat_type = player.combat_type();
    let max_att_roll = player.att_rolls[&combat_type];
    let mut max_def_roll = monster.def_rolls[&combat_type];
    let max_hit = player.max_hits[&combat_type];
    let min_hit = if combat_type == CombatType::Magic
        && player.boosts.sunfire_runes
        && player.is_using_fire_spell()
    {
        1
    } else {
        0
    };

    if combat_type == CombatType::Magic
        && player.is_wearing("Brimstone ring")
        && rng.gen_range(0..4) == 0
    {
        max_def_roll = max_def_roll * 9 / 10;
    }

    let (mut damage, success) = base_attack(max_att_roll, max_def_roll, min_hit, max_hit, rng);
    if success {
        damage = max(1, damage - monster.bonuses.flat_armour)
    };

    (damage, success)
}

pub fn fang_attack(player: &Player, monster: &Monster, rng: &mut impl Rng) -> (u32, bool) {
    let combat_type = player.combat_type();
    let max_att_roll = player.att_rolls[&combat_type];
    let max_def_roll = monster.def_rolls[&combat_type];
    let true_max_hit = player.max_hits[&combat_type];
    let min_hit = true_max_hit * 15 / 100;
    let max_hit = true_max_hit - min_hit;

    let att_roll1 = accuracy_roll(max_att_roll, rng);
    let def_roll1 = defence_roll(max_def_roll, rng);

    let (damage, success) = if att_roll1 > def_roll1 {
        (damage_roll(min_hit, max_hit, rng), true)
    } else {
        let att_roll2 = accuracy_roll(max_att_roll, rng);
        let def_roll2 = if monster.is_toa_monster() {
            defence_roll(max_def_roll, rng)
        } else {
            def_roll1
        };
        if att_roll2 > def_roll2 {
            (damage_roll(min_hit, max_hit, rng), true)
        } else {
            (0, false)
        }
    };

    (damage, success)
}

pub fn ahrims_staff_attack(
    player: &Player,
    monster: &mut Monster,
    rng: &mut impl Rng,
) -> (u32, bool) {
    let combat_type = player.combat_type();
    if combat_type != CombatType::Magic || !player.set_effects.full_ahrims {
        return standard_attack(player, monster, rng);
    }

    let max_att_roll = player.att_rolls[&combat_type];
    let max_def_roll = monster.def_rolls[&combat_type];
    let max_hit = player.max_hits[&combat_type];

    let (mut damage, success) = base_attack(max_att_roll, max_def_roll, 0, max_hit, rng);

    if success && rng.gen_range(0..4) == 0 {
        monster.live_stats.strength = monster.live_stats.strength.saturating_sub(5);
    }

    if player.is_wearing("Amulet of the damned") && rng.gen_range(0..4) == 0 {
        damage = damage * 13 / 10;
    }

    (damage, success)
}

pub fn dharoks_greataxe_attack(
    player: &Player,
    monster: &Monster,
    rng: &mut impl Rng,
) -> (u32, bool) {
    let (mut damage, success) = standard_attack(player, monster, rng);
    if success && player.set_effects.full_dharoks {
        let max_hp = player.stats.hitpoints;
        let current_hp = player.live_stats.hitpoints;
        let dmg_mod = 10000 + (max_hp.saturating_sub(current_hp)) * max_hp;
        damage = damage * dmg_mod / 10000;
    }

    (damage, success)
}

pub fn veracs_flail_attack(player: &Player, monster: &Monster, rng: &mut impl Rng) -> (u32, bool) {
    let combat_type = player.combat_type();
    if player.set_effects.full_veracs && rng.gen_range(0..4) == 0 {
        (
            1 + damage_roll(1, player.max_hits[&combat_type] + 1, rng),
            true,
        )
    } else {
        standard_attack(player, monster, rng)
    }
}

pub fn karils_crossbow_attack(
    player: &Player,
    monster: &Monster,
    rng: &mut impl Rng,
) -> (u32, bool) {
    if player.set_effects.full_karils
        && player.is_wearing("Amulet of the damned")
        && rng.gen_range(0..4) == 0
    {
        let (hit1, success) = standard_attack(player, monster, rng);
        let hit2 = hit1 / 2;
        (hit1 + hit2, success)
    } else {
        standard_attack(player, monster, rng)
    }
}

pub fn guthans_warspear_attack(
    player: &mut Player,
    monster: &Monster,
    rng: &mut impl Rng,
) -> (u32, bool) {
    let (damage, success) = standard_attack(player, monster, rng);
    if player.set_effects.full_guthans && rng.gen_range(0..4) == 0 {
        let max_hp = if player.is_wearing("Amulet of the damned") {
            player.stats.hitpoints + 10
        } else {
            player.stats.hitpoints
        };
        player.live_stats.hitpoints = min(max_hp, player.live_stats.hitpoints + damage);
    }

    (damage, success)
}

pub fn torags_hammers_attack(
    player: &Player,
    monster: &Monster,
    rng: &mut impl Rng,
) -> (u32, bool) {
    let combat_type = player.combat_type();
    let max_hit = player.max_hits[&combat_type];
    let max_hit1 = max_hit / 2;
    let max_hit2 = max_hit - max_hit1;
    let max_att_roll = player.att_rolls[&combat_type];
    let max_def_roll = monster.def_rolls[&combat_type];

    let (damage1, success1) = base_attack(max_att_roll, max_def_roll, 0, max_hit1, rng);
    let (damage2, success2) = base_attack(max_att_roll, max_def_roll, 0, max_hit2, rng);

    (damage1 + damage2, success1 || success2)
}

pub fn sang_staff_attack(
    player: &mut Player,
    monster: &Monster,
    rng: &mut impl Rng,
) -> (u32, bool) {
    let (damage, success) = standard_attack(player, monster, rng);
    if rng.gen_range(0..6) == 0 {
        player.live_stats.hitpoints =
            min(player.stats.hitpoints, player.live_stats.hitpoints + damage);
    }

    (damage, success)
}

pub fn keris_attack(player: &Player, monster: &Monster, rng: &mut impl Rng) -> (u32, bool) {
    let (mut damage, success) = standard_attack(player, monster, rng);
    if monster.is_kalphite() && rng.gen_range(0..51) == 0 {
        damage *= 3;
    }

    (damage, success)
}

pub fn yellow_keris_attack(
    player: &mut Player,
    monster: &Monster,
    rng: &mut impl Rng,
) -> (u32, bool) {
    let combat_type = player.combat_type();
    let max_hit = player.max_hits[&combat_type];
    let mut max_att_roll = player.att_rolls[&combat_type];
    let max_def_roll = monster.def_rolls[&combat_type];

    if (monster.live_stats.hitpoints as f32) / (monster.stats.hitpoints as f32) < 0.25
        && monster.is_toa_monster()
    {
        max_att_roll = max_att_roll * 5 / 4;
    }

    let (damage, success) = base_attack(max_att_roll, max_def_roll, 0, max_hit, rng);

    if monster.live_stats.hitpoints.saturating_sub(damage) == 0 {
        let max_hp = player.stats.hitpoints + player.stats.hitpoints / 5;
        player.live_stats.hitpoints = min(max_hp, player.live_stats.hitpoints + 12);
        player.live_stats.prayer = player.live_stats.prayer.saturating_sub(5);
    }

    (damage, success)
}

fn base_attack(
    max_att_roll: u32,
    max_def_roll: u32,
    min_hit: u32,
    max_hit: u32,
    rng: &mut impl Rng,
) -> (u32, bool) {
    let att_roll = accuracy_roll(max_att_roll, rng);
    let def_roll = defence_roll(max_def_roll, rng);

    let success = att_roll > def_roll;
    let mut damage = 0;
    if success {
        damage = damage_roll(min_hit, max_hit, rng);
    }

    (damage, success)
}

fn accuracy_roll(max_att_roll: u32, rng: &mut impl Rng) -> u32 {
    rng.gen_range(0..=max_att_roll)
}

fn defence_roll(max_def_roll: u32, rng: &mut impl Rng) -> u32 {
    rng.gen_range(0..=max_def_roll)
}

fn damage_roll(min_hit: u32, max_hit: u32, rng: &mut impl Rng) -> u32 {
    rng.gen_range(min_hit..=max_hit)
}
