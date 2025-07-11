// Adapted from the wiki DPS calc - credit to the wiki team

use crate::calc::hit_dist::flat_limit_transformer;
use crate::calc::hit_dist::{
    AttackDistribution, HitDistribution, Hitsplat, TransformOpts, WeightedHit,
    capped_reroll_transformer, division_transformer, flat_add_transformer, linear_min_transformer,
    multiply_transformer,
};
use crate::calc::monster_scaling;
use crate::calc::rolls::{calc_active_player_rolls, get_demonbane_factor, monster_def_rolls};
use crate::constants::*;
use crate::dists;
use crate::dists::bolts::{self, BoltContext};
use crate::types::equipment::{CombatStance, CombatType};
use crate::types::monster::Monster;
use crate::types::player::Player;
use crate::types::spells::{Spell, StandardSpell};
use crate::utils::math::{Fraction, lerp};
use std::cmp::{max, min};
use std::collections::HashMap;

fn get_normal_accuracy(player: &Player, monster: &Monster, using_spec: bool) -> f64 {
    // Calculate theoretical hit chance for most weapons
    let combat_type = player.combat_type();
    let mut max_att_roll = player.att_rolls.get(combat_type);

    if using_spec {
        let att_roll_factor = match &player.gear.weapon.name as &str {
            "Saradomin godsword" | "Bandos godsword" | "Zamorak godsword" | "Armadyl godsword"
            | "Zaryte crossbow" | "Webweaver bow" | "Toxic blowpipe" | "Ancient godsword"
            | "Brine sabre" | "Barrelchest anchor" => Fraction::new(2, 1),
            "Accursed sceptre" | "Accursed sceptre (a)" | "Volatile nightmare staff" => {
                Fraction::new(3, 2)
            }
            "Dragon dagger" => Fraction::new(115, 100),
            "Abyssal dagger" | "Abyssal whip" | "Dragon mace" | "Dragon sword" | "Elder maul" => {
                Fraction::new(5, 4)
            }
            "Soulreaper axe" => {
                Fraction::new(100 + 6 * player.boosts.soulreaper_stacks as i32, 100)
            }
            "Magic shortbow" | "Magic shortbow (i)" => Fraction::new(10, 7),
            "Heavy ballista" | "Light ballista" => Fraction::new(5, 4),
            _ => Fraction::new(1, 1),
        };
        max_att_roll = att_roll_factor.multiply_to_int(max_att_roll);
    }

    if player.is_wearing("Keris partisan of the sun", None)
        && TOA_MONSTERS.contains(&monster.info.id.unwrap_or(0))
        && monster.stats.hitpoints.current < monster.stats.hitpoints.base / 4
    {
        max_att_roll = max_att_roll * 5 / 4;
    }

    let mut def_roll = if using_spec {
        if STAB_SPEC_WEAPONS.contains(&player.gear.weapon.name.as_str()) {
            monster.def_rolls.get(CombatType::Stab)
        } else if SLASH_SPEC_WEAPONS.contains(&player.gear.weapon.name.as_str()) {
            monster.def_rolls.get(CombatType::Slash)
        } else if CRUSH_SPEC_WEAPONS.contains(&player.gear.weapon.name.as_str()) {
            monster.def_rolls.get(CombatType::Crush)
        } else if MAGIC_SPEC_WEAPONS.contains(&player.gear.weapon.name.as_str()) {
            monster.def_rolls.get(CombatType::Magic)
        } else {
            monster.def_rolls.get(combat_type)
        }
    } else {
        monster.def_rolls.get(combat_type)
    };

    let std_roll = |attack: i32, defence: i32| -> f64 {
        if attack > defence {
            1.0 - ((defence as f64 + 2.0) / (2.0 * (attack as f64 + 1.0)))
        } else {
            attack as f64 / (2.0 * (defence as f64 + 1.0))
        }
    };

    if max_att_roll < 0 {
        max_att_roll = min(0, max_att_roll + 2);
    }

    if def_roll < 0 {
        def_roll = min(0, def_roll + 2);
    }

    match (max_att_roll < 0, def_roll < 0) {
        (false, false) => std_roll(max_att_roll, def_roll),
        (false, true) => 1.0 - 1.0 / (-def_roll as f64 + 1.0) / (max_att_roll as f64 + 1.0),
        (true, false) => 0.0,
        (true, true) => std_roll(-max_att_roll, -def_roll),
    }
}

fn get_fang_accuracy(player: &Player, monster: &Monster, using_spec: bool) -> f64 {
    // Calculate theoretical hit chance for Osmumten's fang outside of ToA
    let combat_type = player.combat_type();
    let mut max_att_roll = player.att_rolls.get(combat_type);

    if using_spec {
        max_att_roll = max_att_roll * 3 / 2;
    }

    let mut def_roll = monster.def_rolls.get(combat_type);

    let std_roll = |attack: i32, defence: i32| -> f64 {
        if attack > defence {
            1.0 - (defence as f64 + 2.0) * (2.0 * defence as f64 + 3.0)
                / (attack as f64 + 1.0)
                / (attack as f64 + 1.0)
                / 6.0
        } else {
            attack as f64 * (4.0 * attack as f64 + 5.0)
                / 6.0
                / (defence as f64 + 1.0)
                / (attack as f64 + 1.0)
        }
    };

    let rv_roll = |attack: i32, defence: i32| -> f64 {
        if attack < defence {
            attack as f64 * (defence as f64 * 6.0 - 2.0 * attack as f64 + 5.0)
                / 6.0
                / (defence as f64 + 1.0)
                / (defence as f64 + 1.0)
        } else {
            1.0 - (defence as f64 + 2.0) * (2.0 * defence as f64 + 3.0)
                / 6.0
                / (defence as f64 + 1.0)
                / (attack as f64 + 1.0)
        }
    };

    if max_att_roll < 0 {
        max_att_roll = min(0, max_att_roll + 2);
    }

    if def_roll < 0 {
        def_roll = min(0, def_roll + 2);
    }

    match (max_att_roll < 0, def_roll < 0) {
        (false, false) => std_roll(max_att_roll, def_roll),
        (false, true) => 1.0 - 1.0 / (-def_roll as f64 + 1.0) / (max_att_roll as f64 + 1.0),
        (true, false) => 0.0,
        (true, true) => rv_roll(-def_roll, -max_att_roll),
    }
}

fn get_hit_chance(player: &Player, monster: &Monster, using_spec: bool) -> f64 {
    // Always accurate in these cases
    if (monster.info.name.contains("Verzik")
        && monster.matches_version("Phase 1")
        && player.is_wearing("Dawnbringer", None))
        || (monster.info.name.as_str() == "Giant rat (Scurrius)"
            && player.combat_stance() != CombatStance::ManualCast)
        || (using_spec && player.is_wearing_any(ALWAYS_HITS_SPEC))
        || P2_WARDEN_IDS.contains(&monster.info.id.unwrap_or_default())
        || GUARANTEED_ACCURACY_MONSTERS.contains(&monster.info.id.unwrap_or_default())
    {
        return 1.0;
    }

    let mut hit_chance = get_normal_accuracy(player, monster, using_spec);

    if player.is_wearing("Osmumten's fang", None) && player.combat_type() == CombatType::Stab {
        if monster.is_toa_monster() {
            hit_chance = 1.0 - (1.0 - hit_chance) * (1.0 - hit_chance);
        } else {
            hit_chance = get_fang_accuracy(player, monster, using_spec);
        }
    }

    if player.combat_type() == CombatType::Magic && player.is_wearing("Brimstone ring", None) {
        let mut monster_copy = monster.clone();
        let def_roll = monster.def_rolls.get(CombatType::Magic) * 9 / 10;
        monster_copy.def_rolls.set(CombatType::Magic, def_roll);
        hit_chance =
            hit_chance * 0.75 + get_normal_accuracy(player, &monster_copy, using_spec) * 0.25;
    }

    hit_chance
}

fn get_dot_expected(player: &Player, monster: &Monster, using_spec: bool) -> f64 {
    if using_spec {
        if player.is_wearing("Burning claws", None) {
            burning_claw_dot(player, monster)
        } else if player.is_wearing("Scorching bow", None) {
            if monster.is_demon() { 5.0 } else { 1.0 }
        } else if player.is_wearing("Ancient godsword", None) {
            let accuracy = get_hit_chance(player, monster, true);
            accuracy * 25.0
        } else {
            0.0
        }
    } else {
        0.0
    }
}

fn get_dot_max(player: &Player, monster: &Monster, using_spec: bool) -> u32 {
    if using_spec {
        if player.is_wearing("Burning claws", None) {
            29
        } else if player.is_wearing("Scorching bow", None) {
            if monster.is_demon() { 5 } else { 1 }
        } else {
            0
        }
    } else {
        0
    }
}

fn burning_claw_dot(player: &Player, monster: &Monster) -> f64 {
    let mut dot = 0.0;
    let accuracy = get_hit_chance(player, monster, true);
    for acc_roll in 0..3 {
        let prev_rolls_fail = (1.0 - accuracy).powi(acc_roll);
        let this_roll_hits = prev_rolls_fail * accuracy;
        dot += this_roll_hits * BURN_EXPECTED[acc_roll as usize];
    }

    dot
}

pub fn get_distribution(
    player: &Player,
    monster: &Monster,
    using_spec: bool,
) -> AttackDistribution {
    // Get the attack distribution for the given player and monster
    let acc = get_hit_chance(player, monster, using_spec);
    let combat_type = player.combat_type();
    let (mut min_hit, max_hit) = if using_spec {
        get_spec_min_max_hit(player, monster)
    } else if P2_WARDEN_IDS.contains(&monster.info.id.unwrap_or_default()) {
        get_wardens_p2_min_max(player, monster)
    } else {
        (0, player.max_hits.get(combat_type))
    };

    // Players will always hit at least half their max against sire vents
    if monster.info.name == "Respiratory system" {
        min_hit = max_hit / 2;
    }

    let standard_hit_dist = HitDistribution::linear(acc, min_hit, max_hit);
    let mut dist = AttackDistribution::new(vec![standard_hit_dist.clone()]);
    let mut accurate_zero_applicable = true;

    // Check if the monster always dies in one hit
    if ONE_HIT_MONSTERS.contains(&monster.info.id.unwrap_or(0)) {
        return AttackDistribution::new(vec![HitDistribution::single(
            1.0,
            vec![Hitsplat::new(monster.stats.hitpoints.base, true)],
        )]);
    }

    // Sire vents always die in one hit if the player is using a demonbane weapon
    if monster.info.name == "Respiratory system" && player.is_using_demonbane() {
        return AttackDistribution::new(vec![HitDistribution::single(
            acc,
            vec![Hitsplat::new(monster.stats.hitpoints.current, true)],
        )]);
    }

    // Check if the monster always takes the maximum hit for the current combat type
    if player.combat_type() == CombatType::Magic
        && ALWAYS_MAX_HIT_MAGIC.contains(&monster.info.id.unwrap_or(0))
        || player.is_using_melee() && ALWAYS_MAX_HIT_MELEE.contains(&monster.info.id.unwrap_or(0))
        || player.is_using_ranged() && ALWAYS_MAX_HIT_RANGED.contains(&monster.info.id.unwrap_or(0))
    {
        return AttackDistribution::new(vec![HitDistribution::single(
            1.0,
            vec![Hitsplat::new(max_hit, true)],
        )]);
    }

    // Add a minimum hit if the player is using sunfire runes and a fire spell
    if player.boosts.sunfire.active && player.is_using_fire_spell() {
        dist = AttackDistribution::new(vec![HitDistribution::linear(
            acc,
            player.boosts.sunfire.min_hit,
            max_hit,
        )]);
    }

    // Clamp damage range between 15-85% if using fang
    if player.is_using_melee() && player.is_wearing("Osmumten's fang", None) {
        let min_hit = player.max_hits.get(CombatType::Stab) * 3 / 20;
        dist = AttackDistribution::new(vec![HitDistribution::linear(
            acc,
            min_hit,
            max_hit - min_hit,
        )]);
    }

    // Gadderhammer/shade distribution
    if player.is_using_melee() && player.is_wearing("Gadderhammer", None) && monster.is_shade() {
        let hits1 = standard_hit_dist
            .clone()
            .scale_probability(0.95)
            .scale_damage(Fraction::new(5, 4));
        let hits2 = standard_hit_dist
            .clone()
            .scale_probability(0.05)
            .scale_damage(Fraction::from_integer(2));
        let mut combined_hits = Vec::new();
        combined_hits.extend(hits1.hits);
        combined_hits.extend(hits2.hits);

        dist = AttackDistribution::new(vec![HitDistribution::new(combined_hits)]);
    }

    // Claw specs
    if using_spec {
        if player.is_wearing("Dragon claws", None) {
            // Dragon claw specs do not get the accurate 0 -> 1 transform
            accurate_zero_applicable = false;
            dist = dists::claws::dragon_claw_dist(acc, max_hit);
        } else if player.is_wearing("Burning claws", None) {
            dist = dists::claws::burning_claw_spec(acc, max_hit);
        }
    }

    // Halberd specs
    if using_spec
        && player.is_wearing_any(vec![("Dragon halberd", None), ("Crystal halberd", None)])
    {
        // Second hit has 75% accuracy
        let second_hit_att_roll = player.att_rolls.get(player.combat_type()) * 3 / 4;
        let mut player_copy = player.clone();
        player_copy
            .att_rolls
            .set(player.combat_type(), second_hit_att_roll);
        calc_active_player_rolls(&mut player_copy, monster);

        let second_hit_acc = get_hit_chance(&player_copy, monster, using_spec);
        dist = AttackDistribution::new(vec![
            standard_hit_dist.clone(),
            HitDistribution::linear(second_hit_acc, min_hit, max_hit),
        ])
    }

    // Simple multi-hit specs
    if using_spec {
        let mut hit_count = 1;
        if player.is_wearing_any_version("Dragon dagger")
            || player.is_wearing_any_version("Dragon knife")
            || player.is_wearing_any(MAGIC_SHORTBOWS)
        {
            hit_count = 2;
        } else if player.is_wearing("Webweaver bow", None) {
            hit_count = 4;
        }

        dist = AttackDistribution::default();
        for _ in 0..hit_count {
            dist.add_dist(standard_hit_dist.clone());
        }
    }

    // Abyssal dagger spec
    if using_spec && player.is_wearing_any_version("Abyssal dagger") {
        let second_hit = HitDistribution::linear(1.0, min_hit, max_hit);
        dist = dist.transform(
            &|h| HitDistribution::new(vec![WeightedHit::new(1.0, vec![*h])]).zip(&second_hit),
            &TransformOpts {
                transform_inaccurate: false,
            },
        );
    }

    // Saradomin sword spec
    if using_spec && player.is_wearing("Saradomin sword", None) {
        let magic_hit = HitDistribution::linear(1.0, 1, 16);
        if !IMMUNE_TO_MAGIC_MONSTERS.contains(&monster.info.id.unwrap_or(0)) {
            dist = dist.transform(
                &|h| HitDistribution::new(vec![WeightedHit::new(1.0, vec![*h])]).zip(&magic_hit),
                &TransformOpts {
                    transform_inaccurate: false,
                },
            );
        }
    }

    // Verac's set effect distribution
    if player.is_using_melee() && player.set_effects.full_veracs {
        let hits1 = standard_hit_dist.clone().scale_probability(0.75).hits;
        let hits2 = HitDistribution::linear(1.0, 1, max_hit + 1)
            .scale_probability(0.25)
            .hits;

        dist = dist_from_multiple_hits(vec![hits1, hits2]);
    }

    // Karil's set effect + amulet of the damned distribution
    if player.is_using_ranged()
        && player.set_effects.full_karils
        && player.is_wearing_any_version("Amulet of the damned")
    {
        let hits1 = standard_hit_dist.clone().scale_probability(0.75).hits;
        let hits2 = standard_hit_dist.clone().hits;
        let hits2 = hits2
            .iter()
            .map(|h| {
                WeightedHit::new(
                    h.probability * 0.25,
                    vec![
                        h.hitsplats[0],
                        Hitsplat::new(h.hitsplats[0].damage / 2, h.hitsplats[0].accurate),
                    ],
                )
            })
            .collect();

        dist = dist_from_multiple_hits(vec![hits1, hits2]);
    }

    // Scythe distribution
    if player.is_using_melee() && player.is_wearing_any_version("Scythe of vitur") {
        let mut hits: Vec<HitDistribution> = Vec::new();

        for i in 0..monster.info.size.clamp(1, 3) {
            hits.push(HitDistribution::linear(
                acc,
                0,
                max_hit / (num::pow(2, i as usize)),
            ));
        }
        dist = AttackDistribution::new(hits);
    }

    // Dual macuahuitl distribution (without set effect)
    if player.is_using_melee() && player.is_wearing("Dual macuahuitl", None) {
        let half_max = max_hit / 2;
        let first_hit = AttackDistribution::new(vec![HitDistribution::linear(acc, 0, half_max)]);
        let second_hit = HitDistribution::linear(acc, 0, max_hit - half_max);
        dist = first_hit.transform(
            &|h| {
                if h.accurate {
                    HitDistribution::new(vec![WeightedHit::new(1.0, vec![*h])]).zip(&second_hit)
                } else {
                    HitDistribution::new(vec![WeightedHit::new(
                        1.0,
                        vec![*h, Hitsplat::inaccurate()],
                    )])
                }
            },
            &TransformOpts {
                transform_inaccurate: false,
            },
        );
    }

    // Double-hitting weapon distribution (Torag's hammers/sulphur blades)
    if player.is_using_melee() && player.is_wearing_any(DOUBLE_HIT_WEAPONS) {
        let half_max = max_hit / 2;
        let first_hit = HitDistribution::linear(acc, 0, half_max);
        let second_hit = HitDistribution::linear(acc, 0, max_hit - half_max);

        dist = AttackDistribution::new(vec![first_hit, second_hit]);
    }

    // Tonalztics distribution
    if player.is_using_ranged() && player.gear.weapon.name.contains("Tonalztics") {
        let three_fourths = max_hit * 3 / 4;
        let first_hit = HitDistribution::linear(acc, 0, three_fourths);
        if player.gear.weapon.matches_version("Uncharged") {
            dist = AttackDistribution::new(vec![first_hit]);
        } else {
            let second_hit = HitDistribution::linear(acc, 0, three_fourths);
            if !using_spec {
                dist = AttackDistribution::new(vec![first_hit, second_hit]);
            } else {
                // Defence drain from first hit affects accuracy of second hit
                let mut monster_copy = monster.clone();

                // Drains defence by 10% of the magic level
                let def_drain = monster_copy.stats.magic.base / 10;
                monster_copy.stats.defence.drain(def_drain);
                monster_copy.def_rolls = monster_def_rolls(&monster_copy);

                let second_hit_acc = get_hit_chance(player, &monster_copy, using_spec);
                let lowered_second_hit = HitDistribution::linear(second_hit_acc, 0, three_fourths);
                dist = dist.transform(
                    &|h| {
                        let first_hit_dist =
                            HitDistribution::single(1.0, vec![Hitsplat::new(h.damage, true)]);
                        let second_hit_dist = if h.accurate {
                            &lowered_second_hit
                        } else {
                            &second_hit
                        };
                        first_hit_dist.zip(second_hit_dist)
                    },
                    &TransformOpts::default(),
                );
            }
        }
    }

    // Keris distribution against kalphites
    if player.is_using_melee() && player.is_wearing_keris() && monster.is_kalphite() {
        let hits1 = standard_hit_dist
            .clone()
            .scale_probability(50.0 / 51.0)
            .hits;
        let hits2 = standard_hit_dist
            .clone()
            .scale_probability(1.0 / 51.0)
            .scale_damage(Fraction::from_integer(3))
            .hits;

        dist = dist_from_multiple_hits(vec![hits1, hits2]);
    }

    // Guardians (CoX) distribution
    if player.is_using_melee()
        && monster.info.name.contains("Guardian (Chambers")
        && player.gear.weapon.name.contains("pickaxe")
    {
        let pick_bonus = PICKAXE_BONUSES
            .iter()
            .find(|b| b.0 == player.gear.weapon.name)
            .unwrap_or_else(|| panic!("No pickaxe bonus for {}", player.gear.weapon.name))
            .1;

        let numerator = 50 + player.stats.mining.current + pick_bonus;
        let denominator = 150;

        dist = dist.transform(
            &multiply_transformer(Fraction::new(numerator as i32, denominator), 0),
            &TransformOpts::default(),
        );
    }

    // Fire spell against ice demon distribution
    if monster.info.name.contains("Ice demon") && player.is_using_fire_spell()
        || player.attrs.spell == Some(Spell::Standard(StandardSpell::FlamesOfZamorak))
    {
        dist = dist.scale_damage(Fraction::new(3, 2));
    }

    // Mark of darkness + demonbane distribution
    if player.boosts.mark_of_darkness && player.is_using_demonbane_spell() && monster.is_demon() {
        let damage_boost = if player.is_wearing("Purging staff", None) {
            50
        } else {
            25
        };
        dist = dist.transform(
            &|h| {
                HitDistribution::single(
                    1.0,
                    vec![Hitsplat::new(
                        h.damage
                            + get_demonbane_factor(100, monster)
                                .multiply_to_int(h.damage * damage_boost / 100),
                        h.accurate,
                    )],
                )
            },
            &TransformOpts::default(),
        );
    }

    // Full Ahrim's + amulet of the damned distribution
    if player.combat_type() == CombatType::Magic
        && player.set_effects.full_ahrims
        && player.is_wearing_any_version("Amulet of the damned")
    {
        dist = dist.transform(
            &|h| {
                HitDistribution::new(vec![
                    WeightedHit::new(0.75, vec![*h]),
                    WeightedHit::new(0.25, vec![Hitsplat::new(h.damage * 13 / 10, h.accurate)]),
                ])
            },
            &TransformOpts::default(),
        );
    }

    // Vampyre stuff
    if let Some(tier) = monster.vampyre_tier() {
        if player.is_wearing("Efaritay's aid", None) {
            dist = dist.scale_damage(Fraction::new(11, 10));
        }
        match (
            player.gear.weapon.name.as_str(),
            player.is_wearing_silver_weapon(),
            tier,
        ) {
            ("Blisterwood flail", _, _) => {
                dist = dist.scale_damage(Fraction::new(5, 4));
            }
            ("Blisterwood sickle", _, _) => {
                dist = dist.scale_damage(Fraction::new(23, 20));
            }
            ("Ivandis flail", _, _) => {
                dist = dist.scale_damage(Fraction::new(6, 5));
            }
            ("Rod of ivandis", _, 1 | 2) => {
                dist = dist.scale_damage(Fraction::new(11, 10));
            }
            (_, true, 1) => {
                dist = dist.scale_damage(Fraction::new(11, 10));
            }
            (_, _, _) => {}
        }
    }

    if player.is_using_ranged() && player.is_wearing("Dark bow", None) {
        dist = AttackDistribution::new(vec![standard_hit_dist.clone(), standard_hit_dist.clone()]);
        if using_spec {
            dist = dist.transform(
                &flat_limit_transformer(48, min_hit),
                &TransformOpts::default(),
            );
        }
    }

    let bolt_context = BoltContext::new(
        player.stats.ranged.current,
        max_hit,
        player.is_wearing("Zaryte crossbow", None),
        using_spec,
        player.boosts.kandarin_diary,
        monster,
    );

    // Enchanted bolt distributions
    if player.is_using_ranged() && player.is_using_crossbow() {
        // Opal bolts
        if player.is_wearing_any(OPAL_BOLTS) {
            dist = dist.transform(&bolts::opal_bolts(&bolt_context), &TransformOpts::default());
        }

        // Pearl bolts
        if player.is_wearing_any(PEARL_BOLTS) {
            dist = dist.transform(
                &bolts::pearl_bolts(&bolt_context),
                &TransformOpts::default(),
            );
        }

        // Diamond bolts
        if player.is_wearing_any(DIAMOND_BOLTS) {
            dist = dist.transform(
                &bolts::diamond_bolts(&bolt_context),
                &TransformOpts::default(),
            );
        }

        // Dragonstone bolts
        if player.is_wearing_any(DRAGONSTONE_BOLTS) && (!monster.is_fiery() || !monster.is_dragon())
        {
            dist = dist.transform(
                &bolts::dragonstone_bolts(&bolt_context),
                &TransformOpts::default(),
            );
        }

        // Onyx bolts
        if player.is_wearing_any(ONYX_BOLTS) {
            dist = dist.transform(&bolts::onyx_bolts(&bolt_context), &TransformOpts::default());
        }
    }

    // Apply corp transform before ruby bolt procs
    if monster.info.name.as_str() == "Corporeal Beast" && !player.is_using_corpbane_weapon() {
        dist = dist.transform(&division_transformer(2, 0), &TransformOpts::default());
    }

    // Ruby bolts
    if player.is_using_ranged() && player.is_using_crossbow() && player.is_wearing_any(RUBY_BOLTS) {
        dist = dist.transform(&bolts::ruby_bolts(&bolt_context), &TransformOpts::default());
    }

    // Berserker necklace + obby weapon distribution (tested, confirmed post-roll)
    if player.is_using_melee()
        && player.is_wearing("Berserker necklace", None)
        && player.is_wearing_tzhaar_weapon()
    {
        dist = dist.scale_damage(Fraction::new(6, 5));
    }

    // Dharok's set effect distribution
    if player.is_using_melee() && player.set_effects.full_dharoks {
        let full_hp = player.stats.hitpoints.base;
        let current_hp = player.stats.hitpoints.current;
        let numerator = 10000 + (full_hp - current_hp) as i32 * full_hp as i32;
        dist = dist.scale_damage(Fraction::new(numerator, 10000));
    }

    // Accurate 0 -> 1 is either overwritten by ruby bolts or divided back down to 0
    if accurate_zero_applicable
        && (monster.info.name.as_str() != "Corporeal Beast" || player.is_using_corpbane_weapon())
    {
        dist = dist.transform(
            &|h| HitDistribution::single(1.0, vec![Hitsplat::new(max(h.damage, 1), h.accurate)]),
            &TransformOpts {
                transform_inaccurate: false,
            },
        );
    }

    if player.gets_second_twinflame_hit() {
        dist = dist.transform(
            &|h| {
                HitDistribution::single(
                    1.0,
                    vec![
                        Hitsplat::new(h.damage, h.accurate),
                        Hitsplat::new(h.damage * 2 / 5, h.accurate),
                    ],
                )
            },
            &TransformOpts::default(),
        )
    }

    apply_limiters(dist, player, monster)
}

fn get_spec_min_max_hit(player: &Player, monster: &Monster) -> (u32, u32) {
    let combat_type = player.combat_type();
    let base_max_hit = player.max_hits.get(combat_type);
    match player.gear.weapon.name.as_str() {
        "Soulreaper axe" => {
            let current_stacks = player.boosts.soulreaper_stacks;
            let mut player_copy = player.clone();
            player_copy.boosts.soulreaper_stacks = 0;
            calc_active_player_rolls(&mut player_copy, monster);

            (
                0,
                player_copy.max_hits.get(combat_type) * (100 + 6 * current_stacks) / 100,
            )
        }
        "Saradomin godsword" | "Zamorak godsword" | "Ancient godsword" | "Dragon halberd"
        | "Crystal halberd" | "Saradomin sword" | "Barrelchest anchor" => {
            (0, base_max_hit * 11 / 10)
        }
        "Armadyl godsword" => (0, (base_max_hit * 11 / 10) * 5 / 4),
        "Bandos godsword" => (0, (base_max_hit * 11 / 10) * 11 / 10),
        "Dragon sword"
        | "Dragon longsword"
        | "Saradomin's blessed sword"
        | "Heavy ballista"
        | "Light ballista" => (0, base_max_hit * 5 / 4),
        "Dragon warhammer" | "Toxic blowpipe" | "Dragon mace" => (0, base_max_hit * 3 / 2),
        "Voidwaker" => (base_max_hit / 2, base_max_hit * 3 / 2),
        "Dragon dagger" => (0, base_max_hit * 23 / 20),
        "Abyssal dagger" => (0, base_max_hit * 17 / 20),
        "Abyssal bludegon" => {
            let damage_mod =
                1000 + 5 * max(0, player.stats.prayer.base - player.stats.prayer.current);
            (0, base_max_hit * damage_mod / 1000)
        }
        "Dual macuahuitl" if player.set_effects.full_blood_moon => {
            (base_max_hit / 4, base_max_hit * 5 / 4)
        }
        "Webweaver bow" => (0, base_max_hit - base_max_hit * 6 / 10),
        "Dark bow" => {
            let descent_of_dragons = player.is_wearing("Dragon arrow", None);
            let min_hit = if descent_of_dragons { 5 } else { 8 };
            let damage_factor = if descent_of_dragons { 15 } else { 13 };
            (min_hit, base_max_hit * damage_factor / 10)
        }
        "Accursed sceptre" | "Accursed sceptre (a)" => (0, base_max_hit * 3 / 2),
        "Magic shortbow" | "Magic shortbow (i)" | "Magic longbow" | "Magic comp bow"
        | "Seercull" => (0, player.seercull_spec_max()),
        _ => panic!("Spec not implemented for {}", player.gear.weapon.name),
    }
}

fn apply_limiters(
    dist: AttackDistribution,
    player: &Player,
    monster: &Monster,
) -> AttackDistribution {
    if monster.is_immune(player) {
        return AttackDistribution::new(vec![HitDistribution::new(vec![WeightedHit::new(
            1.0,
            vec![Hitsplat::inaccurate()],
        )])]);
    }

    let mut dist = dist;

    // Any hit over 50 is rerolled between 45 and 50 at Zulrah
    if monster.info.name.contains("Zulrah") {
        dist = dist.transform(
            &capped_reroll_transformer(50, 5, 45),
            &TransformOpts::default(),
        );
    }

    // Seren rolls a number between 22-24 and takes the lower of that and the original damage roll
    if monster.info.name.contains("Fragment of Seren") {
        dist = dist.transform(&linear_min_transformer(2, 22), &TransformOpts::default());
    }

    // Kraken divides all ranged damage by 7
    if monster.info.name.as_str() == "Kraken (Kraken)" && player.is_using_ranged() {
        dist = dist.transform(
            &division_transformer(7, 1),
            &TransformOpts {
                transform_inaccurate: false,
            },
        );
    }

    // Verzik rolls a number between 0-10 (melee) or 0-3 (other styles) and takes the lower of that and the original damage roll
    if monster.info.name.contains("Verzik")
        && monster.matches_version("Phase 1")
        && !player.is_wearing("Dawnbringer", None)
    {
        let limit = if player.is_using_melee() { 10 } else { 3 };
        dist = dist.transform(&linear_min_transformer(limit, 0), &TransformOpts::default());
    }

    // Tekton divides all magic damage by 5, with a minimum accurate hit of 1
    if monster.info.name.contains("Tekton") && player.combat_type() == CombatType::Magic {
        dist = dist.transform(
            &division_transformer(5, 1),
            &TransformOpts {
                transform_inaccurate: false,
            },
        );
    }

    // Vasa crystal takes 1/3 magic damage
    if monster.info.name.contains("Glowing crystal") && player.combat_type() == CombatType::Magic {
        dist = dist.transform(&division_transformer(3, 0), &TransformOpts::default());
    }

    // Olm melee hand or head takes 1/3 magic damage
    if (monster.matches_version("Left claw")
        || (monster.info.name.contains("Great Olm") && monster.matches_version("Head")))
        && player.combat_type() == CombatType::Magic
    {
        dist = dist.transform(&division_transformer(3, 0), &TransformOpts::default());
    }

    // Olm melee hand or mage hand takes 1/3 ranged damage
    if (monster.matches_version("Right claw") || monster.matches_version("Left claw"))
        && player.is_using_ranged()
    {
        dist = dist.transform(&division_transformer(3, 0), &TransformOpts::default());
    }

    // TODO: Implement updated Efaritay's aid here once wiki calc does

    // Ice demon takes 1/3 unless using a fire spell
    if monster.info.name.contains("Ice demon") && !player.is_using_fire_spell() {
        dist = dist.transform(&division_transformer(3, 0), &TransformOpts::default());
    }

    // Slagilith takes 1/3 unless using a pickaxe
    if monster.info.name.contains("Slagilith") && !player.gear.weapon.name.contains("pickaxe") {
        dist = dist.transform(&division_transformer(3, 0), &TransformOpts::default());
    }

    // Zogres take 1/2 damage from Crumble Undead and 1/4 damage from anything other than ranged with brutal arrows
    if ["Slash Bash", "Zogre", "Skogre"].contains(&monster.info.name.as_str()) {
        if player.attrs.spell == Some(Spell::Standard(StandardSpell::CrumbleUndead)) {
            dist = dist.transform(&division_transformer(2, 0), &TransformOpts::default());
        } else if !player.is_using_ranged()
            || !player
                .gear
                .ammo
                .as_ref()
                .is_some_and(|ammo| ammo.name.contains(" brutal"))
            || !player.gear.weapon.name.contains("Comp ogre bow")
        {
            dist = dist.transform(&division_transformer(4, 0), &TransformOpts::default());
        }
    }

    // Efaritay's aid with non-silver weapons against T2 vampyres deals 50% damage, applied post-roll
    if player.is_wearing("Efaritay's aid", None) && monster.vampyre_tier() == Some(2) {
        dist = dist.transform(&division_transformer(2, 0), &TransformOpts::default());
    }

    if monster.info.id == Some(HUEYCOATL_TAIL_ID) {
        let using_crush = player.combat_type() == CombatType::Crush
            && player.bonuses.attack.crush > player.bonuses.attack.stab
            && player.bonuses.attack.crush > player.bonuses.attack.slash;
        let dist_max = if using_crush { 9 } else { 4 };
        dist = dist.transform(
            &linear_min_transformer(dist_max, 0),
            &TransformOpts::default(),
        );
        if using_crush {
            dist = dist.transform(
                &|h| {
                    if h.damage > 0 {
                        HitDistribution::single(1.0, vec![Hitsplat::new(h.damage, true)])
                    } else {
                        HitDistribution::single(1.0, vec![Hitsplat::new(1, false)])
                    }
                },
                &TransformOpts::default(),
            );
        }
    }

    // Subtract flat armour from hitsplat, with a minimum of 1 on an accurate hit

    if monster.bonuses.flat_armour > 0 {
        dist = dist.transform(
            &flat_add_transformer(-monster.bonuses.flat_armour, 1),
            &TransformOpts {
                transform_inaccurate: false,
            },
        );
    }

    dist
}

pub fn get_max(
    dist: AttackDistribution,
    player: &Player,
    monster: &Monster,
    using_spec: bool,
) -> u32 {
    dist.get_max() + get_dot_max(player, monster, using_spec)
}

pub fn get_expected_damage(
    dist: AttackDistribution,
    player: &Player,
    monster: &Monster,
    using_spec: bool,
) -> f64 {
    dist.get_expected_damage() + get_dot_expected(player, monster, using_spec)
}

// Get the average damage per tick
fn get_dpt(dist: AttackDistribution, player: &Player) -> f64 {
    dist.get_expected_damage() / player.gear.weapon.speed as f64
}

// Get the average damage per second
pub fn get_dps(dist: AttackDistribution, player: &Player) -> f64 {
    get_dpt(dist, player) / SECONDS_PER_TICK
}

// Get the expected number of hits per kill
fn get_htk(dist: AttackDistribution, monster: &Monster) -> f64 {
    let mut dist = dist;
    let hist = dist.as_histogram(false);
    let start_hp = monster.stats.hitpoints.current as usize;
    let max_hit = min(start_hp, dist.get_max() as usize);
    if max_hit == 0 {
        return 0.0;
    }

    let mut htk = vec![0.0; start_hp + 1];

    for hp in 1..=start_hp {
        let mut val = 1.0;
        for hit in 1..=hp.min(max_hit) {
            let p = &hist[hit];
            val += p.value * htk[hp - hit];
        }

        htk[hp] = val / (1.0 - hist[0].value);
    }

    htk[start_hp]
}

// Get the expected time to kill
pub fn get_ttk(
    dist: AttackDistribution,
    player: &Player,
    monster: &Monster,
    using_spec: bool,
) -> f64 {
    if dist_is_current_hp_dependent(player, monster) {
        // More expensive than get_htk, so only use this if the hit dist changes during the fight
        let ttk_dist = get_ttk_distribution(&mut dist.clone(), player, monster, using_spec);

        // Find the expected value of the ttk distribution
        ttk_dist
            .iter()
            .map(|(ticks, prob)| *prob * *ticks as f64)
            .sum::<f64>()
            * SECONDS_PER_TICK
    } else {
        get_htk(dist, monster) * player.gear.weapon.speed as f64 * SECONDS_PER_TICK
    }
}

// Get the full ttk distribution
pub fn get_ttk_distribution(
    dist: &mut AttackDistribution,
    player: &Player,
    monster: &Monster,
    using_spec: bool,
) -> HashMap<usize, f64> {
    let speed = player.gear.weapon.speed as usize;
    let max_hp = monster.stats.hitpoints.current as usize;
    let mut dist_copy = dist.clone();
    let dist_single = dist_copy.get_single_hitsplat();

    // Return empty distribution if the expected damage is 0
    if dist_single.expected_hit() == 0.0 {
        return HashMap::new();
    }

    // Probability distribution of hp values at current iteration
    let mut hps = vec![0.0; max_hp + 1];
    hps[max_hp] = 1.0;

    // Output map of ttk values and their probabilities
    let mut ttks: HashMap<usize, f64> = HashMap::new();

    // Sum of non-zero hp probabilities
    let mut epsilon = 1.0;

    // If the dist is based on current hp, recalculate it at each hp and cache results
    let recalc_dist_on_hp = dist_is_current_hp_dependent(player, monster);
    let mut hp_hit_dists = HashMap::new();
    hp_hit_dists.insert(max_hp, dist_single.clone());
    if recalc_dist_on_hp {
        for hp in 0..max_hp {
            dist_at_hp(dist, hp, player, monster, &mut hp_hit_dists, using_spec);
        }
    }

    // Loop until the number of non-zero hp probabilities is less than TTK_DIST_EPSILON
    // or the number of iterations exceeds TTK_DIST_MAX_ITER_ROUNDS
    for hit in 0..=TTK_DIST_MAX_ITER_ROUNDS {
        if epsilon < TTK_DIST_EPSILON {
            break;
        }

        // Initialize the updated hp probability distribution
        let mut next_hps = vec![0.0; max_hp + 1];

        // For each possible hp value
        for (hp, hp_prob) in hps.iter().enumerate() {
            // Get the current hit distribution (the original or cached one based on current hp)
            let current_dist = if recalc_dist_on_hp {
                hp_hit_dists.get(&hp).unwrap()
            } else {
                dist_single
            };

            // For each possible damage amount
            for h in &current_dist.hits {
                let dmg_prob = h.probability;
                let dmg = h.hitsplats[0].damage as usize; // Single hitsplat, so guaranteed to be length 1

                // Chance of this path being reached is the previous chance of landing here * the chance of hitting this amount
                let chance_of_action = dmg_prob * hp_prob;
                if chance_of_action == 0.0 {
                    continue;
                }

                let new_hp = hp as i32 - dmg as i32;

                // If the hp we are about to arrive at is <= 0, the NPC is killed, the iteration count is the number of hits done,
                // and we add this probability path into the delta
                if new_hp <= 0 {
                    let tick = (hit + 1) * speed;
                    ttks.insert(tick, ttks.get(&tick).unwrap_or(&0.0) + chance_of_action);
                    epsilon -= chance_of_action;
                } else {
                    // Otherwise, we add the chance of this path to the next iteration's hp value
                    next_hps[new_hp as usize] += chance_of_action;
                }
            }
        }

        // Update counters and repeat
        hps = next_hps;
    }

    ttks
}

fn dist_from_multiple_hits(hits_vec: Vec<Vec<WeightedHit>>) -> AttackDistribution {
    // Create an AttackDistribution from multiple WeightedHits
    let mut combined_hits = Vec::new();
    for hits in hits_vec {
        combined_hits.extend(hits);
    }
    AttackDistribution::new(vec![HitDistribution::new(combined_hits)])
}

fn dist_is_current_hp_dependent(player: &Player, monster: &Monster) -> bool {
    // Check if the hit distribution depends on the monster's current hp (currently just rubies and Vardorvis)
    if monster.info.name.contains("Vardorvis") {
        return true;
    }

    if player.is_using_crossbow() && player.is_wearing_any(RUBY_BOLTS) {
        return true;
    }

    if player.is_wearing("Keris partisan of the sun", None)
        && TOA_MONSTERS.contains(&monster.info.id.unwrap_or(0))
    {
        return true;
    }

    false
}

fn dist_at_hp<'a>(
    dist: &'a mut AttackDistribution,
    hp: usize,
    player: &'a Player,
    monster: &'a Monster,
    hp_hit_dists: &'a mut HashMap<usize, HitDistribution>,
    using_spec: bool,
) {
    // Calculate the hit distribution at a specific hp

    // Return the original distribution if applicable to save some computation
    // (rubies above 500 hp, hp = max hp, or no hp scaling at all)
    let no_scaling = dist.get_single_hitsplat();
    if !dist_is_current_hp_dependent(player, monster)
        || hp == monster.stats.hitpoints.current as usize
        || (player.is_wearing("Keris partisan of the sun", None)
            && TOA_MONSTERS.contains(&monster.info.id.unwrap_or(0))
            && hp >= monster.stats.hitpoints.current as usize / 4)
        || (player.is_using_ranged()
            && player.is_using_crossbow()
            && player.is_wearing_any(RUBY_BOLTS)
            && monster.stats.hitpoints.current >= 500
            && hp >= 500)
    {
        hp_hit_dists.insert(hp, no_scaling.clone());
        return;
    }

    // Scale monster's stats based on current hp (only applies to Vardorvis currently)
    let mut monster_copy = monster.clone();
    monster_copy.stats.hitpoints.current = hp as u32;
    monster_scaling::scale_monster_hp_only(&mut monster_copy);

    // Return the new hp-scaled distribution
    let mut new_dist = get_distribution(player, &monster_copy, using_spec);
    hp_hit_dists.insert(hp, new_dist.get_single_hitsplat().clone());
}

fn get_wardens_p2_min_max(player: &Player, monster: &Monster) -> (u32, u32) {
    let att_roll = max(
        0,
        player.att_rolls.get(player.combat_type())
            - monster.def_rolls.get(player.combat_type()) / 3,
    );

    let modifier = max(15, lerp(att_roll, 0, 42000, 15, 40));
    let base_max_hit = player.max_hits.get(player.combat_type());
    let min_hit = base_max_hit * modifier as u32 / 100;
    let max_hit = base_max_hit * (modifier + 20) as u32 / 100;

    (min_hit, max_hit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calc::rolls::{calc_player_melee_rolls, calc_player_ranged_rolls};
    use crate::types::equipment::CombatStyle;
    use crate::types::monster::Monster;
    use crate::types::player::Player;
    use crate::types::potions::Potion;
    use crate::types::prayers::Prayer;
    use crate::types::stats::PlayerStats;
    #[test]
    fn test_max_melee_ammonite_crab() {
        let mut player = Player::new();
        player.stats = PlayerStats::default();
        player.prayers.add(Prayer::Piety);
        player.add_potion(Potion::SuperCombat);

        player.equip("Torva full helm", None);
        player.equip("Amulet of torture", None);
        player.equip("Infernal cape", None);
        player.equip("Rada's blessing 4", None);
        player.equip("Ghrazi rapier", None);
        player.equip("Avernic defender", None);
        player.equip("Torva platebody", None);
        player.equip("Torva platelegs", None);
        player.equip("Ferocious gloves", None);
        player.equip("Primordial boots", None);
        player.equip("Ultor ring", None);

        player.update_bonuses();
        player.set_active_style(CombatStyle::Lunge);
        let monster = Monster::new("Ammonite Crab", None).unwrap();
        calc_player_melee_rolls(&mut player, &monster);

        let dist = get_distribution(&player, &monster, false);
        let ttk = get_ttk(dist, &player, &monster, false);

        assert!(num::abs(ttk - 10.2) < 0.1);
    }

    #[test]
    fn test_macuahuitl() {
        let mut player = Player::new();
        player.stats = PlayerStats::default();
        player.prayers.add(Prayer::Piety);
        player.add_potion(Potion::SuperCombat);

        player.equip("Torva full helm", None);
        player.equip("Amulet of torture", None);
        player.equip("Infernal cape", None);
        player.equip("Rada's blessing 4", None);
        player.equip("Dual macuahuitl", None);
        player.equip("Torva platebody", None);
        player.equip("Torva platelegs", None);
        player.equip("Ferocious gloves", None);
        player.equip("Primordial boots", None);
        player.equip("Ultor ring", None);

        player.update_bonuses();
        player.set_active_style(CombatStyle::Pummel);

        let monster = Monster::new("Vet'ion", Some("Normal")).unwrap();
        calc_player_melee_rolls(&mut player, &monster);
        let dist = get_distribution(&player, &monster, false);
        let ttk = get_ttk(dist, &player, &monster, false);

        assert!(num::abs(ttk - 44.2) < 0.1);
    }

    #[test]
    fn test_scythe_vardorvis() {
        let mut player = Player::new();
        player.stats = PlayerStats::default();
        player.prayers.add(Prayer::Piety);
        player.add_potion(Potion::SuperCombat);

        player.equip("Torva full helm", None);
        player.equip("Amulet of torture", None);
        player.equip("Infernal cape", None);
        player.equip("Rada's blessing 4", None);
        player.equip("Scythe of vitur", Some("Charged"));
        player.equip("Torva platebody", None);
        player.equip("Torva platelegs", None);
        player.equip("Ferocious gloves", None);
        player.equip("Primordial boots", None);
        player.equip("Ultor ring", None);

        player.update_bonuses();
        player.set_active_style(CombatStyle::Chop);

        let monster = Monster::new("Vardorvis", Some("Post-quest")).unwrap();
        calc_player_melee_rolls(&mut player, &monster);
        let dist = get_distribution(&player, &monster, false);
        let ttk = get_ttk(dist, &player, &monster, false);

        assert!(num::abs(ttk - 90.6) < 0.1);
    }

    #[test]
    fn test_ruby_bolts_zcb_zebak_500() {
        let mut player = Player::new();
        player.stats = PlayerStats::default();
        player.prayers.add(Prayer::Rigour);
        player.add_potion(Potion::SmellingSalts);

        player.equip("Masori mask (f)", None);
        player.equip("Necklace of anguish", None);
        player.equip("Dizana's quiver", Some("Charged"));
        player.equip("Ruby dragon bolts (e)", None);
        player.equip("Zaryte crossbow", None);
        player.equip("Twisted buckler", None);
        player.equip("Masori body (f)", None);
        player.equip("Masori chaps (f)", None);
        player.equip("Zaryte vambraces", None);
        player.equip("Pegasian boots", None);
        player.equip("Venator ring", None);

        player.update_bonuses();
        player.set_active_style(CombatStyle::Rapid);

        let mut monster = Monster::new("Zebak", None).unwrap();
        monster.info.toa_level = 500;
        monster.scale_toa();
        calc_player_ranged_rolls(&mut player, &monster);

        let dist = get_distribution(&player, &monster, false);
        let ttk = get_ttk(dist, &player, &monster, false);

        assert!(num::abs(ttk - 240.8) < 0.1);
    }
}
