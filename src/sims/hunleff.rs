use crate::combat::limiters::Limiter;
use crate::combat::simulation::{FightResult, FightVars, Simulation, SimulationError};
use crate::constants;
use crate::types::monster::{AttackType, Monster, MonsterMaxHit};
use crate::types::player::{Player, SwitchType};
use crate::utils::logging::FightLogger;
use rand::rngs::ThreadRng;
use rand::Rng;

const TORNADO_MAX_TIMER: u32 = 23;
const TORNADO_COOLDOWN: u32 = 9;
const TORNADO_BASE_CHANCE: u32 = 6;
const PADDLEFISH_HEAL: u32 = 20;
const PADDLEFISH_DELAY: i32 = 3;
const HUNLLEF_REGEN_TICKS: i32 = 100;
const HUNLLEF_ATTACK_SPEED: i32 = 5;
const ALLOWED_GEAR: [&str; 22] = [
    "Crystal helm (basic)",
    "Crystal helm (attuned)",
    "Crystal helm (perfected)",
    "Crystal body (basic)",
    "Crystal body (attuned)",
    "Crystal body (perfected)",
    "Crystal legs (basic)",
    "Crystal legs (attuned)",
    "Crystal legs (perfected)",
    "Corrupted sceptre",
    "Corrupted axe",
    "Corrupted pickaxe",
    "Corrupted harpoon",
    "Corrupted staff (basic)",
    "Corrupted staff (attuned)",
    "Corrupted staff (perfected)",
    "Corrupted halberd (basic)",
    "Corrupted halberd (attuned)",
    "Corrupted halberd (perfected)",
    "Corrupted bow (basic)",
    "Corrupted bow (attuned)",
    "Corrupted bow (perfected)",
];

#[derive(Debug, PartialEq, Clone)]
pub struct HunllefConfig {
    pub food_count: u32, // Only normal paddlefish for now
    pub eat_strategy: EatStrategy,
    pub redemption_attempts: u32, // TODO: Attempt to use redemption a certain number of times at the beginning
    pub attack_strategy: AttackStrategy,
    pub lost_ticks: i32,
    pub logger: FightLogger,
}

impl Default for HunllefConfig {
    fn default() -> Self {
        Self {
            food_count: 20,
            eat_strategy: EatStrategy::EatAtHp(50),
            redemption_attempts: 0,
            attack_strategy: AttackStrategy::TwoT3Weapons {
                style1: SwitchType::Ranged,
                style2: SwitchType::Magic,
            },
            lost_ticks: 0,
            logger: FightLogger::new(false, "hunllef"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum EatStrategy {
    EatAtHp(u32),          // Eat as soon as HP goes below threshold
    TickEatOnly,           // Allow health to go below max hit and then tick eat
    EatToFullDuringNadoes, // Don't eat until tornadoes unless necessary, then eat to full
}

#[derive(Debug, PartialEq, Clone)]
pub enum AttackStrategy {
    TwoT3Weapons {
        style1: SwitchType,
        style2: SwitchType,
    },
    FiveToOne {
        style1: SwitchType,
        style2: SwitchType,
        style3: SwitchType,
    },
}

#[derive(Debug, Clone)]
struct HunllefState {
    hunllef_attack_tick: i32,
    tornado_chance: u32,
    tornado_cd: u32,
    tornado_timer: u32,
    player_attack_count: u32,
    hunllef_attack_count: u32,
    food_eaten: u32,
    damage_taken: u32,
    eat_delay: u32,
    queued_damage: Option<u32>,
    food_count: u32,
}

impl Default for HunllefState {
    fn default() -> Self {
        Self {
            hunllef_attack_tick: 2,
            tornado_chance: 6, // 1/6 initial probability of tornado spawn
            tornado_cd: 6,     // Tornadoes can't spawn until the 6th attack
            tornado_timer: 0,
            player_attack_count: 0,
            hunllef_attack_count: 0,
            food_eaten: 0,
            damage_taken: 0,
            eat_delay: 0,
            queued_damage: None,
            food_count: 20,
        }
    }
}

#[derive(Clone)]
pub struct HunllefFight {
    pub player: Player,
    pub hunllef: Monster,
    pub limiter: Option<Box<dyn Limiter>>,
    pub rng: ThreadRng,
    pub config: HunllefConfig,
}

impl HunllefFight {
    pub fn new(player: Player, config: HunllefConfig) -> HunllefFight {
        if !has_valid_gear(&player) {
            panic!("Equipped gear is not usable in the Gauntlet.")
        }
        let mut hunllef = Monster::new("Corrupted Hunllef", None).unwrap();
        let max_hit = match armor_tier(&player) {
            1 => 13,
            2 => 10,
            3 => 8,
            _ => 16,
        };
        hunllef.max_hits = Some(vec![
            MonsterMaxHit::new(max_hit, AttackType::Ranged),
            MonsterMaxHit::new(max_hit, AttackType::Magic),
        ]);

        let limiter = crate::combat::simulation::assign_limiter(&player, &hunllef);
        let rng = rand::thread_rng();
        HunllefFight {
            player,
            hunllef,
            limiter,
            rng,
            config,
        }
    }

    fn simulate_hunllef_fight(&mut self) -> Result<FightResult, SimulationError> {
        let mut vars = FightVars::new();
        let mut state = HunllefState::default();
        let player_attack = self.player.attack;
        state.food_count = self.config.food_count;
        vars.attack_tick += self.config.lost_ticks;

        self.config
            .logger
            .log_initial_setup(&self.player, &self.hunllef);

        match &self.config.attack_strategy {
            AttackStrategy::TwoT3Weapons { style1, style2 } => {
                // The normal case - two T3 weapons, no 5:1

                // Start off with a random style and store the other for later
                let style_choice = self.rng.gen_range(1..3);
                let mut current_style = if style_choice == 1 { *style1 } else { *style2 };
                let mut other_style = if style_choice == 1 { *style2 } else { *style1 };

                // Ensure the player is switched to the correct starting style
                self.player.switch(current_style);

                self.config
                    .logger
                    .log_gear_switch(vars.tick_counter, current_style);

                // Combat loop
                while self.hunllef.stats.hitpoints.current > 0 {
                    // Regen 1 HP for Hunllef every 100 ticks
                    if vars.tick_counter % HUNLLEF_REGEN_TICKS == 0 {
                        self.hunllef.heal(1);
                        self.config.logger.log_hp_regen(
                            vars.tick_counter,
                            self.hunllef.stats.hitpoints.current,
                            "Hunllef",
                        );
                    }

                    // Regen 1 HP for player every 100 ticks
                    if vars.tick_counter % constants::PLAYER_REGEN_TICKS == 0 {
                        self.player.heal(1, None);
                        self.config.logger.log_hp_regen(
                            vars.tick_counter,
                            self.player.stats.hitpoints.current,
                            "Player",
                        );
                    }

                    // Decrement the tornado timer if active
                    state.tornado_timer = state.tornado_timer.saturating_sub(1);

                    // Decrement eat delay timer if there is one active
                    if state.eat_delay > 0 {
                        state.eat_delay -= 1;
                    }

                    // Handle eating based on set strategy
                    match self.config.eat_strategy {
                        EatStrategy::EatAtHp(threshold) => {
                            // Eat if at or below the provided threshold and force the player to skip the next attack
                            if self.player.stats.hitpoints.current <= threshold
                                && state.eat_delay == 0
                                && state.food_count > 0
                            {
                                self.player.heal(PADDLEFISH_HEAL, None);
                                self.config.logger.log_food_eaten(
                                    vars.tick_counter,
                                    PADDLEFISH_HEAL,
                                    self.player.stats.hitpoints.current,
                                );
                                state.food_count -= 1;
                                state.food_eaten += 1;
                                state.eat_delay = constants::EAT_DELAY;
                                vars.attack_tick += PADDLEFISH_DELAY;
                            }
                        }
                        EatStrategy::TickEatOnly => {
                            if state.queued_damage.is_some()
                                && self.player.stats.hitpoints.current < 14
                                && state.eat_delay == 0
                                && state.food_count > 0
                            {
                                self.player.heal(PADDLEFISH_HEAL, None);
                                self.config.logger.log_food_eaten(
                                    vars.tick_counter,
                                    PADDLEFISH_HEAL,
                                    self.player.stats.hitpoints.current,
                                );
                                state.food_count -= 1;
                                state.food_eaten += 1;
                                state.eat_delay = constants::EAT_DELAY;
                                vars.attack_tick += PADDLEFISH_DELAY;
                            }
                        }
                        EatStrategy::EatToFullDuringNadoes => {
                            if ((state.tornado_timer > 0
                                && self.player.stats.hitpoints.base
                                    - self.player.stats.hitpoints.current
                                    >= PADDLEFISH_HEAL)
                                || self.player.stats.hitpoints.current < 14)
                                && state.eat_delay == 0
                                && state.food_count > 0
                            {
                                self.player.heal(PADDLEFISH_HEAL, None);
                                self.config.logger.log_food_eaten(
                                    vars.tick_counter,
                                    PADDLEFISH_HEAL,
                                    self.player.stats.hitpoints.current,
                                );
                                state.food_count -= 1;
                                state.food_eaten += 1;
                                state.eat_delay = constants::EAT_DELAY;
                                vars.attack_tick += PADDLEFISH_DELAY;
                            }
                        }
                    }

                    // Apply any queued damage to the player
                    if let Some(damage) = state.queued_damage {
                        self.player.take_damage(damage);
                        self.config.logger.log_player_damage(
                            vars.tick_counter,
                            damage,
                            self.player.stats.hitpoints.current,
                        );
                        state.damage_taken += damage;
                        state.queued_damage = None;
                    }

                    if vars.tick_counter == vars.attack_tick {
                        // Process player attack
                        let hit = player_attack(
                            &mut self.player,
                            &mut self.hunllef,
                            &mut self.rng,
                            &self.limiter,
                        );
                        self.config.logger.log_player_attack(
                            vars.tick_counter,
                            hit.damage,
                            hit.success,
                            self.player.combat_type(),
                        );
                        self.player.boosts.first_attack = false;
                        self.hunllef.take_damage(hit.damage);
                        self.config.logger.log_monster_damage(
                            vars.tick_counter,
                            hit.damage,
                            self.hunllef.stats.hitpoints.current,
                            "Hunllef",
                        );
                        vars.hit_attempts += 1;
                        vars.hit_count += if hit.success { 1 } else { 0 };
                        vars.hit_amounts.push(hit.damage);
                        vars.attack_tick += self.player.gear.weapon.speed;

                        // Increment attack count and switch styles every six attacks
                        state.player_attack_count += 1;
                        if state.player_attack_count == 6 {
                            state.player_attack_count = 0;
                            std::mem::swap(&mut current_style, &mut other_style);
                            self.player.switch(current_style);
                            self.config
                                .logger
                                .log_gear_switch(vars.tick_counter, current_style);
                        }
                    }

                    // No combat effects are possible here, so that section is omitted

                    // Process Hunllef's attack
                    if vars.tick_counter == state.hunllef_attack_tick {
                        // Roll for tornado spawn if off cooldown and not about to switch styles
                        state.tornado_cd = state.tornado_cd.saturating_sub(1);
                        let mut tornado_proc = false;
                        if state.tornado_cd == 0 {
                            if self.rng.gen_range(1..=state.tornado_chance) == 1
                                && state.hunllef_attack_count % 4 != 3
                            {
                                // Tornado procs act like an empty attack
                                tornado_proc = true;
                                self.config
                                    .logger
                                    .log_custom(vars.tick_counter, "Tornadoes spawned.");
                                state.hunllef_attack_tick += HUNLLEF_ATTACK_SPEED;
                                state.hunllef_attack_count += 1;

                                // Reset the tornado cooldown and probability
                                state.tornado_cd = TORNADO_COOLDOWN;
                                state.tornado_chance = TORNADO_BASE_CHANCE;
                                state.tornado_timer = TORNADO_MAX_TIMER;
                            } else {
                                // Decrease the denominator by 1 for each failed proc
                                state.tornado_chance =
                                    std::cmp::max(state.tornado_chance.saturating_sub(1), 1);
                            }
                        }
                        if !tornado_proc {
                            // Choose Hunllef's attack style, alternating every 4 attacks (starting with ranged)
                            let hunllef_style = if (state.hunllef_attack_count / 4) % 2 == 0 {
                                AttackType::Ranged
                            } else {
                                AttackType::Magic
                            };
                            let hit = self.hunllef.attack(
                                &mut self.player,
                                Some(hunllef_style),
                                &mut self.rng,
                            );
                            self.config.logger.log_monster_attack(
                                vars.tick_counter,
                                hit.damage,
                                hit.success,
                                hunllef_style,
                                "Hunllef",
                            );
                            // Queue the damage for the next tick to allow for tick eating
                            state.queued_damage = Some(hit.damage);
                            state.hunllef_attack_tick += HUNLLEF_ATTACK_SPEED;
                            state.hunllef_attack_count += 1;
                        }
                    }

                    // Increment tick counter
                    vars.tick_counter += 1;

                    if self.player.stats.hitpoints.current == 0 {
                        self.config.logger.log_player_death(vars.tick_counter);
                        return Err(SimulationError::PlayerDeathError(FightResult {
                            ttk: 0.0,
                            hit_attempts: vars.hit_attempts,
                            hit_count: vars.hit_count,
                            hit_amounts: vars.hit_amounts,
                            food_eaten: state.food_eaten,
                            damage_taken: state.damage_taken,
                            leftover_burn: 0,
                        }));
                    }
                }
            }
            AttackStrategy::FiveToOne {
                style1: _,
                style2: _,
                style3: _,
            } => {
                unimplemented!()
            }
        }

        self.config
            .logger
            .log_monster_death(vars.tick_counter, "Hunllef");

        let ttk = vars.tick_counter as f64 * 0.6;
        let leftover_burn = 0; // Burn isn't possible in CG

        Ok(FightResult {
            ttk,
            hit_attempts: vars.hit_attempts,
            hit_count: vars.hit_count,
            hit_amounts: vars.hit_amounts,
            food_eaten: state.food_eaten,
            damage_taken: state.damage_taken,
            leftover_burn,
        })
    }
}

impl Simulation for HunllefFight {
    fn simulate(&mut self) -> Result<FightResult, SimulationError> {
        self.simulate_hunllef_fight()
    }

    fn is_immune(&self) -> bool {
        self.hunllef.is_immune(&self.player)
    }

    fn player(&self) -> &Player {
        &self.player
    }

    fn monster(&self) -> &Monster {
        &self.hunllef
    }

    fn set_attack_function(&mut self) {
        self.player.attack = crate::combat::attacks::standard::get_attack_functions(&self.player);
    }

    fn reset(&mut self) {
        self.player.reset_current_stats();
        self.hunllef.reset();
    }
}

fn armor_tier(player: &Player) -> i32 {
    if player.gear.head.is_none() || player.gear.body.is_none() || player.gear.legs.is_none() {
        return 0;
    }

    let head = &player.gear.head.as_ref().unwrap().name;
    let body = &player.gear.body.as_ref().unwrap().name;
    let legs = &player.gear.legs.as_ref().unwrap().name;
    let all_armour: [&String; 3] = [head, body, legs];
    if all_armour.iter().any(|s| s.contains("basic")) {
        1
    } else if all_armour.iter().any(|s| s.contains("attuned")) {
        2
    } else {
        3
    }
}

fn has_valid_gear(player: &Player) -> bool {
    if player.gear.ammo.is_some()
        || player.gear.cape.is_some()
        || player.gear.feet.is_some()
        || player.gear.hands.is_some()
        || player.gear.neck.is_some()
        || player.gear.ring.is_some()
        || player.gear.second_ammo.is_some()
        || player.gear.shield.is_some()
        || !ALLOWED_GEAR.contains(&player.gear.weapon.name.as_str())
    {
        false
    } else {
        let slots = vec![&player.gear.head, &player.gear.body, &player.gear.legs];
        for slot in slots.into_iter().flatten() {
            if !ALLOWED_GEAR.contains(&slot.name.as_str()) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calc::rolls::calc_active_player_rolls;
    use crate::types::equipment::CombatStyle;
    use crate::types::monster::Monster;
    use crate::types::player::{GearSwitch, Player, SwitchType};
    use crate::types::prayers::{Prayer, PrayerBoost};
    use crate::types::stats::Stat;
    #[test]
    fn test_hunllef_sim() {
        let mut player = Player::new();
        player.stats.defence = Stat::new(70);
        player.stats.ranged = Stat::new(70);
        player.stats.magic = Stat::new(70);
        player.reset_current_stats();
        player.equip("Corrupted staff (perfected)", None);
        player.equip("Crystal helm (basic)", None);
        player.equip("Crystal body (basic)", None);
        player.equip("Crystal legs (basic)", None);
        player.update_bonuses();
        player.set_active_style(CombatStyle::Accurate);
        player.prayers.add(PrayerBoost::new(Prayer::MysticMight));
        player.prayers.add(PrayerBoost::new(Prayer::SteelSkin));

        let hunllef = Monster::new("Corrupted Hunllef", None).unwrap();
        calc_active_player_rolls(&mut player, &hunllef);

        let mage_switch = GearSwitch::from(&player);

        player.equip("Corrupted bow (perfected)", None);
        player.update_bonuses();
        player.set_active_style(CombatStyle::Rapid);
        player.prayers.add(PrayerBoost::new(Prayer::EagleEye));
        player.prayers.remove(PrayerBoost::new(Prayer::MysticMight));

        calc_active_player_rolls(&mut player, &hunllef);

        let ranged_switch = GearSwitch::from(&player);
        player.switches.push(mage_switch);
        player.switches.push(ranged_switch);

        let fight_config = HunllefConfig {
            food_count: 20,
            eat_strategy: EatStrategy::EatAtHp(15),
            redemption_attempts: 0,
            attack_strategy: AttackStrategy::TwoT3Weapons {
                style1: SwitchType::Magic,
                style2: SwitchType::Ranged,
            },
            lost_ticks: 0,
            logger: FightLogger::new(false, "hunllef"),
        };

        let mut fight = HunllefFight::new(player, fight_config);

        let result = fight.simulate();

        fight.reset();

        if let Ok(result) = result {
            assert!(result.ttk > 0.0);
        }
    }
}
