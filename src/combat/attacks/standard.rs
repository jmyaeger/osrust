use crate::calc::rolls::{self, calc_player_melee_rolls};
use crate::combat::attacks::effects::CombatEffect;
use crate::combat::limiters::Limiter;
use crate::constants::*;
use crate::types::equipment::{CombatStyle, CombatType};
use crate::types::monster::{CombatStat, Monster};
use crate::types::player::Player;
use crate::types::spells::{AncientSpell, Spell};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::cmp::max;
use std::ops::Mul;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AttackInfo {
    pub combat_type: CombatType,
    pub max_att_roll: i32,
    pub max_def_roll: i32,
    pub min_hit: u32,
    pub max_hit: u32,
}

impl AttackInfo {
    pub fn new(player: &Player, monster: &Monster) -> AttackInfo {
        let combat_type = player.combat_type();
        let min_hit = if combat_type == CombatType::Magic
            && player.boosts.sunfire.active
            && player.is_using_fire_spell()
        {
            player.boosts.sunfire.min_hit
        } else {
            0
        };
        AttackInfo {
            combat_type,
            max_att_roll: player.att_rolls.get(combat_type),
            max_def_roll: monster.def_rolls.get(combat_type),
            min_hit,
            max_hit: player.max_hits.get(combat_type),
        }
    }

    fn apply_brimstone_ring(&mut self, player: &Player, rng: &mut ThreadRng) {
        if self.combat_type == CombatType::Magic
            && player.is_wearing("Brimstone ring", None)
            && rng.gen_range(0..4) == 0
        {
            self.max_def_roll = self.max_def_roll * 9 / 10;
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct Hit {
    pub damage: u32,
    pub success: bool,
}

impl Hit {
    pub fn new(damage: u32, success: bool) -> Hit {
        Hit { damage, success }
    }

    pub fn apply_transforms(
        &mut self,
        player: &mut Player,
        monster: &Monster,
        rng: &mut ThreadRng,
        limiter: &Option<Box<dyn Limiter>>,
    ) {
        self.apply_berserker_necklace(player);
        self.damage = max(self.damage, 1);
        self.apply_flat_armour(monster);
        self.apply_limiters(rng, limiter);
    }

    pub fn combine(&self, other: &Hit) -> Hit {
        Hit::new(self.damage + other.damage, self.success || other.success)
    }

    pub fn accurate(damage: u32) -> Hit {
        Hit::new(damage, true)
    }

    pub fn inaccurate() -> Hit {
        Hit::new(0, false)
    }

    fn apply_flat_armour(&mut self, monster: &Monster) {
        // Subtract flat armour from damage, post-roll (clamping at 1 damage)
        if monster.bonuses.flat_armour > 0 {
            self.damage = max(
                1,
                self.damage
                    .saturating_sub(monster.bonuses.flat_armour.try_into().unwrap_or(0)),
            );
        }
    }

    pub fn apply_limiters(&mut self, rng: &mut ThreadRng, limiter: &Option<Box<dyn Limiter>>) {
        // Apply a post-roll transform if there is one
        if let Some(limiter) = limiter {
            self.damage = limiter.apply(self.damage, rng);
        }
    }

    pub fn apply_berserker_necklace(&mut self, player: &Player) {
        if player.is_using_melee()
            && player.is_wearing("Berserker necklace", None)
            && player.is_wearing_tzhaar_weapon()
        {
            self.damage = self.damage * 6 / 5;
        }
    }
}

pub fn standard_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    // Default attack method for most weapons

    // Determine max attack, defense, and damage rolls
    let mut info = AttackInfo::new(player, monster);

    // Roll for brimstone ring effect if applicable
    info.apply_brimstone_ring(player, rng);

    let mut hit = base_attack(&info, rng);

    if hit.success {
        // Transform any accurate zeros into 1s, then apply post-roll transforms
        hit.apply_transforms(player, monster, rng, limiter);
    }

    hit
}

pub fn base_attack(att_info: &AttackInfo, rng: &mut ThreadRng) -> Hit {
    let att_roll = accuracy_roll(att_info.max_att_roll, rng);
    let def_roll = defence_roll(att_info.max_def_roll, rng);

    // Roll for accuracy
    let success = att_roll > def_roll;

    // Roll for damage if successful
    let mut damage = 0;
    if success {
        damage = damage_roll(att_info.min_hit, att_info.max_hit, rng);
    }

    Hit::new(damage, success)
}

fn accuracy_roll(max_att_roll: i32, rng: &mut ThreadRng) -> i32 {
    rng.gen_range(0..=max_att_roll)
}

fn defence_roll(max_def_roll: i32, rng: &mut ThreadRng) -> i32 {
    rng.gen_range(0..=max_def_roll)
}

pub fn damage_roll(min_hit: u32, max_hit: u32, rng: &mut ThreadRng) -> u32 {
    rng.gen_range(min_hit..=max_hit)
}

pub fn fang_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let combat_type = player.combat_type();
    let max_att_roll = player.att_rolls.get(combat_type);
    let max_def_roll = monster.def_rolls.get(combat_type);
    let true_max_hit = player.max_hits.get(combat_type);

    // Fang rolls from 15% of max hit to max_hit - 15%
    let min_hit = true_max_hit * 15 / 100;
    let max_hit = true_max_hit - min_hit;

    let att_roll1 = accuracy_roll(max_att_roll, rng);
    let def_roll1 = defence_roll(max_def_roll, rng);

    let (damage, success) = if att_roll1 > def_roll1 {
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

    let mut hit = Hit::new(damage, success);

    if hit.success {
        // No accurate zeros, so no need to do anything before applying post-roll transforms
        hit.apply_transforms(player, monster, rng, limiter);
    }

    hit
}
pub fn ahrims_staff_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let combat_type = player.combat_type();

    // Do a standard attack if not using magic or the full ahrim's set
    if combat_type != CombatType::Magic || !player.set_effects.full_ahrims {
        return standard_attack(player, monster, rng, limiter);
    }

    let mut info = AttackInfo::new(player, monster);

    info.apply_brimstone_ring(player, rng);

    let mut hit = base_attack(&info, rng);

    if hit.success && rng.gen_range(0..4) == 0 {
        // Base set effect rolls a 25% chance to reduce strength by 5
        monster.drain_stat(CombatStat::Strength, 5, None);
    }

    if player.is_wearing_any_version("Amulet of the damned") && rng.gen_range(0..4) == 0 {
        // With amulet of the damned, 25% chance to increase damage 30% post-roll
        hit.damage = hit.damage * 13 / 10;
    }

    if hit.success {
        hit.apply_transforms(player, monster, rng, limiter);
    }

    hit
}

pub fn dharoks_axe_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let info = AttackInfo::new(player, monster);

    let mut hit = base_attack(&info, rng);

    if hit.success && player.set_effects.full_dharoks {
        // Set effect damage increase is applied post-roll
        let max_hp = player.stats.hitpoints.base;
        let current_hp = player.stats.hitpoints.current;
        let dmg_mod = 10000 + (max_hp - current_hp) * max_hp;
        hit.damage = hit.damage * dmg_mod / 10000;
    }

    if hit.success {
        hit.apply_transforms(player, monster, rng, limiter);
    }

    hit
}

pub fn veracs_flail_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let combat_type = player.combat_type();
    if player.set_effects.full_veracs && rng.gen_range(0..4) == 0 {
        // Set effect rolls 25% chance to guarantee hit (minimum 1 damage)
        let mut hit = Hit::accurate(1 + damage_roll(1, player.max_hits.get(combat_type) + 1, rng));
        hit.apply_transforms(player, monster, rng, limiter);
        hit
    } else {
        standard_attack(player, monster, rng, limiter)
    }
}

pub fn karils_crossbow_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    if player.set_effects.full_karils
        && player.is_wearing_any_version("Amulet of the damned")
        && rng.gen_range(0..4) == 0
    {
        // Set effect rolls 25% chance to hit an additional time for half the first hit's damage
        let hit1 = standard_attack(player, monster, rng, limiter);
        let hit2 = Hit::new(hit1.damage / 2, true);
        hit1.combine(&hit2)
    } else {
        standard_attack(player, monster, rng, limiter)
    }
}

pub fn guthans_warspear_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let hit = standard_attack(player, monster, rng, limiter);
    if player.set_effects.full_guthans && rng.gen_range(0..4) == 0 {
        // Set effect rolls 25% chance to heal by the damage dealt
        if player.is_wearing_any_version("Amulet of the damned") {
            // Amulet of the damned allows up to 10 HP of overheal
            player.heal(hit.damage, Some(10));
        } else {
            player.heal(hit.damage, None);
        }
    }

    hit
}

pub fn torags_hammers_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info1 = AttackInfo::new(player, monster);
    let mut info2 = info1.clone();
    let max_hit = info1.max_hit;
    info1.max_hit = max_hit / 2;
    info2.max_hit = max_hit - max_hit / 2;

    // Hammers attack with two independently rolled hits (tested in-game)
    let mut hit1 = base_attack(&info1, rng);
    let mut hit2 = base_attack(&info2, rng);

    // Not implementing the normal set effect because it only applies in PvP
    // Amulet of the damned effect gets implemented in roll calcs

    if hit1.success {
        hit1.apply_transforms(player, monster, rng, limiter);
    }

    if hit2.success {
        hit2.apply_transforms(player, monster, rng, limiter);
    }

    hit1.combine(&hit2)
}

pub fn sang_staff_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let hit = standard_attack(player, monster, rng, limiter);
    if rng.gen_range(0..6) == 0 {
        // 1/6 chance to heal by half of the damage dealt
        player.heal(hit.damage / 2, None)
    }

    hit
}

pub fn dawnbringer_attack(
    player: &mut Player,
    _: &mut Monster,
    rng: &mut ThreadRng,
    _: &Option<Box<dyn Limiter>>,
) -> Hit {
    let max_hit = player.max_hits.get(player.combat_type());

    // Guaranteed to hit because it can only be used on Verzik
    let mut damage = damage_roll(0, max_hit, rng);
    damage = max(1, damage);
    Hit::accurate(damage)
}

pub fn keris_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut hit = standard_attack(player, monster, rng, limiter);

    // 1/51 chance to deal triple damage (post-roll)
    if monster.is_kalphite() && rng.gen_range(0..51) == 0 {
        hit.damage *= 3;
    }

    hit
}
pub fn yellow_keris_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    if (monster.stats.hitpoints.current as f32) / (monster.stats.hitpoints.base as f32) < 0.25
        && monster.is_toa_monster()
    {
        // In ToA, accuracy is boosted by 25% when monster is below 25% health
        info.max_att_roll = info.max_att_roll * 5 / 4;
    }

    let mut hit = base_attack(&info, rng);

    if monster.stats.hitpoints.current.saturating_sub(hit.damage) == 0 && monster.is_toa_monster() {
        // Killing a ToA monster heals the player by 12 and costs 5 prayer points
        player.heal(12, Some(player.stats.hitpoints.base / 5));
        player.stats.prayer.drain(5, None);
    }

    if hit.success {
        hit.apply_transforms(player, monster, rng, limiter);
    }

    hit
}

pub fn opal_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let proc_chance = player.bolt_proc_chance(OPAL_PROC_CHANCE);

    let extra_damage = if player.is_wearing("Zaryte crossbow", None) {
        player.stats.ranged.current / 9
    } else {
        player.stats.ranged.current / 10
    };

    // Guaranteed hit if the bolt effect procs (verified in-game)
    if rng.gen::<f64>() <= proc_chance {
        // Bolt effect adds on flat damage based on visible ranged level
        let att_info = AttackInfo::new(player, monster);
        let mut hit = base_attack(&att_info, rng);
        hit.damage += extra_damage;
        hit.apply_transforms(player, monster, rng, limiter);
        hit
    } else {
        standard_attack(player, monster, rng, limiter)
    }
}

pub fn pearl_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let proc_chance = player.bolt_proc_chance(PEARL_PROC_CHANCE);

    // Bolt effect is extra effective against fiery monsters
    let mut denominator = if monster.is_fiery() { 15 } else { 20 };

    if player.is_wearing("Zaryte crossbow", None) {
        denominator = denominator * 9 / 10;
    }
    let extra_damage = player.stats.ranged.current / denominator;

    // Same implementation as opal bolts (accurate hit on procs, flat damage added)
    if rng.gen::<f64>() <= proc_chance {
        let att_info = AttackInfo::new(player, monster);
        let mut hit = base_attack(&att_info, rng);
        hit.damage += extra_damage;
        hit.apply_transforms(player, monster, rng, limiter);
        hit
    } else {
        standard_attack(player, monster, rng, limiter)
    }
}

pub fn emerald_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let proc_chance = player.bolt_proc_chance(EMERALD_PROC_CHANCE);

    let poison_severity = if player.is_wearing("Zaryte crossbow", None) {
        27
    } else {
        25
    };

    let hit = standard_attack(player, monster, rng, limiter);

    if hit.success && rng.gen::<f64>() <= proc_chance {
        // TODO: Change this to use a CombatEffect
        monster.info.poison_severity = poison_severity;
    }

    hit
}

pub fn ruby_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let proc_chance = player.bolt_proc_chance(RUBY_PROC_CHANCE);

    let ruby_damage = if player.is_wearing("Zaryte crossbow", None) {
        // Verified to be 22/100, not 2/9
        (monster.stats.hitpoints.current * 22 / 100).clamp(1, 110)
    } else {
        (monster.stats.hitpoints.current / 5).clamp(1, 100)
    };

    if rng.gen::<f64>() <= proc_chance {
        // Bolt proc ignores defense and deals fixed amount of damage
        player.take_damage(player.stats.hitpoints.current / 10);
        let damage = if limiter.is_some() && !monster.info.name.contains("Corporeal Beast") {
            limiter.as_ref().unwrap().apply(ruby_damage, rng)
        } else {
            ruby_damage
        };

        let mut hit = Hit::accurate(damage);
        hit.damage = max(1, hit.damage);
        hit.apply_flat_armour(monster);
        hit
    } else {
        standard_attack(player, monster, rng, limiter)
    }
}

pub fn diamond_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let proc_chance = player.bolt_proc_chance(DIAMOND_PROC_CHANCE);

    let base_max_hit = player.max_hits.get(player.combat_type());
    let max_hit = if player.is_wearing("Zaryte crossbow", None) {
        base_max_hit * 126 / 100
    } else {
        base_max_hit * 115 / 100
    };

    if rng.gen::<f64>() <= proc_chance {
        // Bolt proc ignores defense and boosts max hit
        let mut hit = Hit::accurate(damage_roll(0, max_hit, rng));
        hit.apply_transforms(player, monster, rng, limiter);
        hit
    } else {
        standard_attack(player, monster, rng, limiter)
    }
}

pub fn onyx_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let proc_chance = player.bolt_proc_chance(ONYX_PROC_CHANCE);

    let base_max_hit = player.max_hits.get(player.combat_type());
    let max_hit = if player.is_wearing("Zaryte crossbow", None) {
        base_max_hit * 132 / 100
    } else {
        base_max_hit * 6 / 5
    };

    let mut hit = standard_attack(player, monster, rng, limiter);

    if hit.success && !monster.is_undead() && rng.gen::<f64>() <= proc_chance {
        // Bolt proc boosts max hit but does not ignore defense
        hit.damage = damage_roll(0, max_hit, rng);
        hit.apply_transforms(player, monster, rng, limiter);

        // Heal the player by 1/4 of the damage
        player.heal(hit.damage / 4, None);
    }

    hit
}

pub fn dragonstone_bolt_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let proc_chance = player.bolt_proc_chance(DRAGONSTONE_PROC_CHANCE);

    let extra_damage = if player.is_wearing("Zaryte crossbow", None) {
        player.stats.ranged.current * 2 / 9
    } else {
        player.stats.ranged.current / 5
    };

    let info = AttackInfo::new(player, monster);

    let mut hit = base_attack(&info, rng);

    if hit.success {
        // Only dragons that are also "fiery" are immune
        // Bolt proc requires accurate hit and adds flat damage
        if rng.gen::<f64>() <= proc_chance && !(monster.is_dragon() && monster.is_fiery()) {
            hit.damage += extra_damage;
        }

        hit.apply_transforms(player, monster, rng, limiter);
    }

    hit
}

pub fn smoke_spell_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
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
) -> Hit {
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

    let hit = standard_attack(player, monster, rng, limiter);

    if hit.success {
        // Only drains attack if it hasn't been drained already
        if monster.stats.attack.current == monster.stats.attack.base {
            monster.drain_stat(
                CombatStat::Attack,
                monster.stats.attack.base * drain_amount / 1000,
                None,
            );
        }
        if player.is_wearing("Shadow ancient sceptre", None) {
            // Shadow ancient sceptre also drains strength and defense if not drained previously
            if monster.stats.strength.current == monster.stats.strength.base {
                monster.drain_stat(
                    CombatStat::Strength,
                    monster.stats.strength.base * drain_amount / 1000,
                    None,
                );
            }
            if monster.stats.defence.current == monster.stats.defence.base {
                monster.drain_stat(
                    CombatStat::Defence,
                    monster.stats.defence.base * drain_amount / 1000,
                    None,
                );
            }
        }
    }

    hit
}

pub fn blood_spell_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    // Heal factor is (percentage of damage) * 1000
    let heal_factor = if player.is_wearing_ancient_spectre() {
        // Bloodbark pieces add 2% healing per piece
        275 + 20 * player.set_effects.bloodbark_pieces as u32
    } else {
        250 + 20 * player.set_effects.bloodbark_pieces as u32
    };

    let overheal = if player.is_wearing("Blood ancient sceptre", None) {
        // Blood ancient sceptre allows 10% overheal
        Some(player.stats.hitpoints.base / 10)
    } else {
        None
    };

    let hit = standard_attack(player, monster, rng, limiter);
    player.heal(hit.damage * heal_factor / 1000, overheal);

    hit
}

pub fn ice_spell_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    if monster.is_freezable() {
        // Monster is freezable if not immune and not currently frozen or on cooldown
        let mut info = AttackInfo::new(player, monster);

        if player.is_wearing("Ice ancient sceptre", None) {
            // Ice ancient sceptre is 10% more accurate on unfrozen, freezable targets
            info.max_att_roll = info.max_att_roll * 11 / 10;
        }

        let mut hit = base_attack(&info, rng);

        if hit.success {
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
            hit.apply_transforms(player, monster, rng, limiter);
        }

        hit
    } else {
        standard_attack(player, monster, rng, limiter)
    }
}

pub fn scythe_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let hit1 = standard_attack(player, monster, rng, limiter);
    if monster.info.size == 1 {
        // standard_attack already applies post-roll transforms, so it's not needed here
        return hit1;
    }

    let mut info2 = AttackInfo::new(player, monster);
    info2.max_hit /= 2;
    let mut hit2 = base_attack(&info2, rng);
    if hit2.success {
        hit2.apply_transforms(player, monster, rng, limiter);
    }
    if monster.info.size == 2 {
        return hit1.combine(&hit2);
    }

    let mut info3 = AttackInfo::new(player, monster);
    info3.max_hit /= 4;
    let mut hit3 = base_attack(&info3, rng);
    if hit3.success {
        hit3.apply_transforms(player, monster, rng, limiter);
    }
    hit1.combine(&hit2).combine(&hit3)
}

pub fn soulreaper_axe_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let hit = standard_attack(player, monster, rng, limiter);

    if player.boosts.soulreaper_stacks < 5 && player.stats.hitpoints.current > 8 {
        // Add a soulreaper stack if the player has less than 5 stacks and can survive the self-damage
        player.take_damage(SOULREAPER_STACK_DAMAGE);
        player.boosts.soulreaper_stacks += 1;

        // Recalculate melee rolls with stack boost added
        calc_player_melee_rolls(player, monster);
    }

    hit
}

pub fn gadderhammer_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut hit = standard_attack(player, monster, rng, limiter);

    if hit.success && monster.is_shade() {
        // 25% damage boost with 5% chance to double unboosted damage on shades (all post-roll)
        if rng.gen_range(0..20) == 0 {
            hit.damage *= 2;
        } else {
            hit.damage = hit.damage * 5 / 4;
        }
    }

    hit
}

pub fn tonalztics_of_ralos_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Rolls up to 3/4 of the "true" max hit for each hit
    info.max_hit = info.max_hit * 3 / 4;

    let mut hit1 = base_attack(&info, rng);
    if hit1.success {
        hit1.apply_transforms(player, monster, rng, limiter);
    }
    if player.gear.weapon.matches_version("Charged") {
        // Only the charged version does a second attack
        let mut hit2 = base_attack(&info, rng);
        if hit2.success {
            hit2.apply_transforms(player, monster, rng, limiter);
        }
        return hit1.combine(&hit2);
    }

    hit1
}

pub fn dual_macuahuitl_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info1 = AttackInfo::new(player, monster);
    let mut info2 = info1.clone();
    let max_hit = info1.max_hit;
    info1.max_hit = max_hit / 2;
    info2.max_hit = max_hit - max_hit / 2;

    // Reset attack speed to 4 ticks
    player.gear.weapon.speed = 4;

    // Roll two separate hits
    let mut hit1 = base_attack(&info1, rng);
    if hit1.success {
        hit1.apply_transforms(player, monster, rng, limiter);
    }
    let mut hit2 = if hit1.success {
        // Only roll the second hit if the first hit was accurate
        base_attack(&info2, rng)
    } else {
        Hit::inaccurate()
    };

    if hit2.success {
        hit2.apply_transforms(player, monster, rng, limiter);
    }

    // Roll 33% chance for next attack to be one tick faster if the full set is equipped
    if player.set_effects.full_blood_moon
        && ((hit1.success && rng.gen_range(0..3) == 0)
            || (hit2.success && rng.gen_range(0..3) == 0))
    {
        player.gear.weapon.speed = 3;
    }

    hit1.combine(&hit2)
}

pub fn atlatl_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let hit = standard_attack(player, monster, rng, limiter);
    if hit.success
        && !monster.is_immune_to_strong_burn()
        && player.set_effects.full_eclipse_moon
        && rng.gen_range(0..5) == 0
    {
        // Roll 20% chance to add a burn stack if full set is equipped
        monster.add_burn_stack(10);
    }

    hit
}

pub fn blue_moon_spear_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let hit = standard_attack(player, monster, rng, limiter);

    // Bind spells have a chance to perform a melee attack on the next tick
    if hit.success && player.set_effects.full_blue_moon && player.is_using_bind_spell() {
        // Store current combat style
        let current_style = player.attrs.active_style;

        // Grasp spells have 50% chance while other binds have 20% chance
        let range_max = if player.is_using_grasp_spell() { 2 } else { 5 };
        if rng.gen_range(0..range_max) == 0 {
            player.set_active_style(CombatStyle::Swipe); // Specific melee style unknown, assuming aggressive
            let melee_hit = standard_attack(player, monster, rng, limiter);
            if melee_hit.success {
                // No point pushing an empty effect if it misses
                monster.active_effects.push(CombatEffect::DelayedAttack {
                    tick_delay: Some(1),
                    damage: melee_hit.damage,
                });
            }

            // Reset combat style to original
            player.set_active_style(current_style);
        }
    }

    hit
}

pub fn demonbane_spell_attack(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut hit = standard_attack(player, monster, rng, limiter);
    if player.boosts.mark_of_darkness {
        let damage_boost = if player.is_wearing("Purging staff", None) {
            50
        } else {
            25
        };
        let added_damage = rolls::get_demonbane_factor(100, monster)
            .multiply_to_int(hit.damage * damage_boost / 100);
        hit.damage += added_damage;
    }
    hit
}

pub type AttackFn = fn(&mut Player, &mut Monster, &mut ThreadRng, &Option<Box<dyn Limiter>>) -> Hit;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calc::rolls::{calc_active_player_rolls, monster_def_rolls};
    use crate::combat::simulation::assign_limiter;
    use crate::types::equipment::CombatStyle;
    use crate::types::monster::Monster;
    use crate::types::potions::Potion;
    use crate::types::prayers::{Prayer, PrayerBoost};
    use crate::types::stats::Stat;
    use crate::utils::loadouts::{self, *};

    #[test]
    fn test_atlatl_dps() {
        let mut player = max_melee_player();
        player.equip("Eclipse atlatl", None);
        player.equip("Eclipse moon helm", None);
        player.equip("Eclipse moon chestplate", None);
        player.equip("Eclipse moon tassets", None);
        player.equip("Atlatl dart", None);
        player.update_bonuses();
        player.update_set_effects();
        player.set_active_style(CombatStyle::Rapid);
        player.prayers.add(PrayerBoost::new(Prayer::Rigour));
        player.add_potion(Potion::Ranging);

        let mut monster = Monster::new("Vorkath", Some("Post-quest")).unwrap();
        monster.stats.defence = Stat::new(1);
        monster.bonuses.defence.standard = -64;
        player.bonuses.attack.ranged = 10000;

        calc_active_player_rolls(&mut player, &monster);
        monster.def_rolls = monster_def_rolls(&monster);
        let limiter = assign_limiter(&player, &monster);

        let mut rng = rand::thread_rng();
        let mut hit_damage = 0;
        let mut burn_damage = 0;
        let n = 1000000000;
        let mut hit_counter = 0i64;
        let mut tick_counter = 0i64;
        let mut attack_tick = 0;
        let mut accurate_hits = 0i64;
        let mut num_stacks = Vec::new();

        while hit_counter < n {
            if tick_counter == attack_tick {
                let hit = atlatl_attack(&mut player, &mut monster, &mut rng, &limiter);
                if hit.success {
                    accurate_hits += 1;
                }
                hit_damage += hit.damage as u64;
                hit_counter += 1;
                attack_tick += player.gear.weapon.speed as i64;
            }

            for effect in &mut monster.active_effects {
                let burn = effect.apply();
                burn_damage += burn as u64;
            }
            monster.clear_inactive_effects();

            if let Some(CombatEffect::Burn {
                tick_counter: _,
                stacks,
            }) = monster
                .active_effects
                .iter()
                .find(|item| matches!(item, &CombatEffect::Burn { .. }))
            {
                num_stacks.push(stacks.len());
            } else {
                num_stacks.push(0);
            }

            tick_counter += 1;
        }

        // let mut remaining_burn_damage = 0;
        // if let Some(CombatEffect::Burn {
        //     tick_counter: _,
        //     stacks,
        // }) = monster
        //     .active_effects
        //     .iter()
        //     .find(|item| matches!(item, &CombatEffect::Burn { .. }))
        // {
        //     remaining_burn_damage += stacks.iter().sum::<u32>();
        // }
        // burn_damage += remaining_burn_damage as u64;

        let dps = hit_damage as f64 / (n as f64 * 1.8);
        let dps_with_burn = (hit_damage + burn_damage) as f64 / (n as f64 * 1.8);
        let acc = accurate_hits as f64 / n as f64;
        println!("DPS from burn: {:.5}, acc: {:.5}", dps_with_burn - dps, acc);

        let prob_zero_stacks =
            num_stacks.iter().filter(|s| **s == 0).count() as f64 / tick_counter as f64;
        let prob_one_stack =
            num_stacks.iter().filter(|s| **s == 1).count() as f64 / tick_counter as f64;
        let prob_two_stacks =
            num_stacks.iter().filter(|s| **s == 2).count() as f64 / tick_counter as f64;
        let prob_three_stacks =
            num_stacks.iter().filter(|s| **s == 3).count() as f64 / tick_counter as f64;
        let prob_four_stacks =
            num_stacks.iter().filter(|s| **s == 4).count() as f64 / tick_counter as f64;
        let prob_five_stacks =
            num_stacks.iter().filter(|s| **s == 5).count() as f64 / tick_counter as f64;
        let prob_six_stacks =
            num_stacks.iter().filter(|s| **s == 6).count() as f64 / tick_counter as f64;
        let prob_seven_stacks =
            num_stacks.iter().filter(|s| **s == 7).count() as f64 / tick_counter as f64;
        let prob_eight_stacks =
            num_stacks.iter().filter(|s| **s == 8).count() as f64 / tick_counter as f64;
        let prob_nine_stacks =
            num_stacks.iter().filter(|s| **s == 9).count() as f64 / tick_counter as f64;
        let prob_ten_stacks =
            num_stacks.iter().filter(|s| **s == 10).count() as f64 / tick_counter as f64;
        let prob_eleven_stacks =
            num_stacks.iter().filter(|s| **s == 11).count() as f64 / tick_counter as f64;
        let prob_twelve_stacks =
            num_stacks.iter().filter(|s| **s == 12).count() as f64 / tick_counter as f64;
        let prob_thirteen_stacks =
            num_stacks.iter().filter(|s| **s == 13).count() as f64 / tick_counter as f64;

        println!("Probability of zero stacks: {:.10}", prob_zero_stacks);
        println!("Probability of one stack: {:.10}", prob_one_stack);
        println!("Probability of two stacks: {:.10}", prob_two_stacks);
        println!("Probability of three stacks: {:.10}", prob_three_stacks);
        println!("Probability of four stacks: {:.10}", prob_four_stacks);
        println!("Probability of five stacks: {:.10}", prob_five_stacks);
        println!("Probability of six stacks: {:.10}", prob_six_stacks);
        println!("Probability of seven stacks: {:.10}", prob_seven_stacks);
        println!("Probability of eight stacks: {:.10}", prob_eight_stacks);
        println!("Probability of nine stacks: {:.10}", prob_nine_stacks);
        println!("Probability of ten stacks: {:.10}", prob_ten_stacks);
        println!("Probability of eleven stacks: {:.10}", prob_eleven_stacks);
        println!("Probability of twelve stacks: {:.10}", prob_twelve_stacks);
        println!(
            "Probability of thirteen stacks: {:.10}",
            prob_thirteen_stacks
        );

        let avg_stacks = num_stacks.iter().sum::<usize>() as f64 / num_stacks.len() as f64;
        println!("Average number of stacks: {:.6}", avg_stacks);
        assert!(dps_with_burn > dps);
    }

    #[test]
    fn test_soulreaper_stacks() {
        let mut monster = Monster::new("Ammonite Crab", None).unwrap();

        let mut player = loadouts::max_melee_player();
        player.equip("Soulreaper axe", None);
        player.set_active_style(CombatStyle::Hack);
        player.attack = get_attack_functions(&player);
        player.update_bonuses();
        calc_active_player_rolls(&mut player, &monster);

        let mut rng = rand::thread_rng();
        let mut limiter = assign_limiter(&player, &monster);

        assert_eq!(player.boosts.soulreaper_stacks, 0);
        assert_eq!(player.stats.hitpoints.current, 99);
        assert_eq!(player.max_hits.get(CombatType::Slash), 62);

        let _ = (player.attack)(&mut player, &mut monster, &mut rng, &mut limiter);
        assert_eq!(player.boosts.soulreaper_stacks, 1);
        assert_eq!(player.stats.hitpoints.current, 91);
        assert_eq!(player.max_hits.get(CombatType::Slash), 65);

        let _ = (player.attack)(&mut player, &mut monster, &mut rng, &mut limiter);
        assert_eq!(player.boosts.soulreaper_stacks, 2);
        assert_eq!(player.stats.hitpoints.current, 83);
        assert_eq!(player.max_hits.get(CombatType::Slash), 67);

        let _ = (player.attack)(&mut player, &mut monster, &mut rng, &mut limiter);
        assert_eq!(player.boosts.soulreaper_stacks, 3);
        assert_eq!(player.stats.hitpoints.current, 75);
        assert_eq!(player.max_hits.get(CombatType::Slash), 70);

        let _ = (player.attack)(&mut player, &mut monster, &mut rng, &mut limiter);
        assert_eq!(player.boosts.soulreaper_stacks, 4);
        assert_eq!(player.stats.hitpoints.current, 67);
        assert_eq!(player.max_hits.get(CombatType::Slash), 73);

        let _ = (player.attack)(&mut player, &mut monster, &mut rng, &mut limiter);
        assert_eq!(player.boosts.soulreaper_stacks, 5);
        assert_eq!(player.stats.hitpoints.current, 59);
        assert_eq!(player.max_hits.get(CombatType::Slash), 76);

        let _ = (player.attack)(&mut player, &mut monster, &mut rng, &mut limiter);
        assert_eq!(player.boosts.soulreaper_stacks, 5);
        assert_eq!(player.stats.hitpoints.current, 59);
        assert_eq!(player.max_hits.get(CombatType::Slash), 76);
    }
}
