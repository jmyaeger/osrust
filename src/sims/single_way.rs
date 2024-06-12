use crate::combat::{FightResult, Simulation};
use crate::limiters::Limiter;
use crate::{monster::Monster, player::Player, utils};
use rand::rngs::ThreadRng;

pub struct SingleWayFight<'a> {
    pub player: Player,
    pub monster: Monster,
    pub limiter: &'a Option<Box<dyn Limiter>>,
    pub rng: ThreadRng,
}

impl Simulation for SingleWayFight<'_> {
    fn simulate(&mut self) -> FightResult {
        simulate_fight(
            &mut self.player,
            &mut self.monster,
            &mut self.rng,
            self.limiter,
        )
    }
}

pub fn simulate_fight(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> FightResult {
    let mut tick_counter = 0i32;
    let mut hit_attempts = 0;
    let mut hit_count = 0;
    let mut hit_amounts = Vec::new();
    let mut attack_tick = 0;
    let mut poison_tick = -1;
    let mut freeze_immunity = 0;
    let monster_freeze_resistance = monster.immunities.freeze;
    let player_attack = player.attack;

    while monster.live_stats.hitpoints > 0 {
        if tick_counter == attack_tick {
            let (damage, success) = player_attack(player, monster, rng, limiter);
            monster.take_damage(damage);
            hit_attempts += 1;
            hit_count += if success { 1 } else { 0 };
            hit_amounts.push(damage);
            attack_tick += player.gear.weapon.speed;
        }

        if monster.info.freeze_duration > 0 {
            monster.info.freeze_duration -= 1;
            if monster.info.freeze_duration == 0 {
                freeze_immunity = 5;
                monster.immunities.freeze = 100;
            }
        }

        if freeze_immunity > 0 {
            freeze_immunity -= 1;
            if freeze_immunity == 0 {
                monster.immunities.freeze = monster_freeze_resistance;
            }
        }

        if monster.info.poison_severity > 0 && poison_tick < 0 {
            poison_tick = tick_counter;
        }

        if poison_tick >= 0 && (tick_counter - poison_tick) % 30 == 0 && !monster.immunities.poison
        {
            monster.live_stats.hitpoints -= utils::poison_damage(monster.info.poison_severity);
            monster.info.poison_severity = monster.info.poison_severity.saturating_sub(1);
            if monster.info.poison_severity == 0 {
                poison_tick = -1;
            }
        }

        if monster.live_stats.hitpoints == 0 {
            tick_counter = attack_tick;
        } else {
            tick_counter += 1;
        }
    }

    let ttk = tick_counter as f64 * 0.6;

    FightResult {
        ttk,
        hit_attempts,
        hit_count,
        hit_amounts,
    }
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
        let result = simulate_fight(&mut player, &mut monster, &mut rng, &limiter);

        assert!(result.ttk > 0.0);
        assert!(result.hit_attempts > 0);
        assert!(result.hit_count > 0);
        assert!(!result.hit_amounts.is_empty());
    }
}
