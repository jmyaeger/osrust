use crate::constants::*;
use crate::equipment::{CombatStance, CombatType};
use crate::hit_dist::{
    capped_reroll_transformer, division_transformer, linear_min_transformer, multiply_transformer,
    AttackDistribution, HitDistribution, WeightedHit,
};
use crate::monster::Monster;
use crate::monster_scaling;
use crate::player::Player;
use crate::spells::{Spell, StandardSpell};
use std::cmp::{max, min};
use std::collections::HashMap;

fn get_normal_accuracy(player: &Player, monster: &Monster) -> f64 {
    let combat_type = player.combat_type();
    let mut max_att_roll = player.att_rolls[&combat_type];
    let mut def_roll = monster.def_rolls[&combat_type];

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

fn get_fang_accuracy(player: &Player, monster: &Monster) -> f64 {
    let combat_type = player.combat_type();
    let mut max_att_roll = player.att_rolls[&combat_type];
    let mut def_roll = monster.def_rolls[&combat_type];

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

fn get_hit_chance(player: &Player, monster: &Monster) -> f64 {
    let mut hit_chance = 1.0;

    if (monster.info.name.contains("Verzik")
        && monster.info.name.contains("(P1)")
        && player.is_wearing("Dawnbringer"))
        || (monster.info.name.as_str() == "Giant rat (Scurrius)"
            && player.combat_stance() != CombatStance::ManualCast)
    {
        return hit_chance;
    }

    hit_chance = get_normal_accuracy(player, monster);

    if player.is_wearing("Osmumten's fang") && player.combat_type() == CombatType::Stab {
        if monster.is_toa_monster() {
            hit_chance = 1.0 - (1.0 - hit_chance) * (1.0 - hit_chance);
        } else {
            hit_chance = get_fang_accuracy(player, monster);
        }
    }

    if player.combat_type() == CombatType::Magic && player.is_wearing("Brimstone ring") {
        let mut monster_copy = monster.clone();
        let def_roll = monster.def_rolls[&CombatType::Magic] * 9 / 10;
        monster_copy.def_rolls.insert(CombatType::Magic, def_roll);
        hit_chance = hit_chance * 0.75 + get_normal_accuracy(player, &monster_copy) * 0.25;
    }

    hit_chance
}

pub fn get_distribution(player: &Player, monster: &Monster) -> AttackDistribution {
    let acc = get_hit_chance(player, monster);
    let combat_type = player.combat_type();
    let max_hit = player.max_hits[&combat_type];

    let standard_hit_dist = HitDistribution::linear(acc, 0, max_hit);
    let mut dist = AttackDistribution::new(vec![standard_hit_dist.clone()]);

    if ONE_HIT_MONSTERS.contains(&monster.info.name.as_str()) {
        return AttackDistribution::new(vec![HitDistribution::single(
            1.0,
            monster.stats.hitpoints,
        )]);
    }

    if player.boosts.sunfire_runes && player.is_using_fire_spell() {
        dist = AttackDistribution::new(vec![HitDistribution::linear(acc, max_hit / 10, max_hit)]);
    }

    if player.is_using_melee() && player.is_wearing("Osmumten's fang") {
        let min_hit = player.max_hits[&CombatType::Stab] * 3 / 20;
        dist = AttackDistribution::new(vec![HitDistribution::linear(
            acc,
            min_hit,
            max_hit - min_hit,
        )]);
    }

    if player.is_using_melee() && player.is_wearing("Gadderhammer") && monster.is_shade() {
        let hits1 = standard_hit_dist
            .clone()
            .scale_probability(0.95)
            .scale_damage(5.0, 4);
        let hits2 = standard_hit_dist
            .clone()
            .scale_probability(0.05)
            .scale_damage(2.0, 1);
        let mut combined_hits = Vec::new();
        combined_hits.extend(hits1.hits);
        combined_hits.extend(hits2.hits);

        dist = AttackDistribution::new(vec![HitDistribution::new(combined_hits)]);
    }

    if player.is_using_melee() && player.set_effects.full_dharoks {
        let full_hp = player.stats.hitpoints;
        let current_hp = player.live_stats.hitpoints;
        dist = dist.scale_damage(
            10000.0 + (full_hp - current_hp) as f64 * full_hp as f64,
            10000,
        );
    }

    if player.is_using_melee() && player.set_effects.full_veracs {
        let hits1 = standard_hit_dist.clone().scale_probability(0.75).hits;
        let hits2 = HitDistribution::linear(1.0, 1, max_hit + 1)
            .scale_probability(0.25)
            .hits;

        dist = dist_from_multiple_hits(vec![hits1, hits2]);
    }

    if player.combat_type() == CombatType::Ranged && player.set_effects.full_karils {
        let hits1 = standard_hit_dist.clone().scale_probability(0.75).hits;
        let hits2 = standard_hit_dist.clone().hits;
        let hits2 = hits2
            .iter()
            .map(|h| {
                WeightedHit::new(
                    h.probability * 0.25,
                    vec![h.hitsplats[0], h.hitsplats[0] / 2],
                )
            })
            .collect();

        dist = dist_from_multiple_hits(vec![hits1, hits2]);
    }

    if player.is_using_melee() && player.is_wearing("Scythe of vitur") {
        let mut hits: Vec<HitDistribution> = Vec::new();

        for i in 0..min(max(monster.info.size, 1), 3) {
            hits.push(HitDistribution::linear(
                acc,
                0,
                max_hit / (num::pow(2, i as usize)),
            ));
        }
        dist = AttackDistribution::new(hits);
    }

    if player.is_using_melee() && player.is_wearing("Dual macuahuitl") {
        let half_max = max_hit / 2;
        let first_hit = HitDistribution::linear(1.0, 0, half_max);
        let second_hit = HitDistribution::linear(acc, 0, max_hit - half_max);
        let double_hit = first_hit.transform(|h| HitDistribution::single(1.0, h).zip(&second_hit));

        let mut effect_dist = double_hit.scale_probability(acc);
        effect_dist.add_hit(WeightedHit::new(1.0 - acc, vec![0, 0]));

        dist = AttackDistribution::new(vec![effect_dist]);
    }

    if player.is_using_melee() && player.is_wearing_keris() && monster.is_kalphite() {
        let hits1 = standard_hit_dist
            .clone()
            .scale_probability(50.0 / 51.0)
            .hits;
        let hits2 = standard_hit_dist
            .clone()
            .scale_probability(1.0 / 51.0)
            .scale_damage(3.0, 1)
            .hits;

        dist = dist_from_multiple_hits(vec![hits1, hits2]);
    }

    if player.is_using_melee()
        && monster.info.name.contains("Guardian (Chambers")
        && player.gear.weapon.name.contains("pickaxe")
    {
        let pick_bonus = PICKAXE_BONUSES
            .iter()
            .find(|b| b.0 == player.gear.weapon.name)
            .unwrap_or_else(|| panic!("No pickaxe bonus for {}", player.gear.weapon.name))
            .1;

        let factor = 50 + player.stats.mining + pick_bonus;
        let divisor = 150;

        dist = dist.transform(multiply_transformer(factor, divisor, 0))
    }

    if monster.info.name.contains("Ice demon") && player.is_using_fire_spell()
        || player.attrs.spell == Some(Spell::Standard(StandardSpell::FlamesOfZamorak))
    {
        dist = dist.scale_damage(3.0, 2);
    }

    if player.combat_type() == CombatType::Magic && player.set_effects.full_ahrims {
        let hits1 = standard_hit_dist.clone().scale_probability(0.75).hits;
        let hits2 = standard_hit_dist
            .clone()
            .scale_probability(0.25)
            .scale_damage(13.0, 10)
            .hits;

        dist = dist_from_multiple_hits(vec![hits1, hits2]);
    }

    if player.combat_type() == CombatType::Ranged && player.is_using_crossbow() {
        let zcb = player.is_wearing("Zaryte crossbow");
        let ranged_lvl = player.live_stats.ranged;
        let kandarin = if player.boosts.kandarin_diary {
            1.1
        } else {
            1.0
        };

        if player.is_wearing_any(vec!["Opal bolts (e)", "Opal dragon bolts (e)"]) {
            let chance = OPAL_PROC_CHANCE * kandarin;
            let bonus_dmg = ranged_lvl / (if zcb { 9 } else { 10 });

            dist = dist.transform(|h| {
                HitDistribution::new(vec![
                    WeightedHit::new(chance, vec![h + bonus_dmg]),
                    WeightedHit::new(1.0 - chance, vec![h]),
                ])
            });
        }

        if player.is_wearing_any(vec!["Pearl bolts (e)", "Pearl dragon bolts (e)"]) {
            let chance = PEARL_PROC_CHANCE * kandarin;
            let divisor = if monster.is_fiery() { 15 } else { 20 };
            let bonus_dmg = ranged_lvl / (if zcb { divisor - 2 } else { divisor });

            dist = dist.transform(|h| {
                HitDistribution::new(vec![
                    WeightedHit::new(chance, vec![h + bonus_dmg]),
                    WeightedHit::new(1.0 - chance, vec![h]),
                ])
            });
        }

        if player.is_wearing_any(vec!["Diamond bolts (e)", "Diamond dragon bolts (e)"]) {
            let chance = DIAMOND_PROC_CHANCE * kandarin;
            let effect_max = max_hit + max_hit * (if zcb { 26 } else { 15 } / 100);

            let hits1 = standard_hit_dist
                .clone()
                .scale_probability(1.0 - chance)
                .hits;
            let hits2 = HitDistribution::linear(1.0, 0, effect_max)
                .scale_probability(chance)
                .hits;

            dist = dist_from_multiple_hits(vec![hits1, hits2]);
        }

        if player.is_wearing_any(vec![
            "Dragonstone bolts (e)",
            "Dragonstone dragon bolts (e)",
        ]) && (!monster.is_fiery() || !monster.is_dragon())
        {
            let chance = DRAGONSTONE_PROC_CHANCE * kandarin;
            let bonus_dmg = ranged_lvl * 2 / (if zcb { 9 } else { 10 });

            let hits1 = standard_hit_dist
                .clone()
                .scale_probability(1.0 - chance)
                .hits;
            let hits2 = HitDistribution::linear(acc, bonus_dmg, max_hit + bonus_dmg)
                .scale_probability(chance)
                .hits;

            dist = dist_from_multiple_hits(vec![hits1, hits2]);
        }

        if player.is_wearing_any(vec!["Onyx bolts (e)", "Onyx dragon bolts (e)"]) {
            let chance = ONYX_PROC_CHANCE * kandarin;
            let effect_max = max_hit + ranged_lvl * (if zcb { 32 } else { 20 } / 100);

            let hits1 = standard_hit_dist
                .clone()
                .scale_probability(1.0 - chance)
                .hits;
            let hits2 = HitDistribution::linear(1.0, 0, effect_max)
                .scale_probability(acc * chance)
                .hits;
            let hits3 = vec![WeightedHit::new((1.0 - acc) * chance, vec![0])];

            dist = dist_from_multiple_hits(vec![hits1, hits2, hits3]);
        }

        if monster.info.name.as_str() == "Corporeal Beast" && !player.is_using_corpbane_weapon() {
            dist = dist.transform(division_transformer(2, 0));
        }

        if player.combat_type() == CombatType::Ranged
            && player.is_using_crossbow()
            && player.is_wearing_any(vec!["Ruby bolts (e)", "Ruby dragon bolts (e)"])
        {
            let chance = RUBY_PROC_CHANCE * kandarin;
            let effect_dmg = if zcb {
                min(110, monster.live_stats.hitpoints * 22 / 100)
            } else {
                min(100, monster.live_stats.hitpoints / 5)
            };
            let hits1 = dist.clone().dists[0].scale_probability(1.0 - chance).hits;
            let hits2 = vec![WeightedHit::new(chance, vec![effect_dmg])];

            dist = dist_from_multiple_hits(vec![hits1, hits2]);
        }
    }

    apply_limiters(dist, player, monster)
}

fn apply_limiters(
    dist: AttackDistribution,
    player: &Player,
    monster: &Monster,
) -> AttackDistribution {
    if monster.is_immune(player) {
        return AttackDistribution::new(vec![HitDistribution::new(vec![WeightedHit::new(
            1.0,
            vec![0],
        )])]);
    }

    let mut dist = dist;

    if monster.info.name.contains("Zulrah") {
        dist = dist.transform(capped_reroll_transformer(50, 5, 45));
    }

    if monster.info.name.contains("Fragment of Seren") {
        dist = dist.transform(linear_min_transformer(2, 22));
    }

    if monster.info.name.as_str() == "Kraken (Normal)" && player.combat_type() == CombatType::Ranged
    {
        dist = dist.transform(division_transformer(7, 1));
    }

    if monster.info.name.contains("Verzik")
        && monster.info.name.contains("P1")
        && !player.is_wearing("Dawnbringer")
    {
        let limit = if player.is_using_melee() { 10 } else { 3 };
        dist = dist.transform(linear_min_transformer(limit, 0));
    }

    if monster.info.name.contains("Tekton") && player.combat_type() == CombatType::Magic {
        dist = dist.transform(division_transformer(5, 1));
    }

    if monster.info.name.contains("Glowing crystal") && player.combat_type() == CombatType::Magic {
        dist = dist.transform(division_transformer(3, 0));
    }

    if monster.info.name.contains("(Left claw")
        || monster.info.name.contains("Great Olm (Head")
            && player.combat_type() == CombatType::Magic
    {
        dist = dist.transform(division_transformer(3, 0));
    }

    if monster.info.name.contains("(Right claw")
        || monster.info.name.contains("Left claw") && player.combat_type() == CombatType::Ranged
    {
        dist = dist.transform(division_transformer(3, 0));
    }

    // TODO: Implement updated Efaritay's aid here once wiki calc does

    if monster.info.name.contains("Ice demon")
        && !player.is_using_fire_spell()
        && player.attrs.spell != Some(Spell::Standard(StandardSpell::FlamesOfZamorak))
    {
        dist = dist.transform(division_transformer(3, 0));
    }

    if monster.info.name.contains("Slagilith") && !player.gear.weapon.name.contains("pickaxe") {
        dist = dist.transform(division_transformer(3, 0));
    }

    if ["Slash Bash", "Zogre", "Skogre"].contains(&monster.info.name.as_str()) {
        if player.attrs.spell == Some(Spell::Standard(StandardSpell::CrumbleUndead)) {
            dist = dist.transform(division_transformer(2, 0));
        } else if player.combat_type() != CombatType::Ranged
            || !player
                .gear
                .ammo
                .as_ref()
                .map_or(false, |ammo| ammo.name.contains(" brutal"))
            || !player.gear.weapon.name.contains("Comp ogre bow")
        {
            dist = dist.transform(division_transformer(4, 0));
        }
    }

    dist
}

fn get_dpt(dist: AttackDistribution, player: &Player) -> f64 {
    dist.get_expected_damage() / player.gear.weapon.speed as f64
}

pub fn get_dps(dist: AttackDistribution, player: &Player) -> f64 {
    get_dpt(dist, player) / SECONDS_PER_TICK
}

fn get_htk(dist: AttackDistribution, monster: &Monster) -> f64 {
    let mut dist = dist;
    let hist = dist.as_histogram();
    let start_hp = monster.live_stats.hitpoints as usize;
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

pub fn get_ttk(dist: AttackDistribution, player: &Player, monster: &Monster) -> f64 {
    get_htk(dist, monster) * player.gear.weapon.speed as f64 * SECONDS_PER_TICK
}

pub fn get_ttk_distribution(
    dist: &mut AttackDistribution,
    player: &Player,
    monster: &Monster,
) -> HashMap<usize, f64> {
    let speed = player.gear.weapon.speed as usize;
    let max_hp = monster.stats.hitpoints as usize;
    let mut dist_temp = dist.clone();
    let dist_single = dist_temp.get_single_hitsplat();

    if dist_single.expected_hit() == 0.0 {
        return HashMap::new();
    }

    let mut hps = vec![0.0; max_hp + 1];
    hps[max_hp] = 1.0;

    let mut ttks: HashMap<usize, f64> = HashMap::new();

    let mut epsilon = 1.0;

    let recalc_dist_on_hp = dist_is_current_hp_dependent(player, monster);
    let mut hp_hit_dists = HashMap::new();
    hp_hit_dists.insert(max_hp, dist_single.clone());
    if recalc_dist_on_hp {
        for hp in 0..max_hp {
            dist_at_hp(dist, hp, player, monster, &mut hp_hit_dists);
        }
    }

    for hit in 0..=TTK_DIST_MAX_ITER_ROUNDS {
        if epsilon >= TTK_DIST_EPSILON {
            break;
        }
        let mut next_hps = vec![0.0; max_hp + 1];
        for (hp, hp_prob) in hps.iter().enumerate() {
            let current_dist = if recalc_dist_on_hp {
                hp_hit_dists.get(&hp).unwrap()
            } else {
                dist_single
            };

            for h in &current_dist.hits {
                let dmg_prob = h.probability;
                let dmg = h.hitsplats[0] as usize;

                let chance_of_action = dmg_prob * hp_prob;

                if chance_of_action == 0.0 {
                    continue;
                }

                let new_hp = hp as i32 - dmg as i32;
                if new_hp <= 0 {
                    let tick = hit * speed + 1;
                    ttks.insert(tick, ttks.get(&tick).unwrap_or(&0.0) + chance_of_action);
                    epsilon -= chance_of_action;
                } else {
                    next_hps[new_hp as usize] += chance_of_action;
                }
            }
        }

        hps = next_hps;
    }

    ttks
}
fn dist_from_multiple_hits(hits_vec: Vec<Vec<WeightedHit>>) -> AttackDistribution {
    let mut combined_hits = Vec::new();
    for hits in hits_vec {
        combined_hits.extend(hits);
    }
    AttackDistribution::new(vec![HitDistribution::new(combined_hits)])
}

fn dist_is_current_hp_dependent(player: &Player, monster: &Monster) -> bool {
    if monster.info.name.contains("Vardorvis") {
        return true;
    }

    if player.is_using_crossbow()
        && player.is_wearing_any(vec!["Ruby bolts (e)", "Ruby dragon bolts (e)"])
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
) {
    let no_scaling = dist.get_single_hitsplat();
    if !dist_is_current_hp_dependent(player, monster) || hp == monster.live_stats.hitpoints as usize
    {
        hp_hit_dists.insert(hp, no_scaling.clone());
        return;
    }

    if player.combat_type() == CombatType::Ranged
        && player.is_using_crossbow()
        && player.is_wearing_any(vec!["Ruby bolts (e)", "Ruby dragon bolts (e)"])
        && monster.live_stats.hitpoints >= 500
        && hp >= 500
    {
        hp_hit_dists.insert(hp, no_scaling.clone());
        return;
    }

    let mut monster_copy = monster.clone();

    monster_scaling::scale_monster_hp_only(&mut monster_copy);

    let mut new_dist = get_distribution(player, &monster_copy);

    hp_hit_dists.insert(hp, new_dist.get_single_hitsplat().clone());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::equipment::{Armor, CombatStyle, Weapon};
    use crate::monster::Monster;
    use crate::player::{Gear, Player, PlayerStats};
    use crate::potions::Potion;
    use crate::prayers::{Prayer, PrayerBoost};
    use crate::rolls::{calc_player_melee_rolls, calc_player_ranged_rolls};
    #[test]
    fn test_max_melee_ammonite_crab() {
        let mut player = Player::new();
        player.stats = PlayerStats::default();
        player.prayers.add(PrayerBoost::new(Prayer::Piety));
        player.add_potion(Potion::SuperCombat);

        player.gear = Gear {
            head: Some(Armor::new("Torva full helm")),
            neck: Some(Armor::new("Amulet of torture")),
            cape: Some(Armor::new("Infernal cape")),
            ammo: Some(Armor::new("Rada's blessing 4")),
            second_ammo: None,
            weapon: Weapon::new("Ghrazi rapier"),
            shield: Some(Armor::new("Avernic defender")),
            body: Some(Armor::new("Torva platebody")),
            legs: Some(Armor::new("Torva platelegs")),
            hands: Some(Armor::new("Ferocious gloves")),
            feet: Some(Armor::new("Primordial boots")),
            ring: Some(Armor::new("Ultor ring")),
        };
        player.update_bonuses();
        player.set_active_style(CombatStyle::Lunge);
        let monster = Monster::new("Ammonite Crab").unwrap();
        calc_player_melee_rolls(&mut player, &monster);

        let dist = get_distribution(&player, &monster);
        let ttk = get_ttk(dist, &player, &monster);

        assert!(num::abs(ttk - 10.2) < 0.1);
    }

    #[test]
    fn test_macuahuitl() {
        let mut player = Player::new();
        player.stats = PlayerStats::default();
        player.prayers.add(PrayerBoost::new(Prayer::Piety));
        player.add_potion(Potion::SuperCombat);

        player.gear = Gear {
            head: Some(Armor::new("Torva full helm")),
            neck: Some(Armor::new("Amulet of torture")),
            cape: Some(Armor::new("Infernal cape")),
            ammo: Some(Armor::new("Rada's blessing 4")),
            second_ammo: None,
            weapon: Weapon::new("Dual macuahuitl"),
            shield: None,
            body: Some(Armor::new("Torva platebody")),
            legs: Some(Armor::new("Torva platelegs")),
            hands: Some(Armor::new("Ferocious gloves")),
            feet: Some(Armor::new("Primordial boots")),
            ring: Some(Armor::new("Ultor ring")),
        };
        player.update_bonuses();
        player.set_active_style(CombatStyle::Pummel);

        let monster = Monster::new("Vet'ion (Normal)").unwrap();
        calc_player_melee_rolls(&mut player, &monster);
        let dist = get_distribution(&player, &monster);
        let ttk = get_ttk(dist, &player, &monster);

        assert!(num::abs(ttk - 44.3) < 0.1);
    }

    #[test]
    fn test_scythe_vardorvis() {
        let mut player = Player::new();
        player.stats = PlayerStats::default();
        player.prayers.add(PrayerBoost::new(Prayer::Piety));
        player.add_potion(Potion::SuperCombat);

        player.gear = Gear {
            head: Some(Armor::new("Torva full helm")),
            neck: Some(Armor::new("Amulet of torture")),
            cape: Some(Armor::new("Infernal cape")),
            ammo: Some(Armor::new("Rada's blessing 4")),
            second_ammo: None,
            weapon: Weapon::new("Scythe of vitur"),
            shield: None,
            body: Some(Armor::new("Torva platebody")),
            legs: Some(Armor::new("Torva platelegs")),
            hands: Some(Armor::new("Ferocious gloves")),
            feet: Some(Armor::new("Primordial boots")),
            ring: Some(Armor::new("Ultor ring")),
        };
        player.update_bonuses();
        player.set_active_style(CombatStyle::Chop);

        let monster = Monster::new("Vardorvis (Post-Quest)").unwrap();
        calc_player_melee_rolls(&mut player, &monster);
        let dist = get_distribution(&player, &monster);
        let ttk = get_ttk(dist, &player, &monster);

        assert!(num::abs(ttk - 100.7) < 0.1);
    }

    #[test]
    fn test_ruby_bolts_zcb_zebak_500() {
        let mut player = Player::new();
        player.stats = PlayerStats::default();
        player.prayers.add(PrayerBoost::new(Prayer::Rigour));
        player.add_potion(Potion::SmellingSalts);

        player.gear = Gear {
            head: Some(Armor::new("Masori mask (f)")),
            neck: Some(Armor::new("Necklace of anguish")),
            cape: Some(Armor::new("Dizana's quiver (charged)")),
            ammo: Some(Armor::new("Ruby dragon bolts (e)")),
            second_ammo: None,
            weapon: Weapon::new("Zaryte crossbow"),
            shield: Some(Armor::new("Twisted buckler")),
            body: Some(Armor::new("Masori body (f)")),
            legs: Some(Armor::new("Masori chaps (f)")),
            hands: Some(Armor::new("Zaryte vambraces")),
            feet: Some(Armor::new("Pegasian boots")),
            ring: Some(Armor::new("Venator ring")),
        };

        player.update_bonuses();
        player.set_active_style(CombatStyle::Rapid);

        let mut monster = Monster::new("Zebak").unwrap();
        monster.info.toa_level = 500;
        monster.scale_toa();
        calc_player_ranged_rolls(&mut player, &monster);

        let dist = get_distribution(&player, &monster);
        let ttk = get_ttk(dist, &player, &monster);

        assert!(num::abs(ttk - 225.3) < 0.1);
    }
}
