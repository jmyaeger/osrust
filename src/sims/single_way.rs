use crate::calc::monster_scaling::scale_monster_hp_only;
use crate::combat::attacks::effects::CombatEffect;
use crate::combat::limiters::Limiter;
use crate::combat::simulation::{FightResult, FightVars, Simulation, SimulationError};
use crate::types::{monster::Monster, player::Player};
use rand::rngs::ThreadRng;

pub struct SingleWayFight {
    pub player: Player,
    pub monster: Monster,
    pub limiter: Option<Box<dyn Limiter>>,
    pub rng: ThreadRng,
}

impl SingleWayFight {
    pub fn new(player: Player, monster: Monster) -> SingleWayFight {
        let limiter = crate::combat::simulation::assign_limiter(&player, &monster);
        let rng = rand::thread_rng();
        SingleWayFight {
            player,
            monster,
            limiter,
            rng,
        }
    }
}

impl Simulation for SingleWayFight {
    fn simulate(&mut self) -> Result<FightResult, SimulationError> {
        simulate_fight(
            &mut self.player,
            &mut self.monster,
            &mut self.rng,
            &self.limiter,
        )
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
        self.player.attack = crate::combat::attacks::standard::get_attack_functions(&self.player);
    }

    fn reset(&mut self) {
        self.player.reset_live_stats();
        self.monster.reset();
    }
}

fn simulate_fight(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Result<FightResult, SimulationError> {
    let mut vars = FightVars::new();
    let player_attack = player.attack;
    scale_monster_hp_only(monster);

    while monster.live_stats.hitpoints > 0 {
        if vars.tick_counter == vars.attack_tick {
            // Process player attack
            let hit = player_attack(player, monster, rng, limiter);
            player.boosts.first_attack = false;
            monster.take_damage(hit.damage);
            scale_monster_hp_only(monster);
            vars.hit_attempts += 1;
            vars.hit_count += if hit.success { 1 } else { 0 };
            vars.hit_amounts.push(hit.damage);
            vars.attack_tick += player.gear.weapon.speed;
        }

        // Process effects and apply damage
        let mut effect_damage = 0;
        for effect in &mut monster.active_effects {
            effect_damage += effect.apply();
        }

        if effect_damage > 0 {
            monster.take_damage(effect_damage);
            scale_monster_hp_only(monster);
        }

        monster.clear_inactive_effects();

        // Decrement freeze duration if it's active
        if monster.info.freeze_duration > 0 {
            monster.info.freeze_duration -= 1;
            if monster.info.freeze_duration == 0 {
                // 5 tick freeze immunity when it runs out
                vars.freeze_immunity = 5;
                monster.immunities.freeze = 100;
            }
        }

        // Decrement temporary freeze immunity if applicable
        if vars.freeze_immunity > 0 {
            vars.freeze_immunity -= 1;
            if vars.freeze_immunity == 0 {
                // Reset freeze resistance to original value when immunity runs out
                monster.immunities.freeze = vars.freeze_resistance;
            }
        }

        // Add the attack cooldown on the last hit (for continuous TTK)
        if monster.live_stats.hitpoints == 0 {
            vars.tick_counter = vars.attack_tick;
        } else {
            // Increment tick counter
            vars.tick_counter += 1;
        }
    }

    // Convert ttk to seconds
    let ttk = vars.tick_counter as f64 * 0.6;

    let leftover_burn = {
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
    };

    // Player can never die here, so the result is always Ok(FightResult)
    Ok(FightResult {
        ttk,
        hit_attempts: vars.hit_attempts,
        hit_count: vars.hit_count,
        hit_amounts: vars.hit_amounts,
        food_eaten: 0,
        damage_taken: 0,
        leftover_burn,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calc::rolls::calc_player_melee_rolls;
    use crate::combat::simulation::assign_limiter;
    use crate::types::equipment::{Armor, CombatStyle, Gear, Weapon};
    use crate::types::monster::Monster;
    use crate::types::player::{Player, PlayerStats};
    use crate::types::potions::Potion;
    use crate::types::prayers::{Prayer, PrayerBoost};

    #[test]
    fn test_simulate_fight() {
        let mut player = Player::new();
        player.stats = PlayerStats::default();
        player.prayers.add(PrayerBoost::new(Prayer::Piety));
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
        let mut monster = Monster::new("Ammonite Crab", None).unwrap();
        calc_player_melee_rolls(&mut player, &monster);

        let mut rng = rand::thread_rng();
        let limiter = assign_limiter(&player, &monster);
        let result = simulate_fight(&mut player, &mut monster, &mut rng, &limiter).unwrap();

        assert!(result.ttk > 0.0);
        assert!(result.hit_attempts > 0);
        assert!(result.hit_count > 0);
        assert!(!result.hit_amounts.is_empty());
    }
}
