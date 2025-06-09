use crate::combat::limiters::Limiter;
use crate::combat::mechanics::Mechanics;
use crate::combat::simulation::{FightResult, FightVars, Simulation, SimulationError};
use crate::constants;
use crate::types::monster::{AttackType, Monster};
use crate::types::player::Player;
use crate::utils::logging::FightLogger;
use rand::rngs::SmallRng;
use rand::SeedableRng;

const GRAARDOR_REGEN_TICKS: i32 = 10;
const CYCLE_LENGTH: i32 = 24;
const VALID_EAT_TICKS: &[i32; 8] = &[5, 6, 7, 8, 17, 18, 19, 20];

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum GraardorMethod {
    DoorAltar,
}

struct GraardorMechanics;

impl Mechanics for GraardorMechanics {}

impl GraardorMechanics {}

#[derive(Debug, PartialEq, Clone)]
pub struct GraardorConfig {
    pub method: GraardorMethod,
    pub eat_hp: u32,
    pub heal_amount: u32,
    pub logger: FightLogger,
}

impl Default for GraardorConfig {
    fn default() -> Self {
        Self {
            method: GraardorMethod::DoorAltar,
            eat_hp: 30,
            heal_amount: 20,
            logger: FightLogger::new(false, "graardor"),
        }
    }
}

#[derive(Debug, Clone)]
struct GraardorState {
    mage_attack_tick: i32,
    melee_attack_tick: i32,
    skip_next_attack: bool,
    cycle_tick: i32,
}

impl Default for GraardorState {
    fn default() -> Self {
        Self {
            mage_attack_tick: 1,
            melee_attack_tick: 5,
            skip_next_attack: false,
            cycle_tick: 0,
        }
    }
}

pub struct GraardorFight {
    player: Player,
    graardor: Monster,
    melee_minion: Monster,
    ranged_minion: Monster,
    mage_minion: Monster,
    limiter: Option<Box<dyn Limiter>>,
    rng: SmallRng,
    config: GraardorConfig,
    mechanics: GraardorMechanics,
}

impl GraardorFight {
    pub fn new(player: Player, config: GraardorConfig) -> GraardorFight {
        let graardor = Monster::new("General Graardor", None).unwrap();
        let melee_minion = Monster::new("Sergeant Strongstack", None).unwrap();
        let ranged_minion = Monster::new("Sergeant Grimspike", None).unwrap();
        let mage_minion = Monster::new("Sergeant Steelwill", None).unwrap();
        let limiter = crate::combat::simulation::assign_limiter(&player, &graardor);
        let rng = SmallRng::from_os_rng();
        GraardorFight {
            player,
            graardor,
            melee_minion,
            ranged_minion,
            mage_minion,
            limiter,
            rng,
            config,
            mechanics: GraardorMechanics,
        }
    }

    fn simulate_door_altar_fight(&mut self) -> Result<FightResult, SimulationError> {
        if self.player.gear.weapon.speed != 4 {
            let error_msg = format!(
                "GraardorFight::simulate_door_altar_fight: player weapon speed must be 4, got {}",
                self.player.gear.weapon.speed
            );
            return Err(SimulationError::ConfigError(error_msg));
        }

        let mut vars = FightVars::new();
        let mut state = GraardorState::default();

        self.config
            .logger
            .log_initial_setup(&self.player, &self.graardor);

        while self.graardor.stats.hitpoints.current > 0 {
            // Player attack
            if vars.tick_counter == vars.attack_tick {
                if state.skip_next_attack {
                    state.skip_next_attack = false;
                    vars.attack_tick += 4;
                } else {
                    self.mechanics.player_attack(
                        &mut self.player,
                        &mut self.graardor,
                        &mut self.rng,
                        &self.limiter,
                        &mut vars,
                        &mut self.config.logger,
                    );
                }
            }

            // Process active effects on Graardor
            self.mechanics.process_monster_effects(
                &mut self.graardor,
                &vars,
                &mut self.config.logger,
            );

            // Mage minion attack
            if vars.tick_counter == state.mage_attack_tick {
                self.mechanics.monster_attack(
                    &mut self.mage_minion,
                    &mut self.player,
                    Some(AttackType::Magic),
                    &mut vars,
                    &mut self.rng,
                    &mut self.config.logger,
                );
                if vars.tick_counter == 6 {
                    state.mage_attack_tick += 7;
                } else {
                    state.mage_attack_tick += 5;
                }
            }

            // Melee minion attack
            if vars.tick_counter == state.melee_attack_tick {
                self.mechanics.monster_attack(
                    &mut self.melee_minion,
                    &mut self.player,
                    Some(AttackType::Crush),
                    &mut vars,
                    &mut self.rng,
                    &mut self.config.logger,
                );
                if vars.tick_counter == 5 {
                    state.melee_attack_tick += 22;
                } else {
                    state.melee_attack_tick += 12;
                }
            }

            // Check for player death and return if dead
            if self.player.stats.hitpoints.current == 0 {
                return self.mechanics.process_player_death(
                    &vars,
                    &self.graardor,
                    &mut self.config.logger,
                );
            }

            // Decrement eat delay if there is one
            self.mechanics.decrement_eat_delay(&mut vars);

            // Eat if below the provided threshold and force the player to skip the next attack
            if self.player.stats.hitpoints.current < self.config.eat_hp
                && VALID_EAT_TICKS.contains(&state.cycle_tick)
                && vars.eat_delay == 0
            {
                self.mechanics.eat_food(
                    &mut self.player,
                    self.config.heal_amount,
                    None,
                    &mut vars,
                    &mut self.config.logger,
                );
                state.skip_next_attack = true;
            }

            // Regen all stats by 1 for Graardor every 10 ticks
            if vars.tick_counter % GRAARDOR_REGEN_TICKS == 0 {
                self.mechanics
                    .monster_regen_hp(&mut self.graardor, &vars, &mut self.config.logger);
                self.mechanics.monster_regen_stats(
                    &mut self.graardor,
                    &vars,
                    &mut self.config.logger,
                );
            }

            // Regen all stats by 1 for player every 100 ticks
            if vars.tick_counter % constants::PLAYER_REGEN_TICKS == 0 {
                self.mechanics
                    .player_regen(&mut self.player, &vars, &mut self.config.logger);
            }

            // Increment tick counter
            vars.tick_counter += 1;

            // Update tile position and reset if it's at the end of a cycle
            if state.cycle_tick == CYCLE_LENGTH - 1 {
                state.cycle_tick = 0;
            } else {
                state.cycle_tick += 1;
            }
        }

        let remove_final_attack_delay = true;
        self.mechanics.get_fight_result(
            &self.player,
            &self.graardor,
            &vars,
            &mut self.config.logger,
            remove_final_attack_delay,
        )
    }
}

impl Simulation for GraardorFight {
    fn simulate(&mut self) -> Result<FightResult, SimulationError> {
        match self.config.method {
            GraardorMethod::DoorAltar => self.simulate_door_altar_fight(),
        }
    }

    fn is_immune(&self) -> bool {
        self.graardor.is_immune(&self.player)
    }

    fn player(&self) -> &Player {
        &self.player
    }

    fn monster(&self) -> &Monster {
        &self.graardor
    }

    fn set_attack_function(&mut self) {
        self.player.attack = crate::combat::attacks::standard::get_attack_functions(&self.player);
    }

    fn reset(&mut self) {
        self.player.reset_current_stats();
        self.graardor.reset();
        self.melee_minion.reset();
        self.ranged_minion.reset();
        self.mage_minion.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calc::rolls::calc_player_ranged_rolls;
    use crate::types::equipment::CombatStyle;
    use crate::types::player::Player;
    use crate::types::potions::Potion;
    use crate::types::prayers::{Prayer, PrayerBoost};

    #[test]
    fn test_simulate_door_altar_fight() {
        let mut player = Player::default();
        player.prayers.add(PrayerBoost::new(Prayer::Rigour));
        player.add_potion(Potion::Ranging);

        player.equip("Bow of faerdhinen (c)", None);
        player.equip("Crystal helm", Some("Active"));
        player.equip("Crystal body", Some("Active"));
        player.equip("Crystal legs", Some("Active"));
        player.equip("Zaryte vambraces", None);
        player.equip("Dizana's quiver", Some("Uncharged"));
        player.equip("Necklace of anguish", None);
        player.equip("Pegasian boots", None);
        player.equip("Ring of suffering (i)", Some("Uncharged"));
        player.equip("Rada's blessing 4", None);
        player.update_bonuses();
        player.set_active_style(CombatStyle::Rapid);

        calc_player_ranged_rolls(
            &mut player,
            &Monster::new("General Graardor", None).unwrap(),
        );

        let fight_config = GraardorConfig {
            method: GraardorMethod::DoorAltar,
            eat_hp: 30,
            heal_amount: 22,
            logger: FightLogger::new(false, "graardor"),
        };

        let mut fight = GraardorFight::new(player, fight_config);

        let result = fight.simulate();

        if let Ok(result) = result {
            assert!(result.ttk_ticks > 0);
        }
    }
}
