use crate::monster::Monster;
use crate::player::Player;
use crate::sims::single_way;

pub fn simulate_n_fights(
    player: &mut Player,
    monster: &mut Monster,
    n: u32,
) -> (f32, f32, Vec<Vec<u32>>) {
    let mut ttks = Vec::new();
    let mut hit_counts = Vec::new();
    let mut hit_attempt_counts = Vec::new();
    let mut hit_amounts = Vec::new();

    for _ in 0..n {
        let (ttk, hits, hit_attempts, hit_amount) = single_way::simulate_fight(player, monster);
        ttks.push(ttk);
        hit_counts.push(hits);
        hit_attempt_counts.push(hit_attempts);
        hit_amounts.push(hit_amount);
        monster.reset();
        player.reset_live_stats();
    }

    let avg_ttk = ttks.iter().sum::<f32>() / n as f32;
    let avg_accuracy = hit_counts.iter().sum::<i32>() as f32
        / hit_attempt_counts.iter().sum::<i32>() as f32
        * 100.0;

    (avg_ttk, avg_accuracy, hit_amounts)
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
    fn test_simulate_n_fights() {
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
        let (ttk, accuracy, all_hits) = simulate_n_fights(&mut player, &mut monster, 100000);

        assert!(num::abs(ttk - 10.2) < 0.1);
        assert!(num::abs(accuracy - 99.04) < 0.1);
        assert_eq!(all_hits.len(), 100000);
    }
}
