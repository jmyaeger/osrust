use crate::calc::monster_scaling::scale_monster_hp_only;
use crate::combat::limiters::Limiter;
use crate::combat::mechanics::Mechanics;
use crate::combat::simulation::{FightResult, FightVars, Simulation, SimulationError};
use crate::types::{monster::Monster, player::Player};
use crate::utils::logging::FightLogger;
use rand::rngs::ThreadRng;

pub struct SingleWayFight {
    pub player: Player,
    pub monster: Monster,
    pub limiter: Option<Box<dyn Limiter>>,
    pub rng: ThreadRng,
    pub mechanics: SingleWayMechanics,
    pub logger: FightLogger,
}

impl SingleWayFight {
    pub fn new(player: Player, monster: Monster) -> SingleWayFight {
        let limiter = crate::combat::simulation::assign_limiter(&player, &monster);
        let rng = rand::thread_rng();
        let monster_name = monster.info.name.clone();
        SingleWayFight {
            player,
            monster,
            limiter,
            rng,
            mechanics: SingleWayMechanics,
            logger: FightLogger::new(false, monster_name.as_str()),
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
            &self.mechanics,
            &mut self.logger,
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
        self.player.reset_current_stats();
        self.monster.reset();
    }
}

#[derive(Debug)]
pub struct SingleWayMechanics;

impl Mechanics for SingleWayMechanics {}

fn simulate_fight(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
    mechanics: &SingleWayMechanics,
    logger: &mut FightLogger,
) -> Result<FightResult, SimulationError> {
    let mut vars = FightVars::new();
    scale_monster_hp_only(monster);

    while monster.stats.hitpoints.current > 0 {
        if vars.tick_counter == vars.attack_tick {
            mechanics.player_attack(player, monster, rng, limiter, &mut vars, logger);
        }
        mechanics.process_monster_effects(monster, &vars, logger);
        mechanics.process_freeze(monster, &mut vars, logger);
        mechanics.increment_tick(monster, &mut vars);
    }

    let remove_final_attack_delay = false;
    mechanics.get_fight_result(player, monster, &vars, logger, remove_final_attack_delay)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calc::rolls::calc_player_melee_rolls;
    use crate::combat::simulation::assign_limiter;
    use crate::types::equipment::{Armor, CombatStyle, Gear, Weapon};
    use crate::types::monster::Monster;
    use crate::types::player::Player;
    use crate::types::potions::Potion;
    use crate::types::prayers::{Prayer, PrayerBoost};
    use crate::types::stats::PlayerStats;
    use crate::utils::logging::FightLogger;

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
        let mechanics = SingleWayMechanics;
        let mut logger = FightLogger::new(false, monster.info.name.as_str());
        let result = simulate_fight(
            &mut player,
            &mut monster,
            &mut rng,
            &limiter,
            &mechanics,
            &mut logger,
        )
        .unwrap();

        assert!(result.ttk_ticks > 0);
        assert!(result.hit_attempts > 0);
        assert!(result.hit_count > 0);
        assert!(!result.hit_amounts.is_empty());
    }
}
