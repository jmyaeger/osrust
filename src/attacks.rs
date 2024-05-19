use crate::constants::*;
use crate::equipment::CombatType;
use crate::monster::Monster;
use crate::player::Player;
use crate::rolls::calc_player_melee_rolls;
use crate::spells::{AncientSpell, Spell};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::cmp::{max, min};

pub fn standard_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
) -> (u32, bool) {
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

fn base_attack(
    max_att_roll: i32,
    max_def_roll: i32,
    min_hit: u32,
    max_hit: u32,
    rng: &mut ThreadRng,
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

fn accuracy_roll(max_att_roll: i32, rng: &mut ThreadRng) -> i32 {
    rng.gen_range(0..=max_att_roll)
}

fn defence_roll(max_def_roll: i32, rng: &mut ThreadRng) -> i32 {
    rng.gen_range(0..=max_def_roll)
}

fn damage_roll(min_hit: u32, max_hit: u32, rng: &mut ThreadRng) -> u32 {
    rng.gen_range(min_hit..=max_hit)
}

pub fn fang_attack(player: &mut Player, monster: &mut Monster, rng: &mut ThreadRng) -> (u32, bool) {
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
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
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
        monster.drain_strength(5);
    }

    if player.is_wearing("Amulet of the damned") && rng.gen_range(0..4) == 0 {
        damage = damage * 13 / 10;
    }

    (damage, success)
}

pub fn dharoks_axe_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
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

pub fn veracs_flail_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
) -> (u32, bool) {
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
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
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
    monster: &mut Monster,
    rng: &mut ThreadRng,
) -> (u32, bool) {
    let (damage, success) = standard_attack(player, monster, rng);
    if player.set_effects.full_guthans && rng.gen_range(0..4) == 0 {
        if player.is_wearing("Amulet of the damned") {
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
    monster: &mut Monster,
    rng: &mut ThreadRng,
) -> (u32, bool) {
    let (damage, success) = standard_attack(player, monster, rng);
    if rng.gen_range(0..6) == 0 {
        player.heal(damage, None)
    }

    (damage, success)
}

pub fn keris_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
) -> (u32, bool) {
    let (mut damage, success) = standard_attack(player, monster, rng);
    if monster.is_kalphite() && rng.gen_range(0..51) == 0 {
        damage *= 3;
    }

    (damage, success)
}
pub fn yellow_keris_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
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
        player.heal(12, Some(player.stats.hitpoints / 5));
        player.live_stats.prayer = player.live_stats.prayer.saturating_sub(5);
    }

    (damage, success)
}

pub fn opal_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
) -> (u32, bool) {
    let mut proc_chance = OPAL_PROC_CHANCE;
    if player.boosts.kandarin_diary {
        proc_chance *= 1.1;
    }

    let extra_damage = if player.is_wearing("Zaryte crossbow") {
        player.live_stats.ranged / 9
    } else {
        player.live_stats.ranged / 10
    };

    let max_hit = player.max_hits[&CombatType::Ranged];

    if rng.gen::<f64>() <= proc_chance {
        (damage_roll(0, max_hit, rng) + extra_damage, true)
    } else {
        standard_attack(player, monster, rng)
    }
}

pub fn pearl_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
) -> (u32, bool) {
    let mut proc_chance = PEARL_PROC_CHANCE;
    if player.boosts.kandarin_diary {
        proc_chance *= 1.1;
    }

    let mut denominator = if monster.is_fiery() { 15 } else { 20 };

    if player.is_wearing("Zaryte crossbow") {
        denominator = denominator * 9 / 10;
    }
    let extra_damage = player.live_stats.ranged / denominator;

    let max_hit = player.max_hits[&CombatType::Ranged];

    if rng.gen::<f64>() <= proc_chance {
        (damage_roll(0, max_hit, rng) + extra_damage, true)
    } else {
        standard_attack(player, monster, rng)
    }
}

pub fn emerald_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
) -> (u32, bool) {
    let mut proc_chance = EMERALD_PROC_CHANCE;
    if player.boosts.kandarin_diary {
        proc_chance *= 1.1;
    }

    let poison_severity = if player.is_wearing("Zaryte crossbow") {
        27
    } else {
        25
    };

    let (damage, success) = standard_attack(player, monster, rng);

    if success && rng.gen::<f64>() <= proc_chance {
        monster.info.poison_severity = poison_severity;
    }

    (damage, success)
}

pub fn ruby_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
) -> (u32, bool) {
    let mut proc_chance = RUBY_PROC_CHANCE;
    if player.boosts.kandarin_diary {
        proc_chance *= 1.1;
    }

    let ruby_damage = if player.is_wearing("Zaryte crossbow") {
        min(110, monster.live_stats.hitpoints * 22 / 100)
    } else {
        min(100, monster.live_stats.hitpoints / 5)
    };

    if rng.gen::<f64>() <= proc_chance {
        player.take_damage(player.live_stats.hitpoints / 10);
        (ruby_damage, true)
    } else {
        standard_attack(player, monster, rng)
    }
}

pub fn diamond_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
) -> (u32, bool) {
    let mut proc_chance = DIAMOND_PROC_CHANCE;
    if player.boosts.kandarin_diary {
        proc_chance *= 1.1;
    }

    let base_max_hit = player.max_hits[&CombatType::Ranged];
    let max_hit = if player.is_wearing("Zaryte crossbow") {
        base_max_hit * 126 / 100
    } else {
        base_max_hit * 115 / 100
    };

    if rng.gen::<f64>() <= proc_chance {
        (damage_roll(0, max_hit, rng), true)
    } else {
        standard_attack(player, monster, rng)
    }
}

pub fn onyx_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
) -> (u32, bool) {
    let mut proc_chance = ONYX_PROC_CHANCE;
    if player.boosts.kandarin_diary {
        proc_chance *= 1.1;
    }

    let base_max_hit = player.max_hits[&CombatType::Ranged];
    let max_hit = if player.is_wearing("Zaryte crossbow") {
        base_max_hit * 132 / 100
    } else {
        base_max_hit * 6 / 5
    };

    let (mut damage, success) = standard_attack(player, monster, rng);

    if success && !monster.is_undead() && rng.gen::<f64>() <= proc_chance {
        damage = damage_roll(0, max_hit, rng);
        player.heal(damage / 4, None);
    }

    (damage, success)
}

pub fn dragonstone_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
) -> (u32, bool) {
    let mut proc_chance = DRAGONSTONE_PROC_CHANCE;
    if player.boosts.kandarin_diary {
        proc_chance *= 1.1;
    }

    let extra_damage = if player.is_wearing("Zaryte crossbow") {
        player.live_stats.ranged * 2 / 9
    } else {
        player.live_stats.ranged / 5
    };

    let (mut damage, success) = standard_attack(player, monster, rng);

    if rng.gen::<f64>() <= proc_chance && !(monster.is_dragon() && monster.is_fiery()) {
        damage += extra_damage;
    }

    (damage, success)
}

pub fn smoke_spell_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
) -> (u32, bool) {
    monster.info.poison_severity = match (player.is_wearing_ancient_spectre(), player.attrs.spell) {
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

    standard_attack(player, monster, rng)
}

pub fn shadow_spell_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
) -> (u32, bool) {
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

    let (damage, success) = standard_attack(player, monster, rng);

    if success {
        if monster.live_stats.attack == monster.stats.attack {
            monster.drain_attack(monster.stats.attack * drain_amount / 1000);
        }
        if player.is_wearing("Shadow ancient sceptre") {
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

fn blood_spell_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
) -> (u32, bool) {
    let heal_factor = if player.is_wearing_ancient_spectre() {
        275 + 20 * player.set_effects.bloodbark_pieces as u32
    } else {
        250 + 20 * player.set_effects.bloodbark_pieces as u32
    };

    let overheal = if player.is_wearing("Blood ancient sceptre") {
        Some(player.stats.hitpoints / 10)
    } else {
        None
    };

    let (damage, success) = standard_attack(player, monster, rng);
    player.heal(damage * heal_factor / 1000, overheal);

    (damage, success)
}

pub fn ice_spell_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
) -> (u32, bool) {
    if monster.is_freezable() {
        let mut max_att_roll = player.att_rolls[&CombatType::Magic];
        let max_def_roll = monster.def_rolls[&CombatType::Magic];
        let max_hit = player.max_hits[&CombatType::Magic];

        if player.is_wearing("Ice ancient sceptre") {
            max_att_roll = max_att_roll * 11 / 10;
        }

        let (damage, success) = base_attack(max_att_roll, max_def_roll, 0, max_hit, rng);

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
        }

        (damage, success)
    } else {
        standard_attack(player, monster, rng)
    }
}

pub fn scythe_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
) -> (u32, bool) {
    let combat_type = player.combat_type();
    let max_att_roll = player.att_rolls[&combat_type];
    let max_def_roll = monster.def_rolls[&combat_type];
    let max_hit = player.max_hits[&combat_type];

    let (damage1, success1) = standard_attack(player, monster, rng);
    if monster.info.size == 1 {
        return (damage1, success1);
    }

    let (damage2, success2) = base_attack(max_att_roll, max_def_roll, 0, max_hit / 2, rng);
    if monster.info.size == 2 {
        return (damage2, success2);
    }

    let (damage3, success3) = base_attack(max_att_roll, max_def_roll, 0, max_hit / 4, rng);

    (
        damage1 + damage2 + damage3,
        success1 || success2 || success3,
    )
}

pub fn soulreaper_axe_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
) -> (u32, bool) {
    let (damage, success) = standard_attack(player, monster, rng);

    if player.boosts.soulreaper_stacks < 5 && player.live_stats.hitpoints > 8 {
        player.take_damage(SOULREAPER_STACK_DAMAGE);
        player.boosts.soulreaper_stacks += 1;
        calc_player_melee_rolls(player, monster);
    }

    (damage, success)
}

pub fn gadderhammer_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
) -> (u32, bool) {
    let (mut damage, success) = standard_attack(player, monster, rng);

    if success && monster.is_shade() {
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
) -> (u32, bool) {
    let max_att_roll = player.att_rolls[&CombatType::Ranged];
    let max_def_roll = monster.def_rolls[&CombatType::Ranged];
    let max_hit = player.max_hits[&CombatType::Ranged] * 3 / 4;

    let (damage1, success1) = base_attack(max_att_roll, max_def_roll, 0, max_hit, rng);
    if player.gear.weapon.name.contains("charged") {
        let (damage2, success2) = base_attack(max_att_roll, max_def_roll, 0, max_hit, rng);
        return (damage1 + damage2, success1 || success2);
    }

    (damage1, success1)
}

pub fn dual_macuahuitl_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
) -> (u32, bool) {
    let combat_type = player.combat_type();
    let max_att_roll = player.att_rolls[&combat_type];
    let max_def_roll = monster.def_rolls[&combat_type];
    let max_hit = player.max_hits[&combat_type];

    // Reset attack speed to 4 ticks
    player.gear.weapon.speed = 4;

    let max_hit1 = max_hit / 2;
    let max_hit2 = max_hit - max_hit1;
    let (damage1, success1) = base_attack(max_att_roll, max_def_roll, 0, max_hit1, rng);
    let (damage2, success2) = if success1 {
        base_attack(max_att_roll, max_def_roll, 0, max_hit2, rng)
    } else {
        (0, false)
    };

    // Roll for next attack to be one tick faster
    if player.set_effects.full_blood_moon && (success1 && rng.gen_range(0..3) == 0)
        || (success2 && rng.gen_range(0..3) == 0)
    {
        player.gear.weapon.speed = 3;
    }

    (damage1 + damage2, success1)
}

// TODO: Implement eclipse atlatl set effect

// TODO: Implement blue moon spear

pub type AttackFn = fn(&mut Player, &mut Monster, &mut ThreadRng) -> (u32, bool);

pub fn get_attack_functions(
    player: &Player,
) -> impl Fn(&mut Player, &mut Monster, &mut ThreadRng) -> (u32, bool) {
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
        "Keris"
        | "Keris partisan"
        | "Keris partisan of corruption"
        | "Keris partisan of breaching" => keris_attack as AttackFn,
        "Keris partisan of the sun" => yellow_keris_attack as AttackFn,
        "Scythe of vitur" => scythe_attack as AttackFn,
        "Soulreaper axe" => soulreaper_axe_attack as AttackFn,
        "Gadderhammer" => gadderhammer_attack as AttackFn,
        "Tonalztics of Ralos (charged)" | "Tonalztics of Ralos (uncharged)" => {
            tonalztics_of_ralos_attack as AttackFn
        }
        "Dual macuahuitl" => dual_macuahuitl_attack as AttackFn,
        _ => standard_attack as AttackFn,
    }
}
