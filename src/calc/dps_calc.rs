// Adapted from the wiki DPS calc - credit to the wiki team

use crate::calc::burn::get_expected_burn;
use crate::calc::hit_dist::{
    AttackDistribution, DelayedHit, HitDistribution, Hitsplat, TransformOpts, WeightedHit,
    capped_reroll_transformer, division_transformer, flat_add_transformer, linear_min_transformer,
    multiply_transformer,
};
use crate::calc::hit_dist::{ProbabilisticDelay, WeaponDelayProvider, flat_limit_transformer};
use crate::calc::monster_scaling;
use crate::calc::rolls::{calc_active_player_rolls, get_demonbane_factor, monster_def_rolls};
use crate::constants::{self, TTK_DIST_MAX_ITER_ROUNDS};
use crate::dists;
use crate::dists::bolts::{self, BoltContext};
use crate::error::DpsCalcError;
use crate::types::equipment::{CombatStance, CombatType};
use crate::types::monster::Monster;
use crate::types::player::Player;
use crate::types::spells::{Spell, StandardSpell};
use crate::utils::math::{Fraction, lerp};
use std::cmp::{max, min};
use std::collections::HashMap;

fn get_normal_accuracy(
    player: &Player,
    monster: &Monster,
    using_spec: bool,
) -> Result<f64, DpsCalcError> {
    // Calculate theoretical hit chance for most weapons
    let combat_type = player.combat_type();
    let mut max_att_roll = player.att_rolls.get(combat_type)?;

    if using_spec {
        let att_roll_factor = match &player.gear.weapon.name as &str {
            "Saradomin godsword" | "Bandos godsword" | "Zamorak godsword" | "Armadyl godsword"
            | "Zaryte crossbow" | "Webweaver bow" | "Toxic blowpipe" | "Ancient godsword"
            | "Brine sabre" | "Barrelchest anchor" | "Eye of Ayak" => Fraction::new(2, 1),
            "Accursed sceptre"
            | "Accursed sceptre (a)"
            | "Volatile Nightmare staff"
            | "Arkan blade"
            | "Granite hammer" => Fraction::new(3, 2),
            "Dragon dagger" => Fraction::new(115, 100),
            "Abyssal dagger" | "Abyssal whip" | "Dragon mace" | "Dragon sword" | "Elder maul" => {
                Fraction::new(5, 4)
            }
            "Soulreaper axe" => {
                Fraction::new(100 + 6 * player.boosts.soulreaper_stacks as i32, 100)
            }
            "Magic shortbow" | "Magic shortbow (i)" => Fraction::new(10, 7),
            "Heavy ballista" | "Light ballista" => Fraction::new(5, 4),
            "Rosewood blowpipe" => Fraction::new(4, 5),
            _ => Fraction::new(1, 1),
        }
        .unwrap();
        max_att_roll = att_roll_factor.multiply_to_int(max_att_roll);
    }

    if player.is_wearing("Keris partisan of the sun", None)
        && constants::TOA_MONSTERS.contains(&monster.id())
        && monster.stats.hitpoints.current < monster.stats.hitpoints.base / 4
    {
        max_att_roll = max_att_roll * 5 / 4;
    }

    let mut def_roll = if using_spec {
        if constants::STAB_SPEC_WEAPONS.contains(&player.gear.weapon.name.as_str()) {
            monster.def_rolls.get(CombatType::Stab)
        } else if constants::SLASH_SPEC_WEAPONS.contains(&player.gear.weapon.name.as_str()) {
            monster.def_rolls.get(CombatType::Slash)
        } else if constants::CRUSH_SPEC_WEAPONS.contains(&player.gear.weapon.name.as_str()) {
            monster.def_rolls.get(CombatType::Crush)
        } else if constants::MAGIC_SPEC_WEAPONS.contains(&player.gear.weapon.name.as_str()) {
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
        (false, false) => Ok(std_roll(max_att_roll, def_roll)),
        (false, true) => Ok(1.0 - 1.0 / (-def_roll as f64 + 1.0) / (max_att_roll as f64 + 1.0)),
        (true, false) => Ok(0.0),
        (true, true) => Ok(std_roll(-max_att_roll, -def_roll)),
    }
}

fn get_fang_accuracy(
    player: &Player,
    monster: &Monster,
    using_spec: bool,
) -> Result<f64, DpsCalcError> {
    // Calculate theoretical hit chance for Osmumten's fang outside of ToA
    let combat_type = player.combat_type();
    let mut max_att_roll = player.att_rolls.get(combat_type)?;

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
        (false, false) => Ok(std_roll(max_att_roll, def_roll)),
        (false, true) => Ok(1.0 - 1.0 / (-def_roll as f64 + 1.0) / (max_att_roll as f64 + 1.0)),
        (true, false) => Ok(0.0),
        (true, true) => Ok(rv_roll(-def_roll, -max_att_roll)),
    }
}

fn get_confliction_gauntlets_accuracy(
    player: &Player,
    monster: &Monster,
    using_spec: bool,
) -> Result<f64, DpsCalcError> {
    let single_roll = get_normal_accuracy(player, monster, using_spec)?;
    let double_roll = get_fang_accuracy(player, monster, using_spec)?;

    Ok(double_roll / (1.0 - double_roll - single_roll))
}

pub fn get_hit_chance(
    player: &Player,
    monster: &Monster,
    using_spec: bool,
) -> Result<f64, DpsCalcError> {
    // Always accurate in these cases
    if (monster.info.name.contains("Verzik")
        && monster.matches_version("Phase 1")
        && player.is_wearing("Dawnbringer", None))
        || (monster.name() == "Giant rat (Scurrius)"
            && player.combat_stance() != CombatStance::ManualCast)
        || (using_spec && player.is_wearing_any(constants::ALWAYS_HITS_SPEC))
        || constants::P2_WARDEN_IDS.contains(&monster.id())
        || constants::GUARANTEED_ACCURACY_MONSTERS.contains(&monster.id())
        || (monster.name() == "Eclipse Moon"
            && monster.matches_version("Clone")
            && player.is_using_melee())
    {
        return Ok(1.0);
    }

    let mut hit_chance = if player.is_wearing("Confliction gauntlets", None)
        && player.is_using_magic()
        && !player.gear.weapon.is_two_handed
    {
        get_confliction_gauntlets_accuracy(player, monster, using_spec)?
    } else {
        get_normal_accuracy(player, monster, using_spec)?
    };

    if player.is_wearing("Osmumten's fang", None) && player.combat_type() == CombatType::Stab {
        if monster.is_toa_monster() {
            hit_chance = 1.0 - (1.0 - hit_chance) * (1.0 - hit_chance);
        } else {
            hit_chance = get_fang_accuracy(player, monster, using_spec)?;
        }
    }

    if player.is_using_magic() && player.is_wearing("Brimstone ring", None) {
        let mut monster_copy = monster.clone();
        let def_roll = monster.def_rolls.get(CombatType::Magic) * 9 / 10;
        monster_copy.def_rolls.set(CombatType::Magic, def_roll);
        hit_chance =
            hit_chance * 0.75 + get_normal_accuracy(player, &monster_copy, using_spec)? * 0.25;
    }

    Ok(hit_chance)
}

fn get_dot_expected(
    player: &Player,
    monster: &Monster,
    using_spec: bool,
) -> Result<f64, DpsCalcError> {
    if using_spec {
        if player.is_wearing("Burning claws", None) {
            burning_claw_dot(player, monster)
        } else if player.is_wearing("Scorching bow", None) {
            if monster.is_demon() { Ok(5.0) } else { Ok(1.0) }
        } else if player.is_wearing("Ancient godsword", None) {
            let accuracy = get_hit_chance(player, monster, true)?;
            Ok(accuracy * 25.0)
        } else if player.is_wearing("Arkan blade", None) && !monster.is_immune_to_strong_burn() {
            let accuracy = get_hit_chance(player, monster, true)?;
            Ok(accuracy * 10.0)
        } else {
            Ok(0.0)
        }
    } else if player.set_effects.full_eclipse_moon {
        let accuracy = get_hit_chance(player, monster, using_spec)?;
        let attack_speed = player.gear.weapon.speed as usize;
        Ok(get_expected_burn(
            accuracy,
            attack_speed,
            constants::ECLIPSE_MOON_BURN_CHANCE,
        ))
    } else {
        Ok(0.0)
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

fn burning_claw_dot(player: &Player, monster: &Monster) -> Result<f64, DpsCalcError> {
    if monster.is_immune_to_normal_burn() {
        return Ok(0.0);
    }

    let mut dot = 0.0;
    let accuracy = get_hit_chance(player, monster, true)?;
    for acc_roll in 0..3 {
        let prev_rolls_fail = (1.0 - accuracy).powi(acc_roll);
        let this_roll_hits = prev_rolls_fail * accuracy;
        dot += this_roll_hits * constants::BURN_EXPECTED[acc_roll as usize];
    }

    Ok(dot)
}

pub fn get_distribution(
    player: &Player,
    monster: &Monster,
    using_spec: bool,
) -> Result<AttackDistribution, DpsCalcError> {
    // Get the attack distribution for the given player and monster
    let acc = get_hit_chance(player, monster, using_spec)?;
    let combat_type = player.combat_type();
    let (mut min_hit, max_hit) = if using_spec {
        get_spec_min_max_hit(player, monster)?
    } else if constants::P2_WARDEN_IDS.contains(&monster.id()) {
        get_wardens_p2_min_max(player, monster)?
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
    if constants::ONE_HIT_MONSTERS.contains(&monster.id()) {
        return Ok(AttackDistribution::new(vec![HitDistribution::single(
            1.0,
            vec![Hitsplat::new(monster.stats.hitpoints.base, true)],
        )]));
    }

    // Sire vents always die in one hit if the player is using a demonbane weapon
    if monster.info.name == "Respiratory system" && player.is_using_demonbane() {
        return Ok(AttackDistribution::new(vec![HitDistribution::single(
            acc,
            vec![Hitsplat::new(monster.stats.hitpoints.current, true)],
        )]));
    }

    // Check if the monster always takes the maximum hit for the current combat type
    if player.is_using_magic() && constants::ALWAYS_MAX_HIT_MAGIC.contains(&monster.id())
        || player.is_using_melee() && constants::ALWAYS_MAX_HIT_MELEE.contains(&monster.id())
        || player.is_using_ranged() && constants::ALWAYS_MAX_HIT_RANGED.contains(&monster.id())
    {
        if monster.info.name == "Void Flare"
            && player.boosts.mark_of_darkness
            && player.is_using_demonbane_spell()
        {
            let damage_boost = if player.is_wearing("Purging staff", None) {
                50
            } else {
                25
            };
            return Ok(AttackDistribution::new(vec![HitDistribution::single(
                1.0,
                vec![Hitsplat::new(
                    max_hit
                        + get_demonbane_factor(100, monster)
                            .multiply_to_int(max_hit * damage_boost / 100),
                    true,
                )],
            )]));
        }

        return Ok(AttackDistribution::new(vec![HitDistribution::single(
            1.0,
            vec![Hitsplat::new(dist.get_max(), true)],
        )]));
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
            .scale_damage(Fraction::new(5, 4).unwrap());
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
        let second_hit_att_roll = player.att_rolls.get(player.combat_type())? * 3 / 4;
        let mut player_copy = player.clone();
        player_copy
            .att_rolls
            .set(player.combat_type(), second_hit_att_roll)
            .unwrap_or_else(|_| panic!("Failed to set second attack roll for halberd spec"));
        calc_active_player_rolls(&mut player_copy, monster);

        let second_hit_acc = get_hit_chance(&player_copy, monster, using_spec)?;
        dist = AttackDistribution::new(vec![
            standard_hit_dist.clone(),
            HitDistribution::linear(second_hit_acc, min_hit, max_hit),
        ]);
    }

    // Simple multi-hit specs
    if using_spec {
        let hit_count = if player.is_wearing_any_version("Dragon dagger")
            || player.is_wearing_any_version("Dragon knife")
            || player.is_wearing_any(constants::MAGIC_SHORTBOWS)
            || player.is_wearing_any_version("Rosewood blowpipe")
        {
            2
        } else if player.is_wearing("Webweaver bow", None) {
            4
        } else {
            0
        };

        if hit_count > 0 {
            dist = AttackDistribution::default();
            for _ in 0..hit_count {
                dist.add_dist(standard_hit_dist.clone());
            }
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
        if !constants::IMMUNE_TO_MAGIC_MONSTERS.contains(&monster.id()) {
            dist = dist.transform(
                &|h| HitDistribution::new(vec![WeightedHit::new(1.0, vec![*h])]).zip(&magic_hit),
                &TransformOpts {
                    transform_inaccurate: false,
                },
            );
        }
    }

    // Granite hammer spec
    if using_spec && player.is_wearing("Granite hammer", None) {
        dist = dist.transform(
            &flat_add_transformer(5, 0),
            &TransformOpts {
                transform_inaccurate: true,
            },
        );
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
        && player.is_wearing_any_version("Amulet of the Damned")
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
    if player.is_using_melee() && player.is_wearing_any_version("Scythe of Vitur") {
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
    if player.is_using_melee() && player.is_wearing_any(constants::DOUBLE_HIT_WEAPONS) {
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
            if using_spec {
                // Defence drain from first hit affects accuracy of second hit
                let mut monster_copy = monster.clone();

                // Drains defence by 10% of the magic level
                let def_drain = monster_copy.stats.magic.base / 10;
                monster_copy.stats.defence.drain(def_drain);
                monster_copy.def_rolls = monster_def_rolls(&monster_copy);

                let second_hit_acc = get_hit_chance(player, &monster_copy, using_spec)?;
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
            } else {
                dist = AttackDistribution::new(vec![first_hit, second_hit]);
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
        let pick_bonus = constants::PICKAXE_BONUSES
            .iter()
            .find(|b| b.0 == player.gear.weapon.name)
            .ok_or_else(|| DpsCalcError::NoPickaxeBonus(player.gear.weapon.name.clone()))?
            .1;

        let numerator = 50 + player.stats.mining.current + pick_bonus;
        let denominator = 150;

        dist = dist.transform(
            &multiply_transformer(Fraction::new(numerator as i32, denominator).unwrap(), 0),
            &TransformOpts::default(),
        );
    }

    // Fire spell against ice demon distribution
    if monster.info.name.contains("Ice demon") && player.is_using_fire_spell()
        || player.attrs.spell == Some(Spell::Standard(StandardSpell::FlamesOfZamorak))
    {
        dist = dist.scale_damage(Fraction::new(3, 2).unwrap());
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
    if player.is_using_magic()
        && player.set_effects.full_ahrims
        && player.is_wearing_any_version("Amulet of the Damned")
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
            dist = dist.scale_damage(Fraction::new(11, 10).unwrap());
        }
        match (
            player.gear.weapon.name.as_str(),
            player.is_wearing_silver_weapon(),
            tier,
        ) {
            ("Blisterwood flail", _, _) => {
                dist = dist.scale_damage(Fraction::new(5, 4).unwrap());
            }
            ("Blisterwood sickle", _, _) => {
                dist = dist.scale_damage(Fraction::new(23, 20).unwrap());
            }
            ("Ivandis flail", _, _) => {
                dist = dist.scale_damage(Fraction::new(6, 5).unwrap());
            }
            ("Rod of Ivandis", _, 1 | 2) | (_, true, 1) => {
                dist = dist.scale_damage(Fraction::new(11, 10).unwrap());
            }
            (_, _, _) => {}
        }
    }

    if player.is_using_ranged() && player.is_wearing("Dark bow", None) {
        dist = AttackDistribution::new(vec![standard_hit_dist.clone(), standard_hit_dist.clone()]);
        if using_spec {
            dist = dist.transform(
                &flat_limit_transformer(Some(min_hit), Some(48)),
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
        if player.is_wearing_any(constants::OPAL_BOLTS) {
            dist = dist.transform(&bolts::opal_bolts(&bolt_context), &TransformOpts::default());
        }

        // Pearl bolts
        if player.is_wearing_any(constants::PEARL_BOLTS) {
            dist = dist.transform(
                &bolts::pearl_bolts(&bolt_context),
                &TransformOpts::default(),
            );
        }

        // Diamond bolts
        if player.is_wearing_any(constants::DIAMOND_BOLTS) {
            dist = dist.transform(
                &bolts::diamond_bolts(&bolt_context),
                &TransformOpts::default(),
            );
        }

        // Dragonstone bolts
        if player.is_wearing_any(constants::DRAGONSTONE_BOLTS)
            && (!monster.is_fiery() || !monster.is_dragon())
        {
            dist = dist.transform(
                &bolts::dragonstone_bolts(&bolt_context),
                &TransformOpts::default(),
            );
        }

        // Onyx bolts
        if player.is_wearing_any(constants::ONYX_BOLTS) {
            dist = dist.transform(&bolts::onyx_bolts(&bolt_context), &TransformOpts::default());
        }
    }

    // Apply corp transform before ruby bolt procs
    if monster.name() == "Corporeal Beast" && !player.is_using_corpbane_weapon() {
        dist = dist.transform(&division_transformer(2, 0), &TransformOpts::default());
    }

    // Ruby bolts
    if player.is_using_ranged()
        && player.is_using_crossbow()
        && player.is_wearing_any(constants::RUBY_BOLTS)
    {
        dist = dist.transform(&bolts::ruby_bolts(&bolt_context), &TransformOpts::default());
    }

    // Berserker necklace + obby weapon distribution (tested, confirmed post-roll)
    if player.is_using_melee()
        && player.is_wearing("Berserker necklace", None)
        && player.is_wearing_tzhaar_weapon()
    {
        dist = dist.scale_damage(Fraction::new(6, 5).unwrap());
    }

    // Dharok's set effect distribution
    if player.is_using_melee() && player.set_effects.full_dharoks {
        let full_hp = player.stats.hitpoints.base;
        let current_hp = player.stats.hitpoints.current;
        let numerator = 10000 + (full_hp - current_hp) as i32 * full_hp as i32;
        dist = dist.scale_damage(Fraction::new(numerator, 10000).unwrap());
    }

    // Seeking arrows clamp ranged min hit to 3 on accurate hits
    if player.is_using_ranged() && player.is_using_seeking_arrows() {
        dist = dist.transform(
            &flat_limit_transformer(Some(3), None),
            &TransformOpts {
                transform_inaccurate: false,
            },
        )
    }

    // Accurate 0 -> 1 is either overwritten by ruby bolts or divided back down to 0
    if accurate_zero_applicable
        && (monster.name() != "Corporeal Beast" || player.is_using_corpbane_weapon())
    {
        dist = dist.transform(
            &flat_limit_transformer(Some(1), None),
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
        );
    }

    Ok(apply_limiters(dist, player, monster))
}

fn get_spec_min_max_hit(player: &Player, monster: &Monster) -> Result<(u32, u32), DpsCalcError> {
    let combat_type = player.combat_type();
    let base_max_hit = player.max_hits.get(combat_type);
    let min_max = match player.gear.weapon.name.as_str() {
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
        | "Crystal halberd" | "Saradomin sword" | "Barrelchest anchor" | "Rosewood blowpipe" => {
            (0, base_max_hit * 11 / 10)
        }
        "Armadyl godsword" => (0, (base_max_hit * 11 / 10) * 5 / 4),
        "Bandos godsword" => (0, (base_max_hit * 11 / 10) * 11 / 10),
        "Dragon sword"
        | "Dragon longsword"
        | "Saradomin's blessed sword"
        | "Heavy ballista"
        | "Light ballista" => (0, base_max_hit * 5 / 4),
        "Dragon warhammer"
        | "Toxic blowpipe"
        | "Dragon mace"
        | "Accursed sceptre"
        | "Accursed sceptre (a)"
        | "Arkan blade" => (0, base_max_hit * 3 / 2),
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
        "Magic shortbow" | "Magic shortbow (i)" | "Magic longbow" | "Magic comp bow"
        | "Seercull" => (0, player.seercull_spec_max()),
        "Eye of Ayak" => (0, base_max_hit * 13 / 10),
        _ => (0, base_max_hit),
    };

    Ok(min_max)
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
    if ["Kraken", "Cave kraken"].contains(&monster.name()) && player.is_using_ranged() {
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
    if monster.info.name.contains("Tekton") && player.is_using_magic() {
        dist = dist.transform(
            &division_transformer(5, 1),
            &TransformOpts {
                transform_inaccurate: false,
            },
        );
    }

    // Vasa crystal takes 1/3 magic damage
    if monster.info.name.contains("Glowing crystal") && player.is_using_magic() {
        dist = dist.transform(&division_transformer(3, 0), &TransformOpts::default());
    }

    // Olm melee hand or head takes 1/3 magic damage
    if (monster.matches_version("Left claw")
        || (monster.info.name.contains("Great Olm") && monster.matches_version("Head")))
        && player.is_using_magic()
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
    if monster.info.name.contains("Ice demon")
        && !player.is_using_fire_spell()
        && !player.is_using_demonbane()
    {
        dist = dist.transform(&division_transformer(3, 0), &TransformOpts::default());
    }

    // Slagilith takes 1/3 unless using a pickaxe
    if monster.info.name.contains("Slagilith") && !player.gear.weapon.name.contains("pickaxe") {
        dist = dist.transform(&division_transformer(3, 0), &TransformOpts::default());
    }

    // Zogres take 1/2 damage from Crumble Undead and 1/4 damage from anything other than ranged with brutal arrows
    if ["Slash Bash", "Zogre", "Skogre"].contains(&monster.name()) {
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
    if monster.vampyre_tier() == Some(2) {
        if !player.is_using_vampyrebane(2) && player.is_wearing("Efaritay's aid", None) {
            dist = dist.transform(&division_transformer(2, 0), &TransformOpts::default());
        } else if player.is_wearing_silver_weapon() {
            dist = dist.transform(
                &flat_limit_transformer(None, Some(10)),
                &TransformOpts::default(),
            );
        }
    }

    if monster.id() == constants::HUEYCOATL_TAIL_ID {
        let using_crush = player.combat_type() == CombatType::Crush
            && player.bonuses.attack.crush > player.bonuses.attack.stab
            && player.bonuses.attack.crush > player.bonuses.attack.slash;
        let dist_max = if using_crush || player.is_using_earth_spell() {
            9
        } else {
            4
        };
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
    if monster.bonuses.flat_armour > 0 && player.combat_type() != CombatType::Magic {
        dist = dist.transform(
            &flat_add_transformer(-monster.bonuses.flat_armour, 0),
            &TransformOpts {
                transform_inaccurate: false,
            },
        );
    }

    dist
}

pub fn get_max(
    dist: &AttackDistribution,
    player: &Player,
    monster: &Monster,
    using_spec: bool,
) -> u32 {
    dist.get_max() + get_dot_max(player, monster, using_spec)
}

pub fn get_expected_damage(
    dist: &AttackDistribution,
    player: &Player,
    monster: &Monster,
    using_spec: bool,
) -> Result<f64, DpsCalcError> {
    Ok(dist.get_expected_damage() + get_dot_expected(player, monster, using_spec)?)
}

fn get_attack_speed(player: &Player, using_spec: bool) -> u32 {
    if using_spec && player.is_wearing("Eye of Ayak", Some("Charged")) {
        5
    } else {
        player.gear.weapon.speed as u32
    }
}

fn has_probabilistic_attack_speed(player: &Player) -> bool {
    player.set_effects.full_blood_moon
}

// Get the player's expected attack speed from the attack distribution itself,
// so probabilistic delays stay consistent with the TTK distribution.
pub fn get_expected_attack_speed(
    dist: &AttackDistribution,
    player: &Player,
    using_spec: bool,
) -> f64 {
    let delay_provider = get_weapon_delay_provider(player, using_spec);
    let mut dist = dist.clone();

    dist.zipped()
        .with_probabilistic_delays(delay_provider.as_ref())
        .iter()
        .map(|hit| hit.wh.probability * hit.delay as f64)
        .sum()
}

// Get the average damage per tick
fn get_dpt(
    dist: &AttackDistribution,
    player: &Player,
    monster: &Monster,
    using_spec: bool,
) -> Result<f64, DpsCalcError> {
    let speed = get_expected_attack_speed(dist, player, using_spec);
    Ok(get_expected_damage(dist, player, monster, using_spec)? / speed)
}

// Get the average damage per second
pub fn get_dps(
    dist: &AttackDistribution,
    player: &Player,
    monster: &Monster,
    using_spec: bool,
) -> Result<f64, DpsCalcError> {
    Ok(get_dpt(dist, player, monster, using_spec)? / constants::SECONDS_PER_TICK)
}

// Get the expected number of hits per kill
fn get_htk(dist: &AttackDistribution, monster: &Monster) -> f64 {
    let mut dist = dist.clone();
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
    dist: &AttackDistribution,
    player: &Player,
    monster: &Monster,
    using_spec: bool,
    remove_final_hit_delay: bool,
) -> Result<f64, DpsCalcError> {
    let ttk = if dist_is_current_hp_dependent(player, monster)
        || has_probabilistic_attack_speed(player)
    {
        // More expensive than get_htk, so only use this if the hit dist changes during the fight
        // or attack delay depends on the hit outcome.
        let ttk_dist = get_ttk_distribution(
            &mut dist.clone(),
            player,
            monster,
            using_spec,
            !remove_final_hit_delay,
        )?;

        // Find the expected value of the ttk distribution
        return Ok(ttk_dist
            .iter()
            .map(|(ticks, prob)| *prob * *ticks as f64)
            .sum::<f64>()
            * constants::SECONDS_PER_TICK);
    } else {
        get_htk(dist, monster)
            * get_expected_attack_speed(dist, player, using_spec)
            * constants::SECONDS_PER_TICK
    };

    if remove_final_hit_delay {
        Ok(ttk - (get_attack_speed(player, using_spec) - 1) as f64 * constants::SECONDS_PER_TICK)
    } else {
        Ok(ttk)
    }
}

fn get_weapon_delay_provider(player: &Player, using_spec: bool) -> Box<WeaponDelayProvider> {
    let base_speed = get_attack_speed(player, using_spec);
    if player.set_effects.full_blood_moon {
        Box::new(move |wh: &WeightedHit| {
            let mut chance_no_effect = 1.0;
            for hitsplat in &wh.hitsplats {
                if hitsplat.accurate {
                    chance_no_effect *= 0.67;
                } else {
                    break;
                }
            }

            vec![
                ProbabilisticDelay::new(1.0 - chance_no_effect, base_speed - 1),
                ProbabilisticDelay::new(chance_no_effect, base_speed),
            ]
        })
    } else {
        Box::new(move |_| vec![ProbabilisticDelay::new(1.0, base_speed)])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TtkTiming {
    KillTick,
    AfterFinalDelay,
}

impl TtkTiming {
    fn tick(self, attack_tick: usize, delay: usize) -> usize {
        match self {
            Self::KillTick => attack_tick,
            Self::AfterFinalDelay => attack_tick + delay.saturating_sub(1),
        }
    }
}

// Get the full ttk distribution. If include_final_hit_delay is false, entries
// are keyed by the tick the killing attack lands; if true, entries include the
// delay after that attack
pub fn get_ttk_distribution(
    dist: &mut AttackDistribution,
    player: &Player,
    monster: &Monster,
    using_spec: bool,
    include_final_hit_delay: bool,
) -> Result<HashMap<usize, f64>, DpsCalcError> {
    // Return empty distribution if the expected damage is 0
    if dist.get_expected_damage() == 0.0 {
        return Ok(HashMap::new());
    }

    let timing = if include_final_hit_delay {
        TtkTiming::AfterFinalDelay
    } else {
        TtkTiming::KillTick
    };

    let speed = get_attack_speed(player, using_spec) as usize;
    let iter_max = TTK_DIST_MAX_ITER_ROUNDS * speed;
    let max_hp = monster.stats.hitpoints.current as usize;
    let delay_provider = get_weapon_delay_provider(player, using_spec);
    let dist_with_delays = dist
        .zipped()
        .with_probabilistic_delays(delay_provider.as_ref());

    // If the dist is based on current hp, recalculate it at each hp and cache results
    let recalc_dist_on_hp = dist_is_current_hp_dependent(player, monster);
    let hp_hit_dists = if recalc_dist_on_hp {
        let mut hp_hit_dists = Vec::with_capacity(max_hp + 1);
        for hp in 0..=max_hp {
            hp_hit_dists.push(dist_at_hp(
                &dist_with_delays,
                hp,
                player,
                monster,
                using_spec,
                delay_provider.as_ref(),
            )?);
        }
        Some(hp_hit_dists)
    } else {
        None
    };

    let max_delay = hp_hit_dists
        .as_ref()
        .map(|dists| dists.iter().flatten())
        .into_iter()
        .flatten()
        .chain(dist_with_delays.iter())
        .map(|hit| hit.delay as usize)
        .max()
        .unwrap_or(speed);

    let tick_count = iter_max + max_delay + 1;
    let width = max_hp + 1;
    let mut attack_on_tick = vec![0.0; tick_count];
    attack_on_tick[1] = 1.0;

    // Flattened tick-by-HP table. Index (tick, hp) as tick * width + hp.
    let mut tick_hps = vec![0.0; tick_count * width];
    tick_hps[width + max_hp] = 1.0;

    // Output map of ttk values and their probabilities
    let mut ttks: HashMap<usize, f64> = HashMap::new();

    // Sum of non-zero hp probabilities
    let mut epsilon = 1.0;

    // Loop until the number of non-zero hp probabilities is less than TTK_DIST_EPSILON
    // or the number of iterations exceeds TTK_DIST_MAX_ITER_ROUNDS
    for tick in 1..=iter_max {
        if epsilon < constants::TTK_DIST_EPSILON {
            break;
        }

        if attack_on_tick[tick] == 0.0 {
            continue;
        }

        // For each possible hp value
        let hp_row_offset = tick * width;
        for hp in 1..=max_hp {
            let hp_prob = tick_hps[hp_row_offset + hp];
            if hp_prob == 0.0 {
                continue;
            }

            // Get the current hit distribution (the original or cached one based on current hp)
            let current_dist = if let Some(hp_hit_dists) = &hp_hit_dists {
                &hp_hit_dists[hp]
            } else {
                &dist_with_delays
            };

            // For each possible damage amount
            for h in current_dist {
                let dmg_prob = h.wh.probability;
                let dmg = h.wh.get_sum() as usize;

                // Chance of this path being reached is the previous chance of landing here * the chance of hitting this amount
                let chance_of_action = dmg_prob * hp_prob;
                if chance_of_action == 0.0 {
                    continue;
                }

                // If the damage is more than the remaining hp, the monster dies on this tick
                if dmg >= hp {
                    let ttk_tick = timing.tick(tick, h.delay as usize);
                    ttks.insert(
                        ttk_tick,
                        ttks.get(&ttk_tick).unwrap_or(&0.0) + chance_of_action,
                    );
                    epsilon -= chance_of_action;
                } else {
                    // Otherwise, we add the chance of this path to the next iteration's hp value
                    let next_tick = tick + h.delay as usize;
                    let next_hp = hp - dmg;
                    tick_hps[next_tick * width + next_hp] += chance_of_action;
                    attack_on_tick[next_tick] += chance_of_action;
                }
            }
        }
    }

    Ok(ttks)
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

    if player.is_using_crossbow() && player.is_wearing_any(constants::RUBY_BOLTS) {
        return true;
    }

    if player.is_wearing("Keris partisan of the sun", None)
        && constants::TOA_MONSTERS.contains(&monster.id())
    {
        return true;
    }

    false
}

fn dist_at_hp(
    dist: &[DelayedHit],
    hp: usize,
    player: &Player,
    monster: &Monster,
    using_spec: bool,
    delay_provider: &WeaponDelayProvider,
) -> Result<Vec<DelayedHit>, DpsCalcError> {
    // Calculate the hit distribution at a specific hp

    // Return the original distribution if applicable to save some computation
    // (rubies above 500 hp, hp = max hp, or no hp scaling at all)
    if !dist_is_current_hp_dependent(player, monster)
        || hp == monster.stats.hitpoints.current as usize
        || (player.is_wearing("Keris partisan of the sun", None)
            && constants::TOA_MONSTERS.contains(&monster.id())
            && hp >= monster.stats.hitpoints.current as usize / 4)
        || (player.is_using_ranged()
            && player.is_using_crossbow()
            && player.is_wearing_any(constants::RUBY_BOLTS)
            && monster.stats.hitpoints.current >= 500
            && hp >= 500)
    {
        return Ok(dist.to_vec());
    }

    // Scale monster's stats based on current hp (only applies to Vardorvis currently)
    let mut monster_copy = monster.clone();
    monster_copy.stats.hitpoints.current = hp as u32;
    monster_scaling::scale_monster_hp_only(&mut monster_copy, true);

    // Insert the new hp-scaled distribution
    Ok(get_distribution(player, &monster_copy, using_spec)?
        .zipped()
        .with_probabilistic_delays(delay_provider))
}

fn get_wardens_p2_min_max(player: &Player, monster: &Monster) -> Result<(u32, u32), DpsCalcError> {
    let att_roll = max(
        0,
        player.att_rolls.get(player.combat_type())?
            - monster.def_rolls.get(player.combat_type()) / 3,
    );

    let modifier = max(15, lerp(att_roll, 0, 42000, 15, 40));
    let base_max_hit = player.max_hits.get(player.combat_type());
    let min_hit = base_max_hit * modifier as u32 / 100;
    let max_hit = base_max_hit * (modifier + 20) as u32 / 100;

    Ok((min_hit, max_hit))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calc::monster_scaling::scale_monster_hp_only;
    use crate::calc::rolls::calc_active_player_rolls;
    use crate::types::equipment::CombatStyle;
    use crate::types::monster::Monster;
    use crate::types::player::Player;
    use crate::types::potions::Potion;
    use crate::types::prayers::Prayer;
    use crate::types::stats::PlayerStats;

    #[test]
    fn test_ttk_distribution_can_include_final_delay() {
        let player = Player::new();
        let mut monster = Monster::new("Ammonite Crab", None).expect("Error creating monster.");
        monster.stats.hitpoints.base = 1;
        monster.stats.hitpoints.current = 1;

        let dist = AttackDistribution::new(vec![HitDistribution::single(
            1.0,
            vec![Hitsplat::new(1, true)],
        )]);

        let kill_tick_dist =
            get_ttk_distribution(&mut dist.clone(), &player, &monster, false, false)
                .expect("Error calculating kill-tick ttk distribution.");
        let final_delay_dist =
            get_ttk_distribution(&mut dist.clone(), &player, &monster, false, true)
                .expect("Error calculating final-delay ttk distribution.");

        assert_eq!(kill_tick_dist.get(&1), Some(&1.0));
        assert_eq!(final_delay_dist.get(&5), Some(&1.0));
    }

    #[test]
    fn test_max_melee_ammonite_crab() {
        let mut player = Player::new();
        player.stats = PlayerStats::default();

        player.add_prayer(Prayer::Piety);
        player.add_potion(Potion::SuperCombat);

        player.equip("Torva full helm", None).unwrap();
        player.equip("Amulet of torture", None).unwrap();
        player.equip("Infernal cape", None).unwrap();
        player.equip("Rada's blessing 4", None).unwrap();
        player.equip("Ghrazi rapier", None).unwrap();
        player.equip("Avernic defender", None).unwrap();
        player.equip("Torva platebody", None).unwrap();
        player.equip("Torva platelegs", None).unwrap();
        player.equip("Ferocious gloves", None).unwrap();
        player.equip("Primordial boots", None).unwrap();
        player.equip("Ultor ring", None).unwrap();

        player.update_bonuses();
        player.set_active_style(CombatStyle::Lunge);
        let monster = Monster::new("Ammonite Crab", None).expect("Error creating monster.");
        calc_active_player_rolls(&mut player, &monster);

        let dist = get_distribution(&player, &monster, false)
            .expect("Error calculating attack distribution.");
        let ttk = get_ttk(&dist, &player, &monster, false, false).expect("Error calculating ttk.");

        assert!(num::abs(ttk - 10.2) < 0.1);
    }

    #[test]
    fn test_macuahuitl() {
        let mut player = Player::new();
        player.stats = PlayerStats::default();
        player.add_prayer(Prayer::Piety);
        player.add_potion(Potion::SuperCombat);

        player.equip("Torva full helm", None).unwrap();
        player.equip("Amulet of torture", None).unwrap();
        player.equip("Infernal cape", None).unwrap();
        player.equip("Rada's blessing 4", None).unwrap();
        player.equip("Dual macuahuitl", None).unwrap();
        player.equip("Torva platebody", None).unwrap();
        player.equip("Torva platelegs", None).unwrap();
        player.equip("Ferocious gloves", None).unwrap();
        player.equip("Primordial boots", None).unwrap();
        player.equip("Ultor ring", None).unwrap();

        player.update_bonuses();
        player.set_active_style(CombatStyle::Pummel);

        let monster = Monster::new("Vet'ion", Some("Normal")).expect("Error creating monster.");
        calc_active_player_rolls(&mut player, &monster);
        let dist = get_distribution(&player, &monster, false)
            .expect("Error creating attack distribution.");
        let ttk = get_ttk(&dist, &player, &monster, false, false).expect("Error calculating ttk.");

        assert!(num::abs(ttk - 44.2) < 0.1);
    }

    #[test]
    fn test_scythe_vardorvis() {
        let mut player = Player::new();
        player.stats = PlayerStats::default();
        player.add_prayer(Prayer::Piety);
        player.add_potion(Potion::SuperCombat);

        player.equip("Torva full helm", None).unwrap();
        player.equip("Amulet of torture", None).unwrap();
        player.equip("Infernal cape", None).unwrap();
        player.equip("Rada's blessing 4", None).unwrap();
        player.equip("Scythe of Vitur", Some("Charged")).unwrap();
        player.equip("Torva platebody", None).unwrap();
        player.equip("Torva platelegs", None).unwrap();
        player.equip("Ferocious gloves", None).unwrap();
        player.equip("Primordial boots", None).unwrap();
        player.equip("Ultor ring", None).unwrap();

        player.update_bonuses();
        player.set_active_style(CombatStyle::Chop);

        let mut monster =
            Monster::new("Vardorvis", Some("Post-quest")).expect("Error creating monster.");
        scale_monster_hp_only(&mut monster, true);
        calc_active_player_rolls(&mut player, &monster);
        let dist = get_distribution(&player, &monster, false)
            .expect("Error creating attack distribution.");
        let ttk = get_ttk(&dist, &player, &monster, false, false).expect("Error calculating ttk.");

        assert!(num::abs(ttk - 90.8) < 0.1);
    }

    #[test]
    fn test_ruby_bolts_zcb_zebak_500() {
        let mut player = Player::new();
        player.stats = PlayerStats::default();
        player.add_prayer(Prayer::Rigour);
        player.add_potion(Potion::SmellingSalts);

        player.equip("Masori mask (f)", None).unwrap();
        player.equip("Necklace of anguish", None).unwrap();
        player.equip("Dizana's quiver", Some("Charged")).unwrap();
        player.equip("Ruby dragon bolts (e)", None).unwrap();
        player.equip("Zaryte crossbow", None).unwrap();
        player.equip("Twisted buckler", None).unwrap();
        player.equip("Masori body (f)", None).unwrap();
        player.equip("Masori chaps (f)", None).unwrap();
        player.equip("Zaryte vambraces", None).unwrap();
        player.equip("Pegasian boots", None).unwrap();
        player.equip("Venator ring", None).unwrap();

        player.update_bonuses();
        player.set_active_style(CombatStyle::Rapid);

        let mut monster = Monster::new("Zebak", Some("Normal")).expect("Error creating monster.");
        monster.info.toa_level = 500;
        monster.scale_toa(true, true);
        calc_active_player_rolls(&mut player, &monster);

        let dist = get_distribution(&player, &monster, false)
            .expect("Error creating attack distribution.");
        let ttk = get_ttk(&dist, &player, &monster, false, false).expect("Error calculating ttk.");

        assert!(num::abs(ttk - 236.2) < 0.1);
    }
}
