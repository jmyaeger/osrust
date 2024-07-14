use crate::combat::{FightResult, FightVars, PlayerDeathError, Simulation};
use crate::limiters::Limiter;
use crate::{monster::Monster, player::Player};
use rand::rngs::ThreadRng;

pub struct SingleWayFight {
    pub player: Player,
    pub monster: Monster,
    pub limiter: Option<Box<dyn Limiter>>,
    pub rng: ThreadRng,
}

impl SingleWayFight {
    pub fn new(player: Player, monster: Monster) -> SingleWayFight {
        let limiter = crate::combat::assign_limiter(&player, &monster);
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
    fn simulate(&mut self) -> Result<FightResult, PlayerDeathError> {
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
        self.player.attack = crate::attacks::get_attack_functions(&self.player);
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
) -> Result<FightResult, PlayerDeathError> {
    let mut vars = FightVars::new();
    let player_attack = player.attack;

    while monster.live_stats.hitpoints > 0 {
        if vars.tick_counter == vars.attack_tick {
            // Process player attack
            let hit = player_attack(player, monster, rng, limiter);
            player.boosts.first_attack = false;
            monster.take_damage(hit.damage);
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

        monster.take_damage(effect_damage);
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

    // Player can never die here, so the result is always Ok(FightResult)
    Ok(FightResult {
        ttk,
        hit_attempts: vars.hit_attempts,
        hit_count: vars.hit_count,
        hit_amounts: vars.hit_amounts,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::assign_limiter;
    use crate::equipment::{Armor, CombatStyle, Weapon};
    use crate::monster::Monster;
    use crate::player::{Gear, Player, PlayerStats};
    use crate::potions::Potion;
    use crate::prayers::{Prayer, PrayerBoost};
    use crate::rolls::calc_player_melee_rolls;

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
