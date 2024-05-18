use crate::{monster::Monster, player::Player, utils};

pub fn simulate_fight(player: &mut Player, monster: &mut Monster) -> (f32, i32, i32, Vec<u32>) {
    let mut rng = rand::thread_rng();

    let mut tick_counter = 0i32;
    let mut hit_attempts = 0;
    let mut hits = 0;
    let mut hit_amounts = Vec::new();
    let mut attack_tick = 0;
    let mut poison_tick = -1;
    let mut freeze_immunity = 0;
    let monster_freeze_resistance = monster.immunities.freeze;
    let player_attack = player.attack;

    while monster.live_stats.hitpoints > 0 && player.live_stats.hitpoints > 0 {
        if tick_counter == attack_tick {
            let (damage, success) = player_attack(player, monster, &mut rng);
            monster.take_damage(damage);
            hit_attempts += 1;
            hits += if success { 1 } else { 0 };
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

    let ttk = tick_counter as f32 * 0.6;

    (ttk, hits, hit_attempts, hit_amounts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::equipment::{Armor, CombatStyle, Weapon};
    use crate::monster::Monster;
    use crate::player::{Gear, Player, PlayerStats};
    use crate::potions::Potion;
    use crate::prayers::{Prayer, PrayerBoost};
    use crate::rolls::calc_player_melee_rolls;

    #[test]
    fn test_simulate_fight() {
        let mut player = Player::new();
        player.stats = PlayerStats {
            attack: 99,
            strength: 99,
            defence: 99,
            ranged: 99,
            magic: 99,
            hitpoints: 99,
            prayer: 99,
        };
        player.prayers.add(PrayerBoost::new(Prayer::Piety));
        player.add_potion(Potion::SuperCombat);

        player.gear = Gear {
            head: Some(Armor::new("Torva full helm")),
            neck: Some(Armor::new("Amulet of torture")),
            cape: Some(Armor::new("Infernal cape")),
            ammo: Some(Armor::new("Rada's blessing 4")),
            second_ammo: None,
            weapon: Weapon::new("Ghrazi rapier"),
            shield: Some(Armor::new("Avernic defender")),
            body: Some(Armor::new("Torva platebody")),
            legs: Some(Armor::new("Torva platelegs")),
            hands: Some(Armor::new("Ferocious gloves")),
            feet: Some(Armor::new("Primordial boots")),
            ring: Some(Armor::new("Ultor ring")),
        };
        player.update_bonuses();
        player.set_active_style(CombatStyle::Lunge);
        let mut monster = Monster::new("Ammonite Crab").unwrap();
        calc_player_melee_rolls(&mut player, &monster);
        let (ttk, hits, hit_attempts, all_hits) = simulate_fight(&mut player, &mut monster);

        assert!(ttk > 0.0);
        assert!(hit_attempts > 0);
        assert!(hits > 0);
        assert!(!all_hits.is_empty());
    }
}
