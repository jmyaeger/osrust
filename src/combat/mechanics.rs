use crate::calc::monster_scaling::scale_monster_hp_only;
use crate::combat::limiters::Limiter;
use crate::combat::simulation::FightVars;
use crate::types::monster::Monster;
use crate::types::player::Player;
use crate::utils::logging::FightLogger;
use rand::rngs::ThreadRng;

pub trait Mechanics {
    fn player_attack(
        &self,
        player: &mut Player,
        monster: &mut Monster,
        rng: &mut ThreadRng,
        limiter: &Option<Box<dyn Limiter>>,
        fight_vars: &mut FightVars,
        logger: &mut FightLogger,
    ) {
        let hit = (player.attack)(player, monster, rng, limiter);
        logger.log_player_attack(
            fight_vars.tick_counter,
            hit.damage,
            hit.success,
            player.combat_type(),
        );
        player.boosts.first_attack = false;
        monster.take_damage(hit.damage);
        scale_monster_hp_only(monster);
        fight_vars.hit_attempts += 1;
        fight_vars.hit_count += if hit.success { 1 } else { 0 };
        fight_vars.hit_amounts.push(hit.damage);
        fight_vars.attack_tick += player.gear.weapon.speed;
    }

    fn process_monster_effects(&self, monster: &mut Monster) {
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
    }

    fn process_freeze(&self, monster: &mut Monster, fight_vars: &mut FightVars) {
        // Decrement freeze duration if it's active
        if monster.info.freeze_duration > 0 {
            monster.info.freeze_duration -= 1;
            if monster.info.freeze_duration == 0 {
                // 5 tick freeze immunity when it runs out
                fight_vars.freeze_immunity = 5;
                monster.immunities.freeze = 100;
            }
        }

        // Decrement temporary freeze immunity if applicable
        if fight_vars.freeze_immunity > 0 {
            fight_vars.freeze_immunity -= 1;
            if fight_vars.freeze_immunity == 0 {
                // Reset freeze resistance to original value when immunity runs out
                monster.immunities.freeze = fight_vars.freeze_resistance;
            }
        }
    }

    fn increment_tick(&self, monster: &mut Monster, fight_vars: &mut FightVars) {
        // Add the attack cooldown on the last hit (for continuous TTK)
        if monster.stats.hitpoints.current == 0 {
            fight_vars.tick_counter = fight_vars.attack_tick;
        } else {
            // Increment tick counter
            fight_vars.tick_counter += 1;
        }
    }
}
