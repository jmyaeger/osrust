use crate::calc::monster_scaling::scale_monster_hp_only;
use crate::combat::attacks::effects::CombatEffect;
use crate::combat::limiters::Limiter;
use crate::combat::simulation::FightVars;
use crate::combat::simulation::{FightResult, SimulationError};
use crate::combat::thralls::Thrall;
use crate::constants::{self, THRALL_ATTACK_SPEED};
use crate::types::monster::{AttackType, Monster};
use crate::types::player::Player;
use crate::utils::logging::FightLogger;
use rand::Rng;
use rand::rngs::SmallRng;

use super::attacks::standard::Hit;

pub trait Mechanics {
    fn player_attack(
        &self,
        player: &mut Player,
        monster: &mut Monster,
        rng: &mut SmallRng,
        limiter: &Option<Box<dyn Limiter>>,
        fight_vars: &mut FightVars,
        logger: &mut FightLogger,
    ) {
        let hit = (player.attack)(player, monster, rng, limiter);
        logger.log_player_attack(
            fight_vars.tick_counter,
            hit.damage,
            hit.success,
            player.combat_type(),
        );
        player.boosts.first_attack = false;
        monster.take_damage(hit.damage);
        logger.log_monster_damage(
            fight_vars.tick_counter,
            hit.damage,
            monster.stats.hitpoints.current,
            monster.info.name.as_str(),
        );

        handle_blood_fury(player, &hit, fight_vars, logger, rng);

        scale_monster_hp_only(monster);
        fight_vars.hit_attempts += 1;
        fight_vars.hit_count += if hit.success { 1 } else { 0 };
        fight_vars.hit_amounts.push(hit.damage);
        fight_vars.attack_tick += player.gear.weapon.speed;
    }

    fn monster_attack(
        &self,
        monster: &mut Monster,
        player: &mut Player,
        attack_type: Option<AttackType>,
        fight_vars: &mut FightVars,
        rng: &mut SmallRng,
        logger: &mut FightLogger,
    ) {
        // Note: does not increment monster attack tick for flexibility
        let hit = monster.attack(player, attack_type, rng, true);
        logger.log_monster_attack(
            monster,
            fight_vars.tick_counter,
            hit.damage,
            hit.success,
            attack_type,
        );
        player.take_damage(hit.damage);
        logger.log_player_damage(
            fight_vars.tick_counter,
            hit.damage,
            player.stats.hitpoints.current,
        );
        fight_vars.damage_taken += hit.damage;

        handle_recoil(player, monster, &hit, fight_vars, logger);
    }

    fn thrall_attack(
        &self,
        monster: &mut Monster,
        thrall: Thrall,
        fight_vars: &mut FightVars,
        rng: &mut SmallRng,
        logger: &mut FightLogger,
    ) {
        if monster.is_immune_to_thrall(thrall) {
            logger.log_custom(
                fight_vars.tick_counter,
                format!(
                    "Thrall hit for 0 damage because {} is immune to it.",
                    monster.info.name
                )
                .as_str(),
            );
            return;
        }

        let thrall_hit = std::cmp::min(
            rng.random_range(0..=thrall.max_hit()),
            monster.stats.hitpoints.current,
        );
        logger.log_thrall_attack(fight_vars.tick_counter, thrall_hit);
        monster.take_damage(thrall_hit);
        logger.log_monster_damage(
            fight_vars.tick_counter,
            thrall_hit,
            monster.stats.hitpoints.current,
            monster.info.name.as_str(),
        );
        scale_monster_hp_only(monster);

        fight_vars.thrall_attack_tick += THRALL_ATTACK_SPEED;
        fight_vars.thrall_damage += thrall_hit;
    }

    fn process_monster_effects(
        &self,
        monster: &mut Monster,
        fight_vars: &FightVars,
        logger: &mut FightLogger,
    ) {
        // Process effects and apply damage
        let mut effect_damage = 0;
        for effect in &mut monster.active_effects {
            effect_damage += effect.apply();
        }

        if effect_damage > 0 {
            monster.take_damage(effect_damage);
            logger.log_monster_effect_damage(
                fight_vars.tick_counter,
                effect_damage,
                monster.info.name.as_str(),
            );
            scale_monster_hp_only(monster);
        }

        monster.clear_inactive_effects();
    }

    fn process_freeze(
        &self,
        monster: &mut Monster,
        fight_vars: &mut FightVars,
        logger: &mut FightLogger,
    ) {
        // Decrement freeze duration if it's active
        if monster.info.freeze_duration > 0 {
            monster.info.freeze_duration -= 1;
            if monster.info.freeze_duration == 0 {
                logger.log_freeze_end(fight_vars.tick_counter, monster.info.name.as_str());
                // 5 tick freeze immunity when it runs out
                fight_vars.freeze_immunity = 5;
                monster.immunities.freeze = 100;
            }
        }

        // Decrement temporary freeze immunity if applicable
        if fight_vars.freeze_immunity > 0 {
            fight_vars.freeze_immunity -= 1;
            if fight_vars.freeze_immunity == 0 {
                // Reset freeze resistance to original value when immunity runs out
                monster.immunities.freeze = fight_vars.freeze_resistance;
            }
        }
    }

    fn increment_tick(&self, monster: &mut Monster, fight_vars: &mut FightVars) {
        // Add the attack cooldown on the last hit (for continuous TTK)
        if monster.stats.hitpoints.current == 0 {
            fight_vars.tick_counter = fight_vars.attack_tick;
        } else {
            // Increment tick counter
            fight_vars.tick_counter += 1;
        }
    }

    fn increment_spec_timer(
        &self,
        player: &mut Player,
        fight_vars: &mut FightVars,
        logger: &mut FightLogger,
    ) {
        if let Some(ref mut timer) = fight_vars.spec_regen_timer {
            *timer += 1;
            if (player.is_wearing("Lightbearer", None) && *timer % 25 == 0) || *timer % 50 == 0 {
                player.stats.spec.regen();
                logger.log_custom(
                    fight_vars.tick_counter,
                    "Player has regenerated 10 special attack energy",
                );
            }
            if player.stats.spec.is_full() {
                fight_vars.spec_regen_timer = None;
            }
        }
    }

    fn get_fight_result(
        &self,
        player: &Player,
        monster: &Monster,
        fight_vars: &FightVars,
        logger: &mut FightLogger,
        remove_final_attack_delay: bool,
    ) -> Result<FightResult, SimulationError> {
        logger.log_monster_death(fight_vars.tick_counter, monster.info.name.as_str());
        let ttk_ticks = fight_vars.tick_counter;
        let leftover_burn = calc_leftover_burn(monster);
        let mut result = FightResult {
            ttk_ticks,
            hit_attempts: fight_vars.hit_attempts,
            hit_count: fight_vars.hit_count,
            hit_amounts: fight_vars.hit_amounts.clone(),
            food_eaten: fight_vars.food_eaten,
            damage_taken: fight_vars.damage_taken,
            leftover_burn,
            thrall_damage: fight_vars.thrall_damage,
        };

        if remove_final_attack_delay {
            result.remove_final_attack_delay(player.gear.weapon.speed);
        }

        Ok(result)
    }

    fn process_player_death(
        &self,
        fight_vars: &FightVars,
        monster: &Monster,
        logger: &mut FightLogger,
    ) -> Result<FightResult, SimulationError> {
        logger.log_player_death(fight_vars.tick_counter);
        let leftover_burn = calc_leftover_burn(monster);

        Err(SimulationError::PlayerDeathError(FightResult {
            ttk_ticks: fight_vars.tick_counter,
            hit_attempts: fight_vars.hit_attempts,
            hit_count: fight_vars.hit_count,
            hit_amounts: fight_vars.hit_amounts.clone(),
            food_eaten: fight_vars.food_eaten,
            damage_taken: fight_vars.damage_taken,
            leftover_burn,
            thrall_damage: fight_vars.thrall_damage,
        }))
    }

    fn monster_regen_hp(
        &self,
        monster: &mut Monster,
        fight_vars: &FightVars,
        logger: &mut FightLogger,
    ) {
        monster.heal(1);
        logger.log_hp_regen(
            fight_vars.tick_counter,
            monster.stats.hitpoints.current,
            monster.info.name.as_str(),
        );
    }

    fn monster_regen_stats(
        &self,
        monster: &mut Monster,
        fight_vars: &FightVars,
        logger: &mut FightLogger,
    ) {
        monster.regen_stats();
        logger.log_stats_regen(fight_vars.tick_counter, monster.info.name.as_str());
    }

    fn player_regen(&self, player: &mut Player, fight_vars: &FightVars, logger: &mut FightLogger) {
        player.regen_all_stats();
        logger.log_hp_regen(
            fight_vars.tick_counter,
            player.stats.hitpoints.current,
            "Player",
        );
        logger.log_stats_regen(fight_vars.tick_counter, "Player");
    }

    fn decrement_eat_delay(&self, fight_vars: &mut FightVars) {
        if fight_vars.eat_delay > 0 {
            fight_vars.eat_delay -= 1;
        }
    }

    fn eat_food(
        &self,
        player: &mut Player,
        heal_amount: u32,
        overheal: Option<u32>,
        fight_vars: &mut FightVars,
        logger: &mut FightLogger,
    ) {
        // Note: Does not increment attack delay for flexibility
        player.heal(heal_amount, overheal);
        logger.log_food_eaten(
            fight_vars.tick_counter,
            heal_amount,
            player.stats.hitpoints.current,
        );
        fight_vars.food_eaten += 1;
        fight_vars.eat_delay = constants::EAT_DELAY;
    }
}

fn calc_leftover_burn(monster: &Monster) -> u32 {
    if let Some(CombatEffect::Burn {
        tick_counter: _,
        stacks,
    }) = monster
        .active_effects
        .iter()
        .find(|item| matches!(item, &CombatEffect::Burn { .. }))
    {
        stacks.iter().sum()
    } else {
        0
    }
}

pub fn handle_recoil(
    player: &Player,
    monster: &mut Monster,
    hit: &Hit,
    fight_vars: &mut FightVars,
    logger: &mut FightLogger,
) {
    if !constants::IMMUNE_TO_RECOIL_MONSTERS.contains(&monster.info.id.unwrap_or_default())
        && hit.damage > 0
    {
        if player.is_wearing_any(vec![
            ("Ring of suffering", Some("Recoil")),
            ("Ring of suffering (i)", Some("Recoil")),
            ("Ring of recoil", None),
        ]) {
            let recoil_damage = hit.damage / 10 + 1;
            monster.take_damage(recoil_damage);
            logger.log_custom(
                fight_vars.tick_counter,
                format!("{} took {} recoil damage", monster.info.name, recoil_damage).as_str(),
            );
        }

        if player.is_wearing("Echo boots", None) && player.is_using_melee() {
            monster.take_damage(1);
            logger.log_custom(
                fight_vars.tick_counter,
                format!("{} took 1 recoil damage from echo boots", monster.info.name).as_str(),
            );
        }
    }
}

pub fn handle_blood_fury(
    player: &mut Player,
    hit: &Hit,
    fight_vars: &mut FightVars,
    logger: &mut FightLogger,
    rng: &mut SmallRng,
) {
    if player.is_wearing("Amulet of blood fury", None) && rng.random_range(0..5) == 0 {
        player.heal(hit.damage * 3 / 10, None);
        logger.log_custom(
            fight_vars.tick_counter,
            format!("Blood fury healed for {} HP", hit.damage * 3 / 10).as_str(),
        );
    }
}
