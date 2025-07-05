use crate::calc::monster_scaling::scale_monster_hp_only;
use crate::combat::attacks::standard::AttackFn;
use crate::combat::limiters::Limiter;
use crate::combat::mechanics::Mechanics;
use crate::combat::mechanics::handle_blood_fury;
use crate::combat::simulation::{FightResult, FightVars, Simulation, SimulationError};
use crate::combat::thralls::Thrall;
use crate::constants;
use crate::constants::P2_WARDEN_IDS;
use crate::types::{monster::Monster, player::GearSwitch, player::Player};
use crate::utils::logging::FightLogger;
use rand::SeedableRng;
use rand::rngs::SmallRng;

#[derive(Debug, Clone, PartialEq)]
pub struct SpecConfig {
    pub strategies: Vec<SpecStrategy>,
    pub restore_policy: SpecRestorePolicy,
    lowest_cost: Option<u8>,
}

impl SpecConfig {
    pub fn new(strategies: Vec<SpecStrategy>, restore_policy: SpecRestorePolicy) -> Self {
        let lowest_cost = strategies.iter().map(|s| s.spec_cost).min();
        Self {
            strategies,
            restore_policy,
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

    pub fn lowest_cost(&self) -> u8 {
        if let Some(cost) = self.lowest_cost {
            cost
        } else {
            self.strategies
                .iter()
                .map(|s| s.spec_cost)
                .min()
                .expect("No special attacks found.")
        }
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
    pub gear: GearSwitch,
    pub conditions: Vec<SpecCondition>,
    pub state: SpecStrategyState,
    pub label: String,
    pub spec_cost: u8,
}

impl SpecStrategy {
    pub fn new(gear: GearSwitch, conditions: Option<Vec<SpecCondition>>, label: String) -> Self {
        let spec_cost = constants::SPEC_COSTS
            .iter()
            .find(|w| w.0 == gear.gear.weapon.name)
            .expect("Spec cost not found")
            .1;
        Self {
            gear,
            conditions: conditions.unwrap_or_default(),
            state: SpecStrategyState::default(),
            label,
            spec_cost,
        }
    }

    pub fn add_condition(&mut self, condition: SpecCondition) {
        self.conditions.push(condition);
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
    pub defence_reduced: u32,
    pub magic_reduced: u32,
    pub magic_def_reduced: u32,
    pub attack_reduced: u32,
    pub strength_reduced: u32,
    pub ranged_reduced: u32,
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
        self.player.reset_current_stats();
        self.monster.reset();
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SingleWayConfig {
    pub thralls: Option<Thrall>,
}

#[derive(Debug)]
pub struct SingleWayMechanics;

impl SingleWayMechanics {
    fn player_special_attack(
        &self,
        player: &mut Player,
        monster: &mut Monster,
        rng: &mut SmallRng,
        limiter: &Option<Box<dyn Limiter>>,
        fight_vars: &mut FightVars,
        spec_config: &mut SpecConfig,
        logger: &mut FightLogger,
    ) {
        for strategy in &spec_config.strategies {
            let all_conditions_met = strategy.conditions.iter().all(|condition| match condition {
                SpecCondition::MaxAttempts(attempts) => *attempts < strategy.state.attempt_count,
                SpecCondition::MinSuccesses(successes) => *successes < strategy.state.success_count,
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
            });

            if all_conditions_met && player.stats.spec.has_enough(strategy.spec_cost) {
                // Make sure the current set of gear is added to the player's gear switches to allow switching back
                if player.current_switch.is_none() {
                    let current_gear = GearSwitch::from(&player.clone());
                    player.switches.push(current_gear);
                }

                // Store the previous gear set's label for switching back after the spec
                let previous_switch = player.current_switch.clone().unwrap();

                // Switch to the spec gear and perform the attack
                player.switch(&strategy.gear.label);
                logger.log_gear_switch(fight_vars.tick_counter, &strategy.gear.label);
                let hit = (player.spec)(player, monster, rng, limiter);
                logger.log_player_spec(
                    fight_vars.tick_counter,
                    hit.damage,
                    hit.success,
                    strategy.label.clone(),
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

                player.stats.spec.drain(strategy.spec_cost);

                // Switch back to the previous set of gear
                player.switch(&previous_switch);
                logger.log_gear_switch(fight_vars.tick_counter, &previous_switch);

                return;
            }
        }
    }
}

impl Mechanics for SingleWayMechanics {}

fn simulate_fight(fight: &mut SingleWayFight) -> Result<FightResult, SimulationError> {
    let mut vars = FightVars::new();
    scale_monster_hp_only(&mut fight.monster);

    while fight.monster.stats.hitpoints.current > 0 {
        if vars.tick_counter == vars.attack_tick {
            // if let Some(ref mut specs) = &mut fight.spec_config
            //     && fight.player.stats.spec.value() >= specs.lowest_cost.unwrap_or(101)
            // {
            //     fight.mechanics.player_special_attack(
            //         &mut fight.player,
            //         &mut fight.monster,
            //         &mut fight.rng,
            //         &fight.limiter,
            //         &mut vars,
            //         specs,
            //         &mut fight.logger,
            //     );
            // } else {
            fight.mechanics.player_attack(
                &mut fight.player,
                &mut fight.monster,
                &mut fight.rng,
                &fight.limiter,
                &mut vars,
                &mut fight.logger,
            );
            // }
        }

        if let Some(thrall) = fight.config.thralls {
            if vars.tick_counter == vars.thrall_attack_tick {
                fight.mechanics.thrall_attack(
                    &mut fight.monster,
                    thrall,
                    &mut vars,
                    &mut fight.rng,
                    &mut fight.logger,
                );
            }
        }

        fight
            .mechanics
            .process_monster_effects(&mut fight.monster, &vars, &mut fight.logger);
        fight
            .mechanics
            .process_freeze(&mut fight.monster, &mut vars, &mut fight.logger);
        fight
            .mechanics
            .increment_tick(&mut fight.monster, &mut vars);
    }

    let remove_final_attack_delay = false;
    fight.mechanics.get_fight_result(
        &fight.player,
        &fight.monster,
        &vars,
        &mut fight.logger,
        remove_final_attack_delay,
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

    #[test]
    fn test_simulate_fight() {
        let mut player = Player::new();
        player.stats = PlayerStats::default();
        player.prayers.add(Prayer::Piety);
        player.add_potion(Potion::SuperCombat);

        player.gear = Gear {
            head: Some(Armor::new("Torva full helm", None)),
            neck: Some(Armor::new("Amulet of torture", None)),
            cape: Some(Armor::new("Infernal cape", None)),
            ammo: Some(Armor::new("Rada's blessing 4", None)),
            second_ammo: None,
            weapon: Weapon::new("Ghrazi rapier", None),
            shield: Some(Armor::new("Avernic defender", None)),
            body: Some(Armor::new("Torva platebody", None)),
            legs: Some(Armor::new("Torva platelegs", None)),
            hands: Some(Armor::new("Ferocious gloves", None)),
            feet: Some(Armor::new("Primordial boots", None)),
            ring: Some(Armor::new("Ultor ring", None)),
        };
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
