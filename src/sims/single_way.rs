use crate::calc::monster_scaling::scale_monster_hp_only;
use crate::combat::attacks::standard::AttackFn;
use crate::combat::limiters::Limiter;
use crate::combat::mechanics::Mechanics;
use crate::combat::mechanics::handle_blood_fury;
use crate::combat::simulation::{FightResult, FightVars, Simulation};
use crate::combat::spec::CoreCondition;
use crate::combat::spec::SpecConfig;
use crate::combat::spec::SpecState;
use crate::combat::thralls::Thrall;
use crate::constants::P2_WARDEN_IDS;
use crate::error::SimulationError;
use crate::types::{monster::Monster, player::GearSwitch, player::Player};
use crate::utils::logging::FightLogger;
use rand::SeedableRng;
use rand::rngs::SmallRng;

pub struct SingleWayFight {
    pub player: Player,
    pub monster: Monster,
    pub limiter: Option<Box<dyn Limiter>>,
    pub rng: SmallRng,
    pub mechanics: SingleWayMechanics,
    pub logger: FightLogger,
    pub config: SingleWayConfig,
    pub spec_config: Option<SpecConfig<CoreCondition>>,
    pub spec_state: SpecState,
}

impl SingleWayFight {
    pub fn new(
        player: Player,
        monster: Monster,
        config: SingleWayConfig,
        spec_config: Option<SpecConfig<CoreCondition>>,
        use_logger: bool,
    ) -> Result<SingleWayFight, SimulationError> {
        let limiter = crate::combat::simulation::assign_limiter(&player, &monster);
        let rng = SmallRng::from_os_rng();
        let monster_name = monster.info.name.clone();
        let logger = FightLogger::new(use_logger, monster_name.as_str())
            .map_err(|e| SimulationError::ConfigError(format!("Error initializing logger: {e}")))?;

        Ok(SingleWayFight {
            player,
            monster,
            limiter,
            rng,
            mechanics: SingleWayMechanics,
            logger,
            config,
            spec_config,
            spec_state: SpecState::default(),
        })
    }
}

impl Simulation for SingleWayFight {
    fn simulate(&mut self) -> Result<FightResult, SimulationError> {
        simulate_fight(self)
    }

    fn is_immune(&self) -> bool {
        self.monster.is_immune(&self.player)
    }

    fn player(&self) -> &Player {
        &self.player
    }

    fn monster(&self) -> &Monster {
        &self.monster
    }

    fn set_attack_function(&mut self) {
        if P2_WARDEN_IDS.contains(&self.monster.info.id.unwrap_or_default()) {
            self.player.attack = crate::combat::attacks::standard::wardens_p2_attack as AttackFn;
        } else {
            self.player.attack =
                crate::combat::attacks::standard::get_attack_functions(&self.player);
            self.player.spec =
                crate::combat::attacks::specs::get_spec_attack_function(&self.player);
        }
    }

    fn reset(&mut self) {
        if let Some(ref mut spec_config) = self.spec_config {
            let restore_spec = self.spec_state.on_kill(&mut self.player, spec_config);
            self.player.reset_current_stats(restore_spec);
        }

        self.monster.reset();
        self.player.state.first_attack = true;
        self.player.state.last_attack_hit = true;
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SingleWayConfig {
    pub thralls: Option<Thrall>,
    pub remove_final_attack_delay: bool,
}

#[derive(Debug)]
pub struct SingleWayMechanics;

impl SingleWayMechanics {
    pub fn player_special_attack(
        fight: &mut SingleWayFight,
        fight_vars: &mut FightVars,
    ) -> Result<bool, SimulationError> {
        if let Some(ref mut spec_config) = fight.spec_config {
            for strategy in &mut spec_config.strategies {
                if !strategy.can_execute(&fight.player, &fight.monster, &()) {
                    continue;
                }

                // Make sure the current set of gear is added to the player's gear switches to allow switching back
                if fight.player.current_switch.is_none() {
                    let current_gear = GearSwitch::from(&fight.player);
                    fight.player.current_switch = Some(current_gear.switch_type.clone());
                    fight.player.switches.push(current_gear);
                }

                // Store the previous gear set's label for switching back after the spec
                let previous_switch = fight.player.current_switch.clone().unwrap();

                // Switch to the spec gear and perform the attack
                fight.player.switch(&strategy.switch_type)?;

                if fight.logger.enabled {
                    fight
                        .logger
                        .log_gear_switch(fight_vars.tick_counter, &strategy.switch_type);
                    let _ = fight.logger.log_current_player_rolls(&fight.player);
                    fight.logger.log_current_player_stats(&fight.player);
                    fight.logger.log_current_gear(&fight.player);
                }

                let hit = (fight.player.spec)(
                    &mut fight.player,
                    &mut fight.monster,
                    &mut fight.rng,
                    &mut fight.limiter,
                );

                if fight.logger.enabled {
                    fight.logger.log_player_spec(
                        fight_vars.tick_counter,
                        hit.damage,
                        hit.success,
                        &strategy.switch_type,
                    );
                }

                fight.player.state.first_attack = false;
                fight.monster.take_damage(hit.damage);

                if fight.logger.enabled {
                    fight.logger.log_monster_damage(
                        fight_vars.tick_counter,
                        hit.damage,
                        fight.monster.stats.hitpoints.current,
                        fight.monster.info.name.as_str(),
                    );
                    fight.logger.log_current_monster_stats(&fight.monster);
                    fight.logger.log_current_monster_rolls(&fight.monster);
                }

                strategy.state.attempt_count += 1;
                if hit.success {
                    strategy.state.success_count += 1;
                }

                handle_blood_fury(
                    &mut fight.player,
                    &hit,
                    fight_vars,
                    &mut fight.logger,
                    &mut fight.rng,
                );
                scale_monster_hp_only(&mut fight.monster, true);
                fight_vars.hit_attempts += 1;
                fight_vars.hit_count += u32::from(hit.success);
                fight_vars.hit_amounts.push(hit.damage);
                fight_vars.attack_tick += fight.player.gear.weapon.speed;

                fight.player.stats.spec.drain(strategy.spec_cost);
                if !fight.spec_state.spec_regen_timer.is_active() {
                    fight.spec_state.spec_regen_timer.activate();
                }

                // Switch back to the previous set of gear
                fight.player.switch(&previous_switch)?;

                if fight.logger.enabled {
                    fight
                        .logger
                        .log_gear_switch(fight_vars.tick_counter, &previous_switch);
                    let _ = fight.logger.log_current_player_rolls(&fight.player);
                }

                return Ok(true);
            }
        }
        Ok(false)
    }
}

impl Mechanics for SingleWayMechanics {}

fn simulate_fight(fight: &mut SingleWayFight) -> Result<FightResult, SimulationError> {
    if let Some(ref spec_config) = fight.spec_config
        && let Err(e) = spec_config.validate()
    {
        return Err(SimulationError::ConfigError(e));
    }

    fight
        .logger
        .log_initial_setup(&fight.player, &fight.monster);
    let mut vars = FightVars::new();
    scale_monster_hp_only(&mut fight.monster, true);

    while fight.monster.stats.hitpoints.current > 0 {
        if vars.tick_counter == vars.attack_tick {
            let did_spec = if let Some(ref spec_config) = fight.spec_config {
                if let Some(lowest) = spec_config.lowest_cost() {
                    if fight.player.stats.spec.value() >= lowest {
                        SingleWayMechanics::player_special_attack(fight, &mut vars)?
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };

            if !did_spec {
                fight.mechanics.player_attack(
                    &mut fight.player,
                    &mut fight.monster,
                    &mut fight.rng,
                    &fight.limiter,
                    &mut vars,
                    &mut fight.logger,
                );
            }
        }

        if let Some(thrall) = fight.config.thralls
            && vars.tick_counter == vars.thrall_attack_tick
        {
            fight.mechanics.thrall_attack(
                &mut fight.monster,
                thrall,
                &mut vars,
                &mut fight.rng,
                &mut fight.logger,
            );
        }

        fight
            .mechanics
            .process_monster_effects(&mut fight.monster, &vars, &mut fight.logger);
        fight
            .mechanics
            .process_freeze(&mut fight.monster, &mut vars, &mut fight.logger);
        fight
            .spec_state
            .increment_spec(&mut fight.player, vars.tick_counter, &mut fight.logger);
        fight.spec_state.increment_timers();
        if let Some(ref spec_config) = fight.spec_config {
            fight
                .spec_state
                .process_surge_potion(&mut fight.player, spec_config);
        }

        vars.tick_counter += 1;
    }

    fight.mechanics.get_fight_result(
        &fight.monster,
        &vars,
        &mut fight.logger,
        fight.config.remove_final_attack_delay,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calc::rolls::calc_active_player_rolls;
    use crate::types::equipment::{Armor, CombatStyle, Gear, Weapon};
    use crate::types::monster::Monster;
    use crate::types::player::Player;
    use crate::types::potions::Potion;
    use crate::types::prayers::Prayer;
    use crate::types::stats::PlayerStats;

    use std::rc::Rc;

    #[test]
    fn test_simulate_fight() {
        let mut player = Player::new();
        player.stats = PlayerStats::default();
        player.add_prayer(Prayer::Piety);
        player.add_potion(Potion::SuperCombat);

        player.gear = Rc::new(Gear {
            head: Some(Armor::new("Torva full helm", None).expect("Error creating equipment.")),
            neck: Some(Armor::new("Amulet of torture", None).expect("Error creating equipment.")),
            cape: Some(Armor::new("Infernal cape", None).expect("Error creating equipment.")),
            ammo: Some(Armor::new("Rada's blessing 4", None).expect("Error creating equipment.")),
            second_ammo: None,
            weapon: Weapon::new("Ghrazi rapier", None).expect("Error creating equipment."),
            shield: Some(Armor::new("Avernic defender", None).expect("Error creating equipment.")),
            body: Some(Armor::new("Torva platebody", None).expect("Error creating equipment.")),
            legs: Some(Armor::new("Torva platelegs", None).expect("Error creating equipment.")),
            hands: Some(Armor::new("Ferocious gloves", None).expect("Error creating equipment.")),
            feet: Some(Armor::new("Primordial boots", None).expect("Error creating equipment.")),
            ring: Some(Armor::new("Ultor ring", None).expect("Error creating equipment.")),
        });
        player.update_bonuses();
        player.set_active_style(CombatStyle::Lunge);
        let monster = Monster::new("Ammonite Crab", None).expect("Error creating monster.");
        calc_active_player_rolls(&mut player, &monster);

        let config = SingleWayConfig::default();
        let mut fight = SingleWayFight::new(player, monster, config, None, false)
            .expect("Error setting up single way fight.");
        let result = simulate_fight(&mut fight).expect("Simulation failed.");

        assert!(result.ttk_ticks > 0);
        assert!(result.hit_attempts > 0);
        assert!(result.hit_count > 0);
        assert!(!result.hit_amounts.is_empty());
    }
}
