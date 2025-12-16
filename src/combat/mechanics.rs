use crate::calc::monster_scaling::scale_monster_hp_only;
use crate::combat::attacks::effects::CombatEffect;
use crate::combat::limiters::Limiter;
use crate::combat::simulation::FightVars;
use crate::combat::simulation::{FightResult, SimulationError};
use crate::combat::thralls::Thrall;
use crate::constants::{self, THRALL_ATTACK_SPEED};
use crate::sims::single_way::{DeathCharge, SingleWayState, SpecConfig};
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
        if logger.enabled {
            logger.log_player_attack(
                fight_vars.tick_counter,
                hit.damage,
                hit.success,
                player.combat_type(),
            );
            logger.log_monster_damage(
                fight_vars.tick_counter,
                hit.damage,
                monster.stats.hitpoints.current,
                monster.info.name.as_str(),
            );
        }

        player.state.first_attack = false;
        player.state.last_attack_hit = hit.success;
        monster.take_damage(hit.damage);

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

        if logger.enabled {
            logger.log_monster_attack(
                monster,
                fight_vars.tick_counter,
                hit.damage,
                hit.success,
                attack_type,
            );
            logger.log_player_damage(
                fight_vars.tick_counter,
                hit.damage,
                player.stats.hitpoints.current,
            );
        }
        player.take_damage(hit.damage);
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
        if logger.enabled && monster.is_immune_to_thrall(thrall) {
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
            rng.random_range(0..thrall.max_hit() + 1),
            monster.stats.hitpoints.current,
        );

        if logger.enabled {
            logger.log_thrall_attack(fight_vars.tick_counter, thrall_hit);
            logger.log_monster_damage(
                fight_vars.tick_counter,
                thrall_hit,
                monster.stats.hitpoints.current,
                monster.info.name.as_str(),
            );
        }
        monster.take_damage(thrall_hit);
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
            match effect {
                CombatEffect::Burn { .. } => {
                    let mut burn_damage = effect.apply();
                    let monster_version = monster.info.version.as_ref().map_or("", |s| s.as_str());
                    let monster_name = monster.info.name.as_str();
                    if monster_version == "Left claw"
                        || monster_version == "Right claw"
                        || monster_name == "Ice demon"
                    {
                        burn_damage /= 3;
                    } else if monster_name == "Corporeal Beast" {
                        burn_damage /= 2;
                    }

                    effect_damage += burn_damage;
                }
                _ => {
                    effect_damage += effect.apply();
                }
            }
        }

        if effect_damage > 0 {
            monster.take_damage(effect_damage);

            if logger.enabled {
                logger.log_monster_effect_damage(
                    fight_vars.tick_counter,
                    effect_damage,
                    monster.info.name.as_str(),
                );
            }

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
                if logger.enabled {
                    logger.log_freeze_end(fight_vars.tick_counter, monster.info.name.as_str());
                }

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

    fn increment_spec(
        &self,
        player: &mut Player,
        fight_vars: &mut FightVars,
        state: &mut SingleWayState,
        logger: &mut FightLogger,
    ) {
        if state.spec_regen_timer.is_active() {
            state.spec_regen_timer.increment();
            if (player.is_wearing("Lightbearer", None)
                && state.spec_regen_timer.counter().is_multiple_of(25))
                || state.spec_regen_timer.counter().is_multiple_of(50)
            {
                player.stats.spec.regen();
                if logger.enabled {
                    logger.log_custom(
                        fight_vars.tick_counter,
                        "Player has regenerated 10 special attack energy",
                    );
                }
            }
            if player.stats.spec.is_full() {
                state.spec_regen_timer.reset();
            }
        }
    }

    fn get_fight_result(
        &self,
        monster: &Monster,
        fight_vars: &FightVars,
        logger: &mut FightLogger,
        remove_final_attack_delay: bool,
    ) -> Result<FightResult, SimulationError> {
        if logger.enabled {
            logger.log_monster_death(fight_vars.tick_counter, monster.info.name.as_str());
        }

        let ttk_ticks = if remove_final_attack_delay {
            fight_vars.tick_counter
        } else {
            fight_vars.attack_tick
        };
        let leftover_burn = calc_leftover_burn(monster);

        Ok(FightResult {
            ttk_ticks,
            hit_attempts: fight_vars.hit_attempts,
            hit_count: fight_vars.hit_count,
            hit_amounts: fight_vars.hit_amounts.clone(),
            food_eaten: fight_vars.food_eaten,
            damage_taken: fight_vars.damage_taken,
            leftover_burn,
            thrall_damage: fight_vars.thrall_damage,
        })
    }

    fn process_player_death(
        &self,
        fight_vars: &FightVars,
        monster: &Monster,
        logger: &mut FightLogger,
    ) -> Result<FightResult, SimulationError> {
        if logger.enabled {
            logger.log_player_death(fight_vars.tick_counter);
        }

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

        if logger.enabled {
            logger.log_hp_regen(
                fight_vars.tick_counter,
                monster.stats.hitpoints.current,
                monster.info.name.as_str(),
            );
        }
    }

    fn monster_regen_stats(
        &self,
        monster: &mut Monster,
        fight_vars: &FightVars,
        logger: &mut FightLogger,
    ) {
        monster.regen_stats();

        if logger.enabled {
            logger.log_stats_regen(fight_vars.tick_counter, monster.info.name.as_str());
        }
    }

    fn player_regen(&self, player: &mut Player, fight_vars: &FightVars, logger: &mut FightLogger) {
        player.regen_all_stats();

        if logger.enabled {
            logger.log_hp_regen(
                fight_vars.tick_counter,
                player.stats.hitpoints.current,
                "Player",
            );
            logger.log_stats_regen(fight_vars.tick_counter, "Player");
        }
    }

    fn decrement_eat_delay(&self, fight_vars: &mut FightVars) {
        if fight_vars.eat_delay > 0 {
            fight_vars.eat_delay -= 1;
        }
    }

    fn increment_timers(&self, state: &mut SingleWayState) {
        state.death_charge_cd.increment();
        state.surge_potion_cd.increment();

        if state.death_charge_cd.counter() == 0 && !state.death_charge_cd.is_active() {
            state.death_charge_procs = 0;
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

        if logger.enabled {
            logger.log_food_eaten(
                fight_vars.tick_counter,
                heal_amount,
                player.stats.hitpoints.current,
            );
        }

        fight_vars.food_eaten += 1;
        fight_vars.eat_delay = constants::EAT_DELAY;
    }

    fn process_death_charge(
        &self,
        player: &mut Player,
        spec_config: &Option<SpecConfig>,
        state: &mut SingleWayState,
    ) {
        if let Some(config) = spec_config {
            if state.death_charge_cd.is_active() {
                let max_procs = match config.death_charge {
                    Some(DeathCharge::Single) => 1,
                    Some(DeathCharge::Double) => 2,
                    None => 0,
                };
                if state.death_charge_procs < max_procs {
                    player.stats.spec.death_charge();
                    state.death_charge_procs += 1;
                }
            } else if config.death_charge.is_some() {
                player.stats.spec.death_charge();
                state.death_charge_procs += 1;
                state.death_charge_cd.activate();
            }
        }
    }

    fn process_surge_potion(
        &self,
        player: &mut Player,
        spec_config: &Option<SpecConfig>,
        state: &mut SingleWayState,
    ) {
        if let Some(config) = spec_config
            && config.surge_potion
            && player.stats.spec.value() <= 75
            && !state.surge_potion_cd.is_active()
        {
            player.stats.spec.surge_potion();
            state.surge_potion_cd.activate();
        }
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

            if logger.enabled {
                logger.log_custom(
                    fight_vars.tick_counter,
                    format!("{} took {} recoil damage", monster.info.name, recoil_damage).as_str(),
                );
            }
        }

        if player.is_wearing("Echo boots", None) && player.is_using_melee() {
            monster.take_damage(1);

            if logger.enabled {
                logger.log_custom(
                    fight_vars.tick_counter,
                    format!("{} took 1 recoil damage from echo boots", monster.info.name).as_str(),
                );
            }
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

        if logger.enabled {
            logger.log_custom(
                fight_vars.tick_counter,
                format!("Blood fury healed for {} HP", hit.damage * 3 / 10).as_str(),
            );
        }
    }
}
