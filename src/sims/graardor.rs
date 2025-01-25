use crate::combat::{FightResult, FightVars, Simulation, SimulationError};
use crate::constants;
use crate::effects::CombatEffect;
use crate::limiters::Limiter;
use crate::monster::{AttackType, Monster};
use crate::player::Player;
use rand::rngs::ThreadRng;

const GRAARDOR_REGEN_TICKS: i32 = 10;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum GraardorMethod {
    DoorAltar,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GraardorConfig {
    pub method: GraardorMethod,
    pub eat_hp: u32,
    pub heal_amount: u32,
}

#[derive(Clone)]
pub struct GraardorFight {
    pub player: Player,
    pub graardor: Monster,
    pub melee_minion: Monster,
    pub ranged_minion: Monster,
    pub mage_minion: Monster,
    pub limiter: Option<Box<dyn Limiter>>,
    pub rng: ThreadRng,
    pub config: GraardorConfig,
}

impl GraardorFight {
    pub fn new(player: Player, config: GraardorConfig) -> GraardorFight {
        let graardor = Monster::new("General Graardor", None).unwrap();
        let melee_minion = Monster::new("Sergeant Strongstack", None).unwrap();
        let ranged_minion = Monster::new("Sergeant Grimspike", None).unwrap();
        let mage_minion = Monster::new("Sergeant Steelwill", None).unwrap();
        let limiter = crate::combat::assign_limiter(&player, &graardor);
        let rng = rand::thread_rng();
        GraardorFight {
            player,
            graardor,
            melee_minion,
            ranged_minion,
            mage_minion,
            limiter,
            rng,
            config,
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
        let mut mage_attack_tick = 1;
        let mut melee_attack_tick = 5;
        let player_attack = self.player.attack;
        let mut skip_next_attack = false;
        let mut cycle_tick = 0;
        let mut food_eaten = 0;
        let mut damage_taken = 0;
        let mut eat_delay = 0;

        while self.graardor.live_stats.hitpoints > 0 {
            if vars.tick_counter == vars.attack_tick {
                if skip_next_attack {
                    skip_next_attack = false;
                    vars.attack_tick += 4;
                } else {
                    // Process player attack
                    let hit = player_attack(
                        &mut self.player,
                        &mut self.graardor,
                        &mut self.rng,
                        &self.limiter,
                    );
                    self.player.boosts.first_attack = false;
                    self.graardor.take_damage(hit.damage);
                    vars.hit_attempts += 1;
                    vars.hit_count += if hit.success { 1 } else { 0 };
                    vars.hit_amounts.push(hit.damage);
                    vars.attack_tick += self.player.gear.weapon.speed;
                }
            }

            // Process effects and apply damage
            let mut effect_damage = 0;
            for effect in &mut self.graardor.active_effects {
                effect_damage += effect.apply();
            }

            self.graardor.take_damage(effect_damage);
            self.graardor.clear_inactive_effects();

            // Process mage minion attack
            if vars.tick_counter == mage_attack_tick {
                let hit = self.mage_minion.attack(
                    &mut self.player,
                    Some(AttackType::Magic),
                    &mut self.rng,
                );
                self.player.take_damage(hit.damage);
                damage_taken += hit.damage;
                if vars.tick_counter == 6 {
                    mage_attack_tick += 7;
                } else {
                    mage_attack_tick += 5;
                }
            }

            // Process melee minion attack
            if vars.tick_counter == melee_attack_tick {
                let hit = self.melee_minion.attack(
                    &mut self.player,
                    Some(AttackType::Crush),
                    &mut self.rng,
                );
                self.player.take_damage(hit.damage);
                damage_taken += hit.damage;
                if vars.tick_counter == 5 {
                    melee_attack_tick += 22;
                } else {
                    melee_attack_tick += 12;
                }
            }

            if self.player.live_stats.hitpoints == 0 {
                let leftover_burn = {
                    if let Some(CombatEffect::Burn {
                        tick_counter: _,
                        stacks,
                    }) = self
                        .graardor
                        .active_effects
                        .iter()
                        .find(|item| matches!(item, &CombatEffect::Burn { .. }))
                    {
                        stacks.iter().sum()
                    } else {
                        0
                    }
                };
                return Err(SimulationError::PlayerDeathError(FightResult {
                    ttk: 0.0,
                    hit_attempts: vars.hit_attempts,
                    hit_count: vars.hit_count,
                    hit_amounts: vars.hit_amounts,
                    food_eaten,
                    damage_taken,
                    leftover_burn,
                }));
            }

            // Decrement eat delay timer if there is one active
            if eat_delay > 0 {
                eat_delay -= 1;
            }

            // Eat if below the provided threshold and force the player to skip the next attack
            if self.player.live_stats.hitpoints < self.config.eat_hp
                && ((5..=8).contains(&cycle_tick) || (17..=20).contains(&cycle_tick))
                && eat_delay == 0
            {
                self.player.heal(self.config.heal_amount, None);
                food_eaten += 1;
                eat_delay = 3;
                skip_next_attack = true;
            }

            // Regen 1 HP for Graardor every 10 ticks
            if vars.tick_counter % GRAARDOR_REGEN_TICKS == 0 {
                self.graardor.heal(1);
            }

            // Regen 1 HP for player every 100 ticks
            if vars.tick_counter % constants::PLAYER_REGEN_TICKS == 0 {
                self.player.heal(1, None);
            }

            // Increment tick counter
            vars.tick_counter += 1;

            // Update tile position and reset if it's at the end of a cycle
            if cycle_tick == 23 {
                cycle_tick = 0;
            } else {
                cycle_tick += 1;
            }
        }

        let ttk = vars.tick_counter as f64 * 0.6;

        let leftover_burn = {
            if let Some(CombatEffect::Burn {
                tick_counter: _,
                stacks,
            }) = self
                .graardor
                .active_effects
                .iter()
                .find(|item| matches!(item, &CombatEffect::Burn { .. }))
            {
                stacks.iter().sum()
            } else {
                0
            }
        };

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
        self.player.attack = crate::attacks::get_attack_functions(&self.player);
    }

    fn reset(&mut self) {
        self.player.reset_live_stats();
        self.graardor.reset();
        self.melee_minion.reset();
        self.ranged_minion.reset();
        self.mage_minion.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::equipment::CombatStyle;
    use crate::player::Player;
    use crate::potions::Potion;
    use crate::prayers::{Prayer, PrayerBoost};
    use crate::rolls::calc_player_ranged_rolls;

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
        };

        let mut fight = GraardorFight::new(player, fight_config);

        let result = fight.simulate();

        if let Ok(result) = result {
            assert!(result.ttk > 0.0);
        }
    }
}
