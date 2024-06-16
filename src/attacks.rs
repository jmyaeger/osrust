use crate::constants::*;
use crate::equipment::CombatType;
use crate::limiters::Limiter;
use crate::monster::Monster;
use crate::player::Player;
use crate::rolls::calc_player_melee_rolls;
use crate::spells::{AncientSpell, Spell};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::cmp::max;

pub fn standard_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    // Default attack method for most weapons

    // Determine max attack, defense, and damage rolls
    let combat_type = player.combat_type();
    let max_att_roll = player.att_rolls[&combat_type];
    let mut max_def_roll = monster.def_rolls[&combat_type];
    let max_hit = player.max_hits[&combat_type];
    let min_hit = if combat_type == CombatType::Magic
        && player.boosts.sunfire.active
        && player.is_using_fire_spell()
    {
        player.boosts.sunfire.min_hit
    } else {
        0
    };

    // Roll for brimstone ring effect if applicable
    if combat_type == CombatType::Magic
        && player.is_wearing("Brimstone ring", None)
        && rng.gen_range(0..4) == 0
    {
        max_def_roll = max_def_roll * 9 / 10;
    }

    let (mut damage, success) = base_attack(max_att_roll, max_def_roll, min_hit, max_hit, rng);

    if success {
        // Transform any accurate zeros into 1s, then apply post-roll transforms (TODO: verify this order)
        damage = max(damage, 1);
        damage = apply_flat_armour_and_limiters(damage, monster, rng, limiter);
    }

    (damage, success)
}

fn base_attack(
    max_att_roll: i32,
    max_def_roll: i32,
    min_hit: u32,
    max_hit: u32,
    rng: &mut ThreadRng,
) -> (u32, bool) {
    let att_roll = accuracy_roll(max_att_roll, rng);
    let def_roll = defence_roll(max_def_roll, rng);

    // Roll for accuracy
    let success = att_roll > def_roll;

    // Roll for damage if successful
    let mut damage = 0;
    if success {
        damage = damage_roll(min_hit, max_hit, rng);
    }

    (damage, success)
}

fn accuracy_roll(max_att_roll: i32, rng: &mut ThreadRng) -> i32 {
    rng.gen_range(0..=max_att_roll)
}

fn defence_roll(max_def_roll: i32, rng: &mut ThreadRng) -> i32 {
    rng.gen_range(0..=max_def_roll)
}

fn damage_roll(min_hit: u32, max_hit: u32, rng: &mut ThreadRng) -> u32 {
    rng.gen_range(min_hit..=max_hit)
}

pub fn fang_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let combat_type = player.combat_type();
    let max_att_roll = player.att_rolls[&combat_type];
    let max_def_roll = monster.def_rolls[&combat_type];
    let true_max_hit = player.max_hits[&combat_type];

    // Fang rolls from 15% of max hit to max_hit - 15%
    let min_hit = true_max_hit * 15 / 100;
    let max_hit = true_max_hit - min_hit;

    let att_roll1 = accuracy_roll(max_att_roll, rng);
    let def_roll1 = defence_roll(max_def_roll, rng);

    let (mut damage, success) = if att_roll1 > def_roll1 {
        // Skip second roll if first roll was successful
        (damage_roll(min_hit, max_hit, rng), true)
    } else {
        let att_roll2 = accuracy_roll(max_att_roll, rng);

        // Only re-roll defense if in ToA
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

    if success {
        // No accurate zeros, so no need to do anything before applying post-roll transforms
        damage = apply_flat_armour_and_limiters(damage, monster, rng, limiter);
    }

    (damage, success)
}
pub fn ahrims_staff_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let combat_type = player.combat_type();

    // Do a standard attack if not using magic or the full ahrim's set
    if combat_type != CombatType::Magic || !player.set_effects.full_ahrims {
        return standard_attack(player, monster, rng, limiter);
    }

    let max_att_roll = player.att_rolls[&combat_type];
    let max_def_roll = monster.def_rolls[&combat_type];
    let max_hit = player.max_hits[&combat_type];
    let min_hit = if player.combat_type() == CombatType::Magic
        && player.is_using_fire_spell()
        && player.boosts.sunfire.active
    {
        player.boosts.sunfire.min_hit
    } else {
        0
    };

    let (mut damage, success) = base_attack(max_att_roll, max_def_roll, min_hit, max_hit, rng);

    if success && rng.gen_range(0..4) == 0 {
        // Base set effect rolls a 25% chance to reduce strength by 5
        monster.drain_strength(5);
    }

    if player.is_wearing_any_version("Amulet of the damned") && rng.gen_range(0..4) == 0 {
        // With amulet of the damned, 25% chance to increase damage 30% post-roll
        damage = damage * 13 / 10;
    }

    if success {
        damage = max(damage, 1);
        // Unconfirmed if the post-roll multiplier comes before or after limiters
        damage = apply_flat_armour_and_limiters(damage, monster, rng, limiter);
    }

    (damage, success)
}

pub fn dharoks_axe_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let combat_type = player.combat_type();
    let max_att_roll = player.att_rolls[&combat_type];
    let max_def_roll = monster.def_rolls[&combat_type];
    let max_hit = player.max_hits[&combat_type];

    let (mut damage, success) = base_attack(max_att_roll, max_def_roll, 0, max_hit, rng);

    if success && player.set_effects.full_dharoks {
        // Set effect damage increase is applied post-roll
        let max_hp = player.stats.hitpoints;
        let current_hp = player.live_stats.hitpoints;
        let dmg_mod = 10000 + (max_hp.saturating_sub(current_hp)) * max_hp;
        damage = damage * dmg_mod / 10000;
    }

    if success {
        damage = max(damage, 1);
        damage = apply_flat_armour_and_limiters(damage, monster, rng, limiter);
    }

    (damage, success)
}

pub fn veracs_flail_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let combat_type = player.combat_type();
    if player.set_effects.full_veracs && rng.gen_range(0..4) == 0 {
        // Set effect rolls 25% chance to guarantee hit (minimum 1 damage)
        let mut damage = 1 + damage_roll(1, player.max_hits[&combat_type] + 1, rng);
        damage = apply_flat_armour_and_limiters(damage, monster, rng, limiter);
        (damage, true)
    } else {
        standard_attack(player, monster, rng, limiter)
    }
}

pub fn karils_crossbow_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    if player.set_effects.full_karils
        && player.is_wearing_any_version("Amulet of the damned")
        && rng.gen_range(0..4) == 0
    {
        // Set effect rolls 25% chance to hit an additional time for half the first hit's damage
        let (hit1, success) = standard_attack(player, monster, rng, limiter);
        let hit2 = hit1 / 2;
        (hit1 + hit2, success)
    } else {
        standard_attack(player, monster, rng, limiter)
    }
}

pub fn guthans_warspear_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let (damage, success) = standard_attack(player, monster, rng, limiter);
    if player.set_effects.full_guthans && rng.gen_range(0..4) == 0 {
        // Set effect rolls 25% chance to heal by the damage dealt
        if player.is_wearing_any_version("Amulet of the damned") {
            // Amulet of the damned allows up to 10 HP of overheal
            player.heal(damage, Some(10));
        } else {
            player.heal(damage, None);
        }
    }

    (damage, success)
}

pub fn torags_hammers_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let combat_type = player.combat_type();
    let max_hit = player.max_hits[&combat_type];
    let max_hit1 = max_hit / 2;
    let max_hit2 = max_hit - max_hit1;
    let max_att_roll = player.att_rolls[&combat_type];
    let max_def_roll = monster.def_rolls[&combat_type];

    // Hammers attack with two independently rolled hits (tested in-game)
    let (mut damage1, success1) = base_attack(max_att_roll, max_def_roll, 0, max_hit1, rng);
    let (mut damage2, success2) = base_attack(max_att_roll, max_def_roll, 0, max_hit2, rng);

    // Not implementing the normal set effect because it only applies in PvP
    // Amulet of the damned effect gets implemented in roll calcs

    if success1 {
        damage1 = max(damage1, 1);
        damage1 = apply_flat_armour_and_limiters(damage1, monster, rng, limiter);
    }

    if success2 {
        damage2 = max(damage2, 1);
        damage2 = apply_flat_armour_and_limiters(damage2, monster, rng, limiter);
    }

    (damage1 + damage2, success1 || success2)
}

pub fn sang_staff_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let (damage, success) = standard_attack(player, monster, rng, limiter);
    if rng.gen_range(0..6) == 0 {
        // 1/6 chance to heal by half of the damage dealt
        player.heal(damage / 2, None)
    }

    (damage, success)
}

pub fn dawnbringer_attack(
    player: &mut Player,
    _: &mut Monster,
    rng: &mut ThreadRng,
    _: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let max_hit = player.max_hits[&player.combat_type()];

    // Guaranteed to hit because it can only be used on Verzik
    let mut damage = damage_roll(0, max_hit, rng);
    damage = max(1, damage);
    (damage, true)
}

pub fn keris_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let (mut damage, success) = standard_attack(player, monster, rng, limiter);

    // 1/51 chance to deal triple damage (post-roll)
    if monster.is_kalphite() && rng.gen_range(0..51) == 0 {
        damage *= 3;
    }

    (damage, success)
}
pub fn yellow_keris_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let combat_type = player.combat_type();
    let max_hit = player.max_hits[&combat_type];
    let mut max_att_roll = player.att_rolls[&combat_type];
    let max_def_roll = monster.def_rolls[&combat_type];

    if (monster.live_stats.hitpoints as f32) / (monster.stats.hitpoints as f32) < 0.25
        && monster.is_toa_monster()
    {
        // In ToA, accuracy is boosted by 25% when monster is below 25% health
        max_att_roll = max_att_roll * 5 / 4;
    }

    let (mut damage, success) = base_attack(max_att_roll, max_def_roll, 0, max_hit, rng);

    if monster.live_stats.hitpoints.saturating_sub(damage) == 0 && monster.is_toa_monster() {
        // Killing a ToA monster heals the player by 12 and costs 5 prayer points
        player.heal(12, Some(player.stats.hitpoints / 5));
        player.live_stats.prayer = player.live_stats.prayer.saturating_sub(5);
    }

    if success {
        damage = max(damage, 1);
        damage = apply_flat_armour_and_limiters(damage, monster, rng, limiter);
    }

    (damage, success)
}

pub fn opal_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let mut proc_chance = OPAL_PROC_CHANCE;
    if player.boosts.kandarin_diary {
        proc_chance *= 1.1;
    }

    let extra_damage = if player.is_wearing("Zaryte crossbow", None) {
        player.live_stats.ranged / 9
    } else {
        player.live_stats.ranged / 10
    };

    let max_hit = player.max_hits[&player.combat_type()];

    // Guaranteed hit if the bolt effect procs (verified in-game)
    if rng.gen::<f64>() <= proc_chance {
        // Bolt effect adds on flat damage based on visible ranged level
        let mut damage = damage_roll(0, max_hit, rng) + extra_damage;
        damage = max(damage, 1); // Probably not necessary but just in case
        damage = apply_flat_armour_and_limiters(damage, monster, rng, limiter);
        (damage, true)
    } else {
        standard_attack(player, monster, rng, limiter)
    }
}

pub fn pearl_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let mut proc_chance = PEARL_PROC_CHANCE;
    if player.boosts.kandarin_diary {
        proc_chance *= 1.1;
    }

    // Bolt effect is extra effective against fiery monsters
    let mut denominator = if monster.is_fiery() { 15 } else { 20 };

    if player.is_wearing("Zaryte crossbow", None) {
        denominator = denominator * 9 / 10;
    }
    let extra_damage = player.live_stats.ranged / denominator;

    let max_hit = player.max_hits[&player.combat_type()];

    // Same implementation as opal bolts (accurate hit on procs, flat damage added)
    if rng.gen::<f64>() <= proc_chance {
        let mut damage = damage_roll(0, max_hit, rng) + extra_damage;
        damage = max(damage, 1);
        damage = apply_flat_armour_and_limiters(damage, monster, rng, limiter);
        (damage, true)
    } else {
        standard_attack(player, monster, rng, limiter)
    }
}

pub fn emerald_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let mut proc_chance = EMERALD_PROC_CHANCE;
    if player.boosts.kandarin_diary {
        proc_chance *= 1.1;
    }

    let poison_severity = if player.is_wearing("Zaryte crossbow", None) {
        27
    } else {
        25
    };

    let (damage, success) = standard_attack(player, monster, rng, limiter);

    if success && rng.gen::<f64>() <= proc_chance {
        // TODO: Change this to use a CombatEffect
        monster.info.poison_severity = poison_severity;
    }

    (damage, success)
}

pub fn ruby_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let mut proc_chance = RUBY_PROC_CHANCE;
    if player.boosts.kandarin_diary {
        proc_chance *= 1.1;
    }

    let ruby_damage = if player.is_wearing("Zaryte crossbow", None) {
        // Verified to be 22/100, not 2/9
        (monster.live_stats.hitpoints * 22 / 100).clamp(1, 110)
    } else {
        (monster.live_stats.hitpoints / 5).clamp(1, 100)
    };

    if rng.gen::<f64>() <= proc_chance {
        // Bolt proc ignores defense and deals fixed amount of damage
        player.take_damage(player.live_stats.hitpoints / 10);
        let mut damage = if limiter.is_some() && !monster.info.name.contains("Corporeal Beast") {
            limiter.as_ref().unwrap().apply(ruby_damage, rng)
        } else {
            ruby_damage
        };

        // TODO: Test how this interacts with flat armour
        if monster.bonuses.flat_armour > 0 {
            damage = max(1, damage.saturating_sub(monster.bonuses.flat_armour));
        }

        (damage, true)
    } else {
        standard_attack(player, monster, rng, limiter)
    }
}

pub fn diamond_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let mut proc_chance = DIAMOND_PROC_CHANCE;
    if player.boosts.kandarin_diary {
        proc_chance *= 1.1;
    }

    let base_max_hit = player.max_hits[&player.combat_type()];
    let max_hit = if player.is_wearing("Zaryte crossbow", None) {
        base_max_hit * 126 / 100
    } else {
        base_max_hit * 115 / 100
    };

    if rng.gen::<f64>() <= proc_chance {
        // Bolt proc ignores defense and boosts max hit
        let mut damage = damage_roll(0, max_hit, rng);
        damage = max(damage, 1);
        damage = apply_flat_armour_and_limiters(damage, monster, rng, limiter);
        (damage, true)
    } else {
        standard_attack(player, monster, rng, limiter)
    }
}

pub fn onyx_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let mut proc_chance = ONYX_PROC_CHANCE;
    if player.boosts.kandarin_diary {
        proc_chance *= 1.1;
    }

    let base_max_hit = player.max_hits[&player.combat_type()];
    let max_hit = if player.is_wearing("Zaryte crossbow", None) {
        base_max_hit * 132 / 100
    } else {
        base_max_hit * 6 / 5
    };

    let (mut damage, success) = standard_attack(player, monster, rng, limiter);

    if success && !monster.is_undead() && rng.gen::<f64>() <= proc_chance {
        // Bolt proc boosts max hit but does not ignore defense
        damage = damage_roll(0, max_hit, rng);
        damage = max(damage, 1);
        damage = apply_flat_armour_and_limiters(damage, monster, rng, limiter);

        // Heal the player by 1/4 of the damage
        player.heal(damage / 4, None);
    }

    (damage, success)
}

pub fn dragonstone_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let mut proc_chance = DRAGONSTONE_PROC_CHANCE;
    if player.boosts.kandarin_diary {
        proc_chance *= 1.1;
    }

    let extra_damage = if player.is_wearing("Zaryte crossbow", None) {
        player.live_stats.ranged * 2 / 9
    } else {
        player.live_stats.ranged / 5
    };

    let combat_type = player.combat_type();
    let max_att_roll = player.att_rolls[&combat_type];
    let max_def_roll = monster.def_rolls[&combat_type];
    let max_hit = player.max_hits[&combat_type];

    let (mut damage, success) = base_attack(max_att_roll, max_def_roll, 0, max_hit, rng);

    if success {
        // Only dragons that are also "fiery" are immune
        // Bolt proc requires accurate hit and adds flat damage
        if rng.gen::<f64>() <= proc_chance && !(monster.is_dragon() && monster.is_fiery()) {
            damage += extra_damage;
        }

        damage = max(damage, 1);
        damage = apply_flat_armour_and_limiters(damage, monster, rng, limiter);
    }

    (damage, success)
}

pub fn smoke_spell_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    // Assuming that it always applies poison if the monster is not immune
    if !monster.immunities.poison {
        monster.info.poison_severity =
            match (player.is_wearing_ancient_spectre(), player.attrs.spell) {
                (
                    true,
                    Some(Spell::Ancient(AncientSpell::SmokeRush))
                    | Some(Spell::Ancient(AncientSpell::SmokeBurst)),
                ) => 11,
                (
                    true,
                    Some(Spell::Ancient(AncientSpell::SmokeBlitz))
                    | Some(Spell::Ancient(AncientSpell::SmokeBarrage)),
                ) => 22,
                (
                    false,
                    Some(Spell::Ancient(AncientSpell::SmokeRush))
                    | Some(Spell::Ancient(AncientSpell::SmokeBurst)),
                ) => 10,
                (
                    false,
                    Some(Spell::Ancient(AncientSpell::SmokeBlitz))
                    | Some(Spell::Ancient(AncientSpell::SmokeBarrage)),
                ) => 20,
                _ => 0,
            };
    }

    standard_attack(player, monster, rng, limiter)
}

pub fn shadow_spell_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    // Drain amounts are the percentages multiplied by 1000 to avoid floating point math
    let drain_amount = match (player.is_wearing_ancient_spectre(), player.attrs.spell) {
        (
            true,
            Some(Spell::Ancient(AncientSpell::ShadowRush))
            | Some(Spell::Ancient(AncientSpell::ShadowBurst)),
        ) => 110,
        (
            true,
            Some(Spell::Ancient(AncientSpell::ShadowBlitz))
            | Some(Spell::Ancient(AncientSpell::ShadowBarrage)),
        ) => 165,
        (
            false,
            Some(Spell::Ancient(AncientSpell::ShadowRush))
            | Some(Spell::Ancient(AncientSpell::ShadowBurst)),
        ) => 100,
        (
            false,
            Some(Spell::Ancient(AncientSpell::ShadowBlitz))
            | Some(Spell::Ancient(AncientSpell::ShadowBarrage)),
        ) => 150,
        _ => 0,
    };

    let (damage, success) = standard_attack(player, monster, rng, limiter);

    if success {
        // Only drains attack if it hasn't been drained already
        if monster.live_stats.attack == monster.stats.attack {
            monster.drain_attack(monster.stats.attack * drain_amount / 1000);
        }
        if player.is_wearing("Shadow ancient sceptre", None) {
            // Shadow ancient sceptre also drains strength and defense if not drained previously
            if monster.live_stats.strength == monster.stats.strength {
                monster.drain_strength(monster.stats.strength * drain_amount / 1000);
            }
            if monster.live_stats.defence == monster.stats.defence {
                monster.drain_defence(monster.stats.defence * drain_amount / 1000);
            }
        }
    }

    (damage, success)
}

pub fn blood_spell_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    // Heal factor is (percentage of damage) * 1000
    let heal_factor = if player.is_wearing_ancient_spectre() {
        // Bloodbark pieces add 2% healing per piece
        275 + 20 * player.set_effects.bloodbark_pieces as u32
    } else {
        250 + 20 * player.set_effects.bloodbark_pieces as u32
    };

    let overheal = if player.is_wearing("Blood ancient sceptre", None) {
        // Blood ancient sceptre allows 10% overheal
        Some(player.stats.hitpoints / 10)
    } else {
        None
    };

    let (damage, success) = standard_attack(player, monster, rng, limiter);
    player.heal(damage * heal_factor / 1000, overheal);

    (damage, success)
}

pub fn ice_spell_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    if monster.is_freezable() {
        // Monster is freezable if not immune and not currently frozen or on cooldown
        let mut max_att_roll = player.att_rolls[&CombatType::Magic];
        let max_def_roll = monster.def_rolls[&CombatType::Magic];
        let max_hit = player.max_hits[&CombatType::Magic];

        if player.is_wearing("Ice ancient sceptre", None) {
            // Ice ancient sceptre is 10% more accurate on unfrozen, freezable targets
            max_att_roll = max_att_roll * 11 / 10;
        }

        let (mut damage, success) = base_attack(max_att_roll, max_def_roll, 0, max_hit, rng);

        if success {
            monster.info.freeze_duration =
                match (player.is_wearing_ancient_spectre(), player.attrs.spell) {
                    (_, Some(Spell::Ancient(AncientSpell::IceRush))) => 8,
                    (true, Some(Spell::Ancient(AncientSpell::IceBurst))) => 17,
                    (false, Some(Spell::Ancient(AncientSpell::IceBurst))) => 16,
                    (true, Some(Spell::Ancient(AncientSpell::IceBlitz))) => 26,
                    (false, Some(Spell::Ancient(AncientSpell::IceBlitz))) => 24,
                    (true, Some(Spell::Ancient(AncientSpell::IceBarrage))) => 35,
                    (false, Some(Spell::Ancient(AncientSpell::IceBarrage))) => 32,
                    _ => 0,
                };
            damage = max(damage, 1);
            damage = apply_flat_armour_and_limiters(damage, monster, rng, limiter);
        }

        (damage, success)
    } else {
        standard_attack(player, monster, rng, limiter)
    }
}

pub fn scythe_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let combat_type = player.combat_type();
    let max_att_roll = player.att_rolls[&combat_type];
    let max_def_roll = monster.def_rolls[&combat_type];
    let max_hit = player.max_hits[&combat_type];

    let (damage1, success1) = standard_attack(player, monster, rng, limiter);
    if monster.info.size == 1 {
        // standard_attack already applies post-roll transforms, so it's not needed here
        return (damage1, success1);
    }

    let (mut damage2, success2) = base_attack(max_att_roll, max_def_roll, 0, max_hit / 2, rng);
    if success2 {
        damage2 = max(damage2, 1);
        damage2 = apply_flat_armour_and_limiters(damage2, monster, rng, limiter);
    }
    if monster.info.size == 2 {
        return (damage1 + damage2, success1 | success2);
    }

    let (mut damage3, success3) = base_attack(max_att_roll, max_def_roll, 0, max_hit / 4, rng);
    if success3 {
        damage3 = max(damage3, 1);
        damage3 = apply_flat_armour_and_limiters(damage3, monster, rng, limiter);
    }
    (
        damage1 + damage2 + damage3,
        success1 || success2 || success3,
    )
}

pub fn soulreaper_axe_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let (damage, success) = standard_attack(player, monster, rng, limiter);

    if player.boosts.soulreaper_stacks < 5 && player.live_stats.hitpoints > 8 {
        // Add a soulreaper stack if the player has less than 5 stacks and can survive the self-damage
        player.take_damage(SOULREAPER_STACK_DAMAGE);
        player.boosts.soulreaper_stacks += 1;

        // Recalculate melee rolls with stack boost added
        calc_player_melee_rolls(player, monster);
    }

    (damage, success)
}

pub fn gadderhammer_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let (mut damage, success) = standard_attack(player, monster, rng, limiter);

    if success && monster.is_shade() {
        // 25% damage boost with 5% chance to double unboosted damage on shades (all post-roll)
        if rng.gen_range(0..20) == 0 {
            damage *= 2;
        } else {
            damage = damage * 5 / 4;
        }
    }

    (damage, success)
}

pub fn tonalztics_of_ralos_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let combat_type = player.combat_type();
    let max_att_roll = player.att_rolls[&combat_type];
    let max_def_roll = monster.def_rolls[&combat_type];

    // Rolls up to 3/4 of the "true" max hit for each hit
    let max_hit = player.max_hits[&combat_type] * 3 / 4;

    let (mut damage1, success1) = base_attack(max_att_roll, max_def_roll, 0, max_hit, rng);
    if success1 {
        damage1 = max(damage1, 1);
        damage1 = apply_flat_armour_and_limiters(damage1, monster, rng, limiter);
    }
    if player.gear.weapon.matches_version("Charged") {
        // Only the charged version does a second attack
        let (mut damage2, success2) = base_attack(max_att_roll, max_def_roll, 0, max_hit, rng);
        if success2 {
            damage2 = max(damage2, 1);
            damage2 = apply_flat_armour_and_limiters(damage2, monster, rng, limiter);
        }
        return (damage1 + damage2, success1 || success2);
    }

    (damage1, success1)
}

pub fn dual_macuahuitl_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let combat_type = player.combat_type();
    let max_att_roll = player.att_rolls[&combat_type];
    let max_def_roll = monster.def_rolls[&combat_type];
    let max_hit = player.max_hits[&combat_type];

    // Reset attack speed to 4 ticks
    player.gear.weapon.speed = 4;

    let max_hit1 = max_hit / 2;
    let max_hit2 = max_hit - max_hit1;

    // Roll two separate hits
    let (mut damage1, success1) = base_attack(max_att_roll, max_def_roll, 0, max_hit1, rng);
    if success1 {
        damage1 = max(damage1, 1);
        damage1 = apply_flat_armour_and_limiters(damage1, monster, rng, limiter);
    }
    let (mut damage2, success2) = if success1 {
        // Only roll the second hit if the first hit was accurate
        base_attack(max_att_roll, max_def_roll, 0, max_hit2, rng)
    } else {
        (0, false)
    };

    if success2 {
        damage2 = max(damage2, 1);
        damage2 = apply_flat_armour_and_limiters(damage2, monster, rng, limiter);
    }

    // Roll 33% chance for next attack to be one tick faster if the full set is equipped
    if player.set_effects.full_blood_moon
        && ((success1 && rng.gen_range(0..3) == 0) || (success2 && rng.gen_range(0..3) == 0))
    {
        player.gear.weapon.speed = 3;
    }

    (damage1 + damage2, success1)
}

pub fn atlatl_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> (u32, bool) {
    let (damage, success) = standard_attack(player, monster, rng, limiter);
    if success && player.set_effects.full_eclipse_moon && rng.gen_range(0..5) == 0 {
        // Roll 20% chance to add a burn stack if full set is equipped
        monster.add_burn_stack();
    }

    (damage, success)
}

// TODO: Implement blue moon spear attack

pub type AttackFn =
    fn(&mut Player, &mut Monster, &mut ThreadRng, &Option<Box<dyn Limiter>>) -> (u32, bool);

pub fn get_attack_functions(player: &Player) -> AttackFn {
    // Dispatch attack function based on player's weapon

    if player.is_using_smoke_spell() {
        return smoke_spell_attack as AttackFn;
    } else if player.is_using_shadow_spell() {
        return shadow_spell_attack as AttackFn;
    } else if player.is_using_blood_spell() {
        return blood_spell_attack as AttackFn;
    } else if player.is_using_ice_spell() {
        return ice_spell_attack as AttackFn;
    }

    if player.is_using_crossbow() && !player.gear.weapon.name.contains("Karil") {
        match player.gear.ammo.as_ref().unwrap().name.as_str() {
            "Opal bolts (e)" | "Opal dragon bolts (e)" => return opal_bolt_attack as AttackFn,
            "Pearl bolts (e)" | "Pearl dragon bolts (e)" => return pearl_bolt_attack as AttackFn,
            "Emerald bolts (e)" | "Emerald dragon bolts (e)" => {
                return emerald_bolt_attack as AttackFn
            }
            "Ruby bolts (e)" | "Ruby dragon bolts (e)" => return ruby_bolt_attack as AttackFn,
            "Diamond bolts (e)" | "Diamond dragon bolts (e)" => {
                return diamond_bolt_attack as AttackFn
            }
            "Onyx bolts (e)" | "Onyx dragon bolts (e)" => return onyx_bolt_attack as AttackFn,
            "Dragonstone bolts (e)" | "Dragonstone dragon bolts (e)" => {
                return dragonstone_bolt_attack as AttackFn
            }
            _ => return standard_attack as AttackFn,
        }
    }

    match player.gear.weapon.name.as_str() {
        "Osmumten's fang" => fang_attack as AttackFn,
        "Ahrim's staff" => ahrims_staff_attack as AttackFn,
        "Dharok's greataxe" => dharoks_axe_attack as AttackFn,
        "Verac's flail" => veracs_flail_attack as AttackFn,
        "Karil's crossbow" => karils_crossbow_attack as AttackFn,
        "Guthan's warspear" => guthans_warspear_attack as AttackFn,
        "Torag's hammers" => torags_hammers_attack as AttackFn,
        "Sanguinesti staff" => sang_staff_attack as AttackFn,
        "Dawnbringer" => dawnbringer_attack as AttackFn,
        "Keris"
        | "Keris partisan"
        | "Keris partisan of corruption"
        | "Keris partisan of breaching" => keris_attack as AttackFn,
        "Keris partisan of the sun" => yellow_keris_attack as AttackFn,
        "Scythe of vitur" => scythe_attack as AttackFn,
        "Soulreaper axe" => soulreaper_axe_attack as AttackFn,
        "Gadderhammer" => gadderhammer_attack as AttackFn,
        "Tonalztics of ralos" => tonalztics_of_ralos_attack as AttackFn,
        "Dual macuahuitl" => dual_macuahuitl_attack as AttackFn,
        "Eclipse atlatl" => atlatl_attack as AttackFn,
        _ => standard_attack as AttackFn,
    }
}

fn apply_flat_armour_and_limiters(
    damage: u32,
    monster: &Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> u32 {
    let mut damage = damage;

    // Subtract flat armour from damage, post-roll (clamping at 1 damage)
    if monster.bonuses.flat_armour > 0 {
        damage = max(1, damage.saturating_sub(monster.bonuses.flat_armour));
    }

    // Apply a post-roll transform if there is one
    if let Some(limiter) = limiter {
        damage = limiter.apply(damage, rng);
    }

    damage
}
