use crate::calc::monster_scaling::scale_monster_hp_only;
use crate::combat::limiters::Limiter;
use crate::combat::mechanics::{Mechanics, handle_recoil};
use crate::combat::simulation::{
    FightResult, FightVars, Simulation, SimulationError, assign_limiter,
};
use crate::constants;
use crate::types::monster::{AttackType, Monster, MonsterMaxHit};
use crate::types::player::Player;
use crate::utils::logging::FightLogger;
use rand::SeedableRng;
use rand::rngs::SmallRng;

const VARDORVIS_ATTACK_STYLE: AttackType = AttackType::Slash;
const VARDORVIS_ATTACK_SPEED: i32 = 5;
const VARDORVIS_REGEN_TICKS: i32 = 100;

#[derive(Debug, PartialEq, Clone)]
pub struct VardorvisConfig {
    pub food_heal_amount: u32,
    pub food_eat_delay: i32,
    pub eat_strategy: VardorvisEatStrategy,
    pub logger: FightLogger,
}

impl Default for VardorvisConfig {
    fn default() -> Self {
        Self {
            food_heal_amount: 22,
            food_eat_delay: 3,
            eat_strategy: VardorvisEatStrategy::EatAtHp(20),
            logger: FightLogger::new(false, "vardorvis"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum VardorvisEatStrategy {
    EatAtHp(u32), // Eat as soon as HP goes below threshold
}

#[derive(Debug, Clone)]
struct VardorvisState {
    vardorvis_attack_tick: i32,
}

impl Default for VardorvisState {
    fn default() -> Self {
        Self {
            vardorvis_attack_tick: 2,
        }
    }
}

struct VardorvisMechanics;

impl Mechanics for VardorvisMechanics {}

impl VardorvisMechanics {
    fn vardorvis_attack(
        &self,
        vard: &mut Monster,
        player: &mut Player,
        state: &mut VardorvisState,
        vars: &mut FightVars,
        rng: &mut SmallRng,
        logger: &mut FightLogger,
    ) {
        scale_monster_hp_only(vard);
        let mut hit = vard.attack(player, Some(VARDORVIS_ATTACK_STYLE), rng, false);
        hit.damage /= 4; // Assumes Protect from Melee is active

        logger.log_monster_attack(
            vard,
            vars.tick_counter,
            hit.damage,
            hit.success,
            Some(VARDORVIS_ATTACK_STYLE),
        );
        if hit.success {
            player.take_damage(hit.damage);
            vars.damage_taken += hit.damage;
            vard.heal(hit.damage / 2);
            handle_recoil(player, vard, &hit, vars, logger);
            scale_monster_hp_only(vard);

            logger.log_custom(
                vars.tick_counter,
                format!("Vardorvis healed {} HP", hit.damage / 2).as_str(),
            );
            logger.log_custom(
                vars.tick_counter,
                format!("Vardorvis HP: {}", vard.stats.hitpoints.current).as_str(),
            );
            logger.log_custom(
                vars.tick_counter,
                format!("Vardorvis defence level: {}", vard.stats.defence.current).as_str(),
            );
            logger.log_custom(
                vars.tick_counter,
                format!("Vardorvis strength level: {}", vard.stats.strength.current).as_str(),
            );

            logger.log_custom(
                vars.tick_counter,
                format!(
                    "Vardorvis max hit: {}",
                    vard.max_hits.as_ref().unwrap()[0].value
                )
                .as_str(),
            );
        }

        state.vardorvis_attack_tick += VARDORVIS_ATTACK_SPEED;
    }

    fn handle_eating(
        &self,
        config: &mut VardorvisConfig,
        vars: &mut FightVars,
        player: &mut Player,
    ) {
        // Handle eating based on set strategy
        match config.eat_strategy {
            VardorvisEatStrategy::EatAtHp(threshold) => {
                // Eat if at or below the provided threshold and force the player to skip the next attack
                if player.stats.hitpoints.current <= threshold && vars.eat_delay == 0 {
                    self.eat_food(
                        player,
                        config.food_heal_amount,
                        None,
                        vars,
                        &mut config.logger,
                    );
                    vars.attack_tick += config.food_eat_delay;
                }
            }
        }
    }
}

pub struct VardorvisFight {
    player: Player,
    vard: Monster,
    limiter: Option<Box<dyn Limiter>>,
    rng: SmallRng,
    config: VardorvisConfig,
    mechanics: VardorvisMechanics,
}

impl VardorvisFight {
    pub fn new(player: Player, config: VardorvisConfig) -> Self {
        let mut vard = Monster::new("Vardorvis", Some("Post-quest")).unwrap();
        vard.max_hits = Some(vec![MonsterMaxHit::new(0, AttackType::Slash)]);
        scale_monster_hp_only(&mut vard);

        let limiter = assign_limiter(&player, &vard);
        let rng = SmallRng::from_os_rng();
        Self {
            player,
            vard,
            limiter,
            rng,
            config,
            mechanics: VardorvisMechanics,
        }
    }

    fn simulate_vardorvis_fight(&mut self) -> Result<FightResult, SimulationError> {
        let mut vars = FightVars::new();
        let mut state = VardorvisState::default();
        scale_monster_hp_only(&mut self.vard);
        self.config
            .logger
            .log_initial_setup(&self.player, &self.vard);

        while self.vard.stats.hitpoints.current > 0 {
            // Assuming regen rate is 100 ticks for now; TODO: test this
            if vars.tick_counter % VARDORVIS_REGEN_TICKS == 0 {
                self.mechanics
                    .monster_regen_hp(&mut self.vard, &vars, &mut self.config.logger);
            }

            // Regen 1 HP for player every 100 ticks
            if vars.tick_counter % constants::PLAYER_REGEN_TICKS == 0 {
                self.mechanics
                    .player_regen(&mut self.player, &vars, &mut self.config.logger);
            }

            self.mechanics.decrement_eat_delay(&mut vars);
            self.mechanics
                .handle_eating(&mut self.config, &mut vars, &mut self.player);

            if vars.tick_counter == vars.attack_tick {
                self.mechanics.player_attack(
                    &mut self.player,
                    &mut self.vard,
                    &mut self.rng,
                    &self.limiter,
                    &mut vars,
                    &mut self.config.logger,
                );
            }

            self.mechanics
                .process_monster_effects(&mut self.vard, &vars, &mut self.config.logger);

            if vars.tick_counter == state.vardorvis_attack_tick {
                self.mechanics.vardorvis_attack(
                    &mut self.vard,
                    &mut self.player,
                    &mut state,
                    &mut vars,
                    &mut self.rng,
                    &mut self.config.logger,
                );
            }

            // Increment tick counter
            vars.tick_counter += 1;

            if self.player.stats.hitpoints.current == 0 {
                return self.mechanics.process_player_death(
                    &vars,
                    &self.vard,
                    &mut self.config.logger,
                );
            }
        }
        let remove_final_attack_delay = false;
        self.mechanics.get_fight_result(
            &self.player,
            &self.vard,
            &vars,
            &mut self.config.logger,
            remove_final_attack_delay,
        )
    }
}

impl Simulation for VardorvisFight {
    fn simulate(&mut self) -> Result<FightResult, SimulationError> {
        self.simulate_vardorvis_fight()
    }

    fn is_immune(&self) -> bool {
        self.vard.is_immune(&self.player)
    }

    fn player(&self) -> &Player {
        &self.player
    }

    fn monster(&self) -> &Monster {
        &self.vard
    }

    fn set_attack_function(&mut self) {
        self.player.attack = crate::combat::attacks::standard::get_attack_functions(&self.player);
    }

    fn reset(&mut self) {
        self.player.reset_current_stats(true);
        self.vard.reset();
    }
}
