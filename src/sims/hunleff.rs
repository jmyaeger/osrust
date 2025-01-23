use crate::combat::{FightResult, FightVars, Simulation, SimulationError};
use crate::constants;
use crate::equipment::CombatType;
use crate::limiters::Limiter;
use crate::monster::{AttackType, Monster, MonsterMaxHit};
use crate::player::{Player, SwitchType};
use rand::rngs::ThreadRng;
use rand::Rng;

const PADDLEFISH_HEAL: u32 = 20;
const HUNLLEF_REGEN_TICKS: i32 = 100;
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
    pub redemption_attempts: u32, // Attempt to use redemption a certain number of times at the beginning
    pub attack_strategy: AttackStrategy,
    pub lost_ticks: i32,
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

        let limiter = crate::combat::assign_limiter(&player, &hunllef);
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
        let mut hunllef_attack_tick = 2;
        let mut tornado_chance = 6; // 1/6 initial probability of tornado spawn
        let mut tornado_cd: u32 = 5; // Tornadoes can't spawn in the first 5 attacks
        let mut tornado_timer: u32 = 0;
        let player_attack = self.player.attack;
        let mut hunllef_attack_count = 0;
        let mut player_attack_count = 0;
        let mut food_eaten = 0;
        let mut damage_taken = 0;
        let mut eat_delay = 0;
        let mut queued_damage: Option<u32> = None;
        let mut food_count = self.config.food_count;

        vars.attack_tick += self.config.lost_ticks;

        match &self.config.attack_strategy {
            AttackStrategy::TwoT3Weapons { style1, style2 } => {
                // The normal case - two T3 weapons, no 5:1

                // Start off with the first style and store the other for later
                let mut current_style = *style1;
                let mut other_style = *style2;

                // Ensure the player is switched to the correct starting style
                self.player.switch(current_style);

                // Combat loop
                while self.hunllef.live_stats.hitpoints > 0 {
                    // Decrement the tornado timer if active and the tornado cooldown
                    tornado_timer = tornado_timer.saturating_sub(1);
                    tornado_cd = tornado_cd.saturating_sub(1);

                    // Decrement eat delay timer if there is one active
                    if eat_delay > 0 {
                        eat_delay -= 1;
                    }

                    // Handle eating based on set strategy
                    match self.config.eat_strategy {
                        EatStrategy::EatAtHp(threshold) => {
                            // Eat if at or below the provided threshold and force the player to skip the next attack
                            if self.player.live_stats.hitpoints <= threshold
                                && eat_delay == 0
                                && food_count > 0
                            {
                                self.player.heal(PADDLEFISH_HEAL, None);
                                food_count -= 1;
                                food_eaten += 1;
                                eat_delay = 3;
                                vars.attack_tick += 3;
                            }
                        }
                        EatStrategy::TickEatOnly => {
                            if queued_damage.is_some()
                                && self.player.live_stats.hitpoints < 14
                                && eat_delay == 0
                                && food_count > 0
                            {
                                self.player.heal(PADDLEFISH_HEAL, None);
                                food_count -= 1;
                                food_eaten += 1;
                                eat_delay = 3;
                                vars.attack_tick += 3;
                            }
                        }
                        EatStrategy::EatToFullDuringNadoes => {
                            if tornado_timer > 0
                                && self.player.stats.hitpoints - self.player.live_stats.hitpoints
                                    >= PADDLEFISH_HEAL
                                && eat_delay == 0
                                && food_count > 0
                            {
                                self.player.heal(PADDLEFISH_HEAL, None);
                                food_count -= 1;
                                food_eaten += 1;
                                eat_delay = 3;
                                vars.attack_tick += 3;
                            }
                        }
                    }

                    // Apply any queued damage to the player
                    if let Some(damage) = queued_damage {
                        self.player.take_damage(damage);
                        damage_taken += damage;
                        queued_damage = None;
                    }

                    if vars.tick_counter == vars.attack_tick {
                        // Process player attack
                        let hit = player_attack(
                            &mut self.player,
                            &mut self.hunllef,
                            &mut self.rng,
                            &self.limiter,
                        );
                        self.player.boosts.first_attack = false;
                        self.hunllef.take_damage(hit.damage);
                        vars.hit_attempts += 1;
                        vars.hit_count += if hit.success { 1 } else { 0 };
                        vars.hit_amounts.push(hit.damage);
                        vars.attack_tick += self.player.gear.weapon.speed;

                        // Increment attack count and switch styles every six attacks
                        player_attack_count += 1;
                        if player_attack_count == 6 {
                            player_attack_count = 0;
                            std::mem::swap(&mut current_style, &mut other_style);
                            self.player.switch(current_style);
                        }
                    }

                    // No combat effects are possible here, so that section is omitted

                    // Process Hunllef's attack
                    if vars.tick_counter == hunllef_attack_tick {
                        // Roll for tornado spawn if off cooldown and not about to switch styles
                        let mut tornado_proc = false;
                        if tornado_cd == 0 && hunllef_attack_count % 4 != 3 {
                            if self.rng.gen_range(1..=tornado_chance) == 1 {
                                // Tornado procs act like an empty attack
                                tornado_proc = true;
                                hunllef_attack_tick += 5;
                                hunllef_attack_count += 1;

                                // Reset the tornado cooldown and probability
                                tornado_cd = 8;
                                tornado_chance = 6;
                                tornado_timer = 23;
                            } else {
                                // Decrease the denominator by 1 for each failed proc
                                tornado_chance -= 1;
                            }
                        }
                        if !tornado_proc {
                            // Choose Hunllef's attack style, alternating every 4 attacks (starting with ranged)
                            let hunllef_style = if (hunllef_attack_count / 4) % 2 == 0 {
                                Some(AttackType::Ranged)
                            } else {
                                Some(AttackType::Magic)
                            };
                            let hit =
                                self.hunllef
                                    .attack(&mut self.player, hunllef_style, &mut self.rng);
                            // Queue the damage for the next tick to allow for tick eating
                            queued_damage = Some(hit.damage);
                            hunllef_attack_tick += 5;
                            hunllef_attack_count += 1;
                        }
                    }

                    // Regen 1 HP for Hunllef every 100 ticks
                    if vars.tick_counter % HUNLLEF_REGEN_TICKS == 0 {
                        self.hunllef.heal(1);
                    }

                    // Regen 1 HP for player every 100 ticks
                    if vars.tick_counter % constants::PLAYER_REGEN_TICKS == 0 {
                        self.player.heal(1, None);
                    }

                    // Increment tick counter
                    vars.tick_counter += 1;

                    if self.player.live_stats.hitpoints == 0 {
                        return Err(SimulationError::PlayerDeathError);
                    }
                }
            }
            AttackStrategy::FiveToOne {
                style1,
                style2,
                style3,
            } => {
                unimplemented!()
            }
        }

        let ttk = vars.tick_counter as f64 * 0.6;
        let leftover_burn = 0; // Burn isn't possible in CG

        Ok(FightResult {
            ttk,
            hit_attempts: vars.hit_attempts,
            hit_count: vars.hit_count,
            hit_amounts: vars.hit_amounts,
            food_eaten,
            damage_taken,
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
        self.player.attack = crate::attacks::get_attack_functions(&self.player);
    }

    fn reset(&mut self) {
        self.player.reset_live_stats();
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
    use crate::equipment::CombatStyle;
    use crate::monster::Monster;
    use crate::player::{GearSwitch, Player, SwitchType};
    use crate::prayers::{Prayer, PrayerBoost};
    use crate::rolls::calc_active_player_rolls;
    #[test]
    fn test_hunllef_sim() {
        let mut player = Player::new();
        player.stats.defence = 70;
        player.stats.ranged = 70;
        player.stats.magic = 70;
        player.reset_live_stats();
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

        let mage_switch = GearSwitch::from_player(&player);

        player.equip("Corrupted bow (perfected)", None);
        player.update_bonuses();
        player.set_active_style(CombatStyle::Rapid);
        player.prayers.add(PrayerBoost::new(Prayer::Rigour));

        calc_active_player_rolls(&mut player, &hunllef);

        let ranged_switch = GearSwitch::from_player(&player);
        player.switches.push(mage_switch);
        player.switches.push(ranged_switch);

        let fight_config = HunllefConfig {
            food_count: 1,
            eat_strategy: EatStrategy::TickEatOnly,
            redemption_attempts: 0,
            attack_strategy: AttackStrategy::TwoT3Weapons {
                style1: SwitchType::Magic,
                style2: SwitchType::Ranged,
            },
            lost_ticks: 0,
        };

        let mut fight = HunllefFight::new(player, fight_config);

        let result = fight.simulate();

        fight.reset();

        if let Ok(result) = result {
            assert!(result.ttk > 0.0);
        }
    }
}
