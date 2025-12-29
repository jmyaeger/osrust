use crate::calc::monster_scaling::scale_monster_hp_only;
use crate::combat::attacks::standard::AttackFn;
use crate::combat::limiters::Limiter;
use crate::combat::mechanics::Mechanics;
use crate::combat::mechanics::handle_blood_fury;
use crate::combat::simulation::{FightResult, FightVars, Simulation, SimulationError};
use crate::combat::thralls::Thrall;
use crate::constants;
use crate::constants::P2_WARDEN_IDS;
use crate::types::player::SwitchType;
use crate::types::timers::Timer;
use crate::types::{monster::Monster, player::GearSwitch, player::Player};
use crate::utils::logging::FightLogger;
use rand::SeedableRng;
use rand::rngs::SmallRng;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum DeathCharge {
    Single,
    Double,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpecConfig {
    pub strategies: Vec<SpecStrategy>,
    pub restore_policy: SpecRestorePolicy,
    pub death_charge: Option<DeathCharge>,
    pub surge_potion: bool,
    lowest_cost: Option<u8>,
}

impl SpecConfig {
    pub fn new(
        strategies: Vec<SpecStrategy>,
        restore_policy: SpecRestorePolicy,
        death_charge: Option<DeathCharge>,
        surge_potion: bool,
    ) -> Self {
        let lowest_cost = strategies.iter().map(|s| s.spec_cost).min();
        Self {
            strategies,
            restore_policy,
            death_charge,
            surge_potion,
            lowest_cost,
        }
    }

    pub fn add_strategy(&mut self, strategy: SpecStrategy, position: Option<usize>) {
        if let Some(ind) = position {
            self.strategies.insert(ind, strategy);
        } else {
            self.strategies.push(strategy);
        }
    }

    pub fn lowest_cost(&self) -> Option<u8> {
        self.lowest_cost
            .or_else(|| self.strategies.iter().map(|s| s.spec_cost).min())
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.strategies.is_empty() {
            return Err("No spec strategies defined".to_string());
        }

        for strategy in &self.strategies {
            self.validate_conditions(&strategy.conditions)?;
        }

        Ok(())
    }

    fn validate_conditions(&self, conditions: &[SpecCondition]) -> Result<(), String> {
        // Check for HP conflicts
        let hp_above: Vec<_> = conditions
            .iter()
            .filter_map(|c| {
                if let SpecCondition::MonsterHpAbove(hp) = c {
                    Some(*hp)
                } else {
                    None
                }
            })
            .collect();

        let hp_below: Vec<_> = conditions
            .iter()
            .filter_map(|c| {
                if let SpecCondition::MonsterHpBelow(hp) = c {
                    Some(*hp)
                } else {
                    None
                }
            })
            .collect();

        for above in &hp_above {
            for below in &hp_below {
                if above >= below {
                    return Err(format!(
                        "Conflicting HP conditions: above {above} and below {below}"
                    ));
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpecRestorePolicy {
    RestoreEveryKill,
    RestoreAfter(u32),
    NeverRestore,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpecStrategy {
    pub conditions: Vec<SpecCondition>,
    pub state: SpecStrategyState,
    pub switch_type: SwitchType,
    pub spec_cost: u8,
}

impl SpecStrategy {
    pub fn new(gear: &GearSwitch, conditions: Option<Vec<SpecCondition>>) -> Self {
        let spec_cost = constants::SPEC_COSTS
            .iter()
            .find(|w| w.0 == gear.gear.weapon.name)
            .expect("Spec cost not found")
            .1;

        Self {
            conditions: conditions.unwrap_or_default(),
            state: SpecStrategyState::default(),
            switch_type: gear.switch_type.clone(),
            spec_cost,
        }
    }

    pub fn add_condition(&mut self, condition: SpecCondition) {
        self.conditions.push(condition);
    }

    pub fn can_execute(&self, player: &Player, monster: &Monster) -> bool {
        if !player.stats.spec.has_enough(self.spec_cost) {
            return false;
        }
        self.conditions
            .iter()
            .all(|condition| self.evaluate_condition(condition, player, monster))
    }

    fn evaluate_condition(
        &self,
        condition: &SpecCondition,
        player: &Player,
        monster: &Monster,
    ) -> bool {
        match condition {
            SpecCondition::MaxAttempts(attempts) => self.state.attempt_count < *attempts,
            SpecCondition::MinSuccesses(successes) => self.state.success_count < *successes,
            SpecCondition::MonsterHpAbove(hp) => monster.stats.hitpoints.current > *hp,
            SpecCondition::MonsterHpBelow(hp) => monster.stats.hitpoints.current <= *hp,
            SpecCondition::PlayerHpAbove(hp) => player.stats.hitpoints.current > *hp,
            SpecCondition::PlayerHpBelow(hp) => player.stats.hitpoints.current <= *hp,
            SpecCondition::TargetAttackReduction(amt) => {
                monster
                    .stats
                    .attack
                    .base
                    .saturating_sub(monster.stats.attack.current)
                    < *amt
            }
            SpecCondition::TargetStrengthReduction(amt) => {
                monster
                    .stats
                    .strength
                    .base
                    .saturating_sub(monster.stats.strength.current)
                    < *amt
            }
            SpecCondition::TargetDefenceReduction(amt) => {
                monster
                    .stats
                    .defence
                    .base
                    .saturating_sub(monster.stats.defence.current)
                    < *amt
            }
            SpecCondition::TargetRangedReduction(amt) => {
                monster
                    .stats
                    .ranged
                    .base
                    .saturating_sub(monster.stats.ranged.current)
                    < *amt
            }
            SpecCondition::TargetMagicReduction(amt) => {
                monster
                    .stats
                    .magic
                    .base
                    .saturating_sub(monster.stats.magic.current)
                    < *amt
            }
            SpecCondition::TargetMagicDefReduction(amt) => {
                monster
                    .bonuses
                    .defence
                    .magic_base
                    .saturating_sub(monster.bonuses.defence.magic)
                    < *amt
            }
        }
    }

    pub fn builder(gear: &GearSwitch) -> SpecStrategyBuilder {
        SpecStrategyBuilder::new(gear)
    }
}

#[derive(Debug)]
pub struct SpecStrategyBuilder {
    strategy: SpecStrategy,
}

impl SpecStrategyBuilder {
    pub fn new(gear: &GearSwitch) -> Self {
        Self {
            strategy: SpecStrategy::new(gear, None),
        }
    }

    fn with_condition(mut self, condition: SpecCondition) -> Self {
        self.strategy.add_condition(condition);
        self
    }

    pub fn with_max_attempts(self, attempts: u8) -> Self {
        self.with_condition(SpecCondition::MaxAttempts(attempts))
    }

    pub fn with_min_successes(self, successes: u8) -> Self {
        self.with_condition(SpecCondition::MinSuccesses(successes))
    }

    pub fn with_monster_hp_below(self, hp: u32) -> Self {
        self.with_condition(SpecCondition::MonsterHpBelow(hp))
    }

    pub fn with_monster_hp_above(self, hp: u32) -> Self {
        self.with_condition(SpecCondition::MonsterHpAbove(hp))
    }

    pub fn with_target_def_reduction(self, amount: u32) -> Self {
        self.with_condition(SpecCondition::TargetDefenceReduction(amount))
    }

    pub fn build(self) -> SpecStrategy {
        self.strategy
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpecCondition {
    MonsterHpBelow(u32),
    MonsterHpAbove(u32),
    PlayerHpBelow(u32),
    PlayerHpAbove(u32),
    MaxAttempts(u8),
    MinSuccesses(u8),
    TargetDefenceReduction(u32),
    TargetMagicReduction(u32),
    TargetMagicDefReduction(i32),
    TargetAttackReduction(u32),
    TargetStrengthReduction(u32),
    TargetRangedReduction(u32),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpecStrategyState {
    pub attempt_count: u8,
    pub success_count: u8,
}

impl SpecStrategyState {
    pub fn reset(&mut self) {
        self.attempt_count = 0;
        self.success_count = 0;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SingleWayState {
    pub kill_counter: u32,
    pub death_charge_procs: u8,
    pub death_charge_cd: Timer,
    pub surge_potion_cd: Timer,
    pub spec_regen_timer: Timer,
}

impl Default for SingleWayState {
    fn default() -> Self {
        Self {
            kill_counter: 0,
            death_charge_procs: 0,
            death_charge_cd: Timer::new(constants::DEATH_CHARGE_CD),
            surge_potion_cd: Timer::new(constants::SURGE_POTION_CD),
            spec_regen_timer: Timer::new(constants::FULL_SPEC_REGEN_TIME),
        }
    }
}

impl SingleWayState {
    pub fn reset(&mut self) {
        self.kill_counter = 0;
        self.death_charge_procs = 0;
        self.death_charge_cd.reset();
        self.surge_potion_cd.reset();
        self.spec_regen_timer.reset();
    }
}

pub struct SingleWayFight {
    pub player: Player,
    pub monster: Monster,
    pub limiter: Option<Box<dyn Limiter>>,
    pub rng: SmallRng,
    pub mechanics: SingleWayMechanics,
    pub logger: FightLogger,
    pub config: SingleWayConfig,
    pub spec_config: Option<SpecConfig>,
    pub state: SingleWayState,
}

impl SingleWayFight {
    pub fn new(
        player: Player,
        monster: Monster,
        config: SingleWayConfig,
        spec_config: Option<SpecConfig>,
        use_logger: bool,
    ) -> SingleWayFight {
        let limiter = crate::combat::simulation::assign_limiter(&player, &monster);
        let rng = SmallRng::from_os_rng();
        let monster_name = monster.info.name.clone();
        SingleWayFight {
            player,
            monster,
            limiter,
            rng,
            mechanics: SingleWayMechanics,
            logger: FightLogger::new(use_logger, monster_name.as_str()),
            config,
            spec_config,
            state: SingleWayState::default(),
        }
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
        if let Some(spec_config) = &self.spec_config {
            let restore_spec = match spec_config.restore_policy {
                SpecRestorePolicy::NeverRestore => false,
                SpecRestorePolicy::RestoreAfter(kills) => self.state.kill_counter >= kills,
                SpecRestorePolicy::RestoreEveryKill => true,
            };
            self.player.reset_current_stats(restore_spec);
            if restore_spec {
                self.state.reset();
            }
        }

        if let Some(ref mut spec_config) = self.spec_config {
            spec_config
                .strategies
                .iter_mut()
                .for_each(|s| s.state.reset());
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
    pub fn player_special_attack(fight: &mut SingleWayFight, fight_vars: &mut FightVars) -> bool {
        if let Some(ref mut spec_config) = fight.spec_config {
            for strategy in &mut spec_config.strategies {
                if !strategy.can_execute(&fight.player, &fight.monster) {
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
                fight.player.switch(&strategy.switch_type);

                if fight.logger.enabled {
                    fight
                        .logger
                        .log_gear_switch(fight_vars.tick_counter, &strategy.switch_type);
                    fight.logger.log_current_player_rolls(&fight.player);
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
                    fight.logger.log_monster_damage(
                        fight_vars.tick_counter,
                        hit.damage,
                        fight.monster.stats.hitpoints.current,
                        fight.monster.info.name.as_str(),
                    );
                    fight.logger.log_current_monster_stats(&fight.monster);
                    fight.logger.log_current_monster_rolls(&fight.monster);
                }

                fight.player.state.first_attack = false;
                fight.monster.take_damage(hit.damage);

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
                fight_vars.hit_count += if hit.success { 1 } else { 0 };
                fight_vars.hit_amounts.push(hit.damage);
                fight_vars.attack_tick += fight.player.gear.weapon.speed;

                fight.player.stats.spec.drain(strategy.spec_cost);
                if !fight.state.spec_regen_timer.is_active() {
                    fight.state.spec_regen_timer.activate();
                }

                // Switch back to the previous set of gear
                fight.player.switch(&previous_switch);

                if fight.logger.enabled {
                    fight
                        .logger
                        .log_gear_switch(fight_vars.tick_counter, &previous_switch);
                    fight.logger.log_current_player_rolls(&fight.player);
                }

                return true;
            }
        }
        false
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
                        SingleWayMechanics::player_special_attack(fight, &mut vars)
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
        fight.mechanics.increment_spec(
            &mut fight.player,
            &mut vars,
            &mut fight.state,
            &mut fight.logger,
        );
        fight.mechanics.increment_timers(&mut fight.state);
        fight.mechanics.process_surge_potion(
            &mut fight.player,
            &fight.spec_config,
            &mut fight.state,
        );

        vars.tick_counter += 1;
    }

    fight.state.kill_counter += 1;
    fight
        .mechanics
        .process_death_charge(&mut fight.player, &fight.spec_config, &mut fight.state);
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
    use crate::calc::rolls::calc_player_melee_rolls;
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
            head: Some(Armor::new("Torva full helm", None).unwrap()),
            neck: Some(Armor::new("Amulet of torture", None).unwrap()),
            cape: Some(Armor::new("Infernal cape", None).unwrap()),
            ammo: Some(Armor::new("Rada's blessing 4", None).unwrap()),
            second_ammo: None,
            weapon: Weapon::new("Ghrazi rapier", None).unwrap(),
            shield: Some(Armor::new("Avernic defender", None).unwrap()),
            body: Some(Armor::new("Torva platebody", None).unwrap()),
            legs: Some(Armor::new("Torva platelegs", None).unwrap()),
            hands: Some(Armor::new("Ferocious gloves", None).unwrap()),
            feet: Some(Armor::new("Primordial boots", None).unwrap()),
            ring: Some(Armor::new("Ultor ring", None).unwrap()),
        });
        player.update_bonuses();
        player.set_active_style(CombatStyle::Lunge);
        let monster = Monster::new("Ammonite Crab", None).unwrap();
        calc_player_melee_rolls(&mut player, &monster);

        let config = SingleWayConfig::default();
        let mut fight = SingleWayFight::new(player, monster, config, None, false);
        let result = simulate_fight(&mut fight).unwrap();

        assert!(result.ttk_ticks > 0);
        assert!(result.hit_attempts > 0);
        assert!(result.hit_count > 0);
        assert!(!result.hit_amounts.is_empty());
    }
}
