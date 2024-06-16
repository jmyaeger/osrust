use std::collections::HashMap;

use crate::attacks::get_attack_functions;
use crate::equipment::CombatType;
use crate::limiters;
use crate::monster::Monster;
use crate::player::Player;
use crate::sims::single_way;
use crate::spells::{Spell, StandardSpell};

#[derive(Debug, PartialEq, Clone)]
pub struct FightResult {
    pub ttk: f64,
    pub hit_attempts: u32,
    pub hit_count: u32,
    pub hit_amounts: Vec<u32>,
}

pub trait Simulation {
    fn simulate(&mut self) -> FightResult;
}

pub fn assign_limiter(player: &Player, monster: &Monster) -> Option<Box<dyn limiters::Limiter>> {
    // Dispatch post-roll transform based on monster name
    if monster.info.name.contains("Zulrah") {
        return Some(Box::new(limiters::Zulrah {}));
    }

    if monster.info.name.contains("Fragment of Seren") {
        return Some(Box::new(limiters::Seren {}));
    }

    if monster.info.name.as_str() == "Kraken (Kraken)" && player.is_using_ranged() {
        return Some(Box::new(limiters::Kraken {}));
    }

    if monster.info.name.contains("Verzik")
        && monster.matches_version("Phase 1")
        && !player.is_wearing("Dawnbringer", None)
    {
        let limit = if player.is_using_melee() { 10 } else { 3 };
        return Some(Box::new(limiters::VerzikP1 { limit }));
    }

    if monster.info.name.contains("Tekton") && player.combat_type() == CombatType::Magic {
        return Some(Box::new(limiters::Tekton {}));
    }

    if (monster.info.name.contains("Glowing crystal") && player.combat_type() == CombatType::Magic)
        || (monster.matches_version("Left claw")
            || (monster.info.name.contains("Great Olm") && monster.matches_version("Head"))
                && player.combat_type() == CombatType::Magic)
        || (monster.matches_version("Right claw")
            || monster.matches_version("Left claw") && player.is_using_ranged())
        || (monster.info.name.contains("Ice demon")
            && !player.is_using_fire_spell()
            && player.attrs.spell != Some(Spell::Standard(StandardSpell::FlamesOfZamorak)))
        || (monster.info.name.contains("Slagilith") && !player.gear.weapon.name.contains("pickaxe"))
    {
        return Some(Box::new(limiters::OneThirdDamage {}));
    }

    if ["Slash Bash", "Zogre", "Skogre"].contains(&monster.info.name.as_str()) {
        if player.attrs.spell == Some(Spell::Standard(StandardSpell::CrumbleUndead)) {
            return Some(Box::new(limiters::ZogreCrumbleUndead {}));
        } else if !player.is_using_ranged()
            || !player
                .gear
                .ammo
                .as_ref()
                .map_or(false, |ammo| ammo.name.contains(" brutal"))
            || !player.gear.weapon.name.contains("Comp ogre bow")
        {
            return Some(Box::new(limiters::Zogre {}));
        }
    }

    if monster.info.name.contains("Corporeal Beast") && !player.is_using_corpbane_weapon() {
        return Some(Box::new(limiters::CorporealBeast {}));
    }

    None
}

pub fn simulate_n_fights(
    player: &mut Player,
    monster: &mut Monster,
    n: u32,
) -> (f64, f64, HashMap<u32, f64>) {
    // Check if the monster is immune before running simulations
    if monster.is_immune(player) {
        panic!("{} is immune to this setup", monster.info.name);
    }

    // Set up result variables (probably will make a struct for this)
    let mut ttks = Vec::new();
    let mut hit_counts = Vec::new();
    let mut hit_attempt_counts = Vec::new();
    let mut hit_amounts = Vec::new();
    let mut rng = rand::thread_rng();

    // Retrieve attack function and limiter
    let limiter = assign_limiter(player, monster);
    player.attack = get_attack_functions(player);

    for _ in 0..n {
        // Run a single fight simulation and update the result variables
        let result = single_way::simulate_fight(player, monster, &mut rng, &limiter);
        ttks.push(result.ttk);
        hit_counts.push(result.hit_count);
        hit_attempt_counts.push(result.hit_attempts);
        hit_amounts.extend(result.hit_amounts);
        monster.reset();
        player.reset_live_stats();
    }

    // Calculate average ttk and accuracy
    let avg_ttk = ttks.iter().sum::<f64>() / n as f64;
    let avg_accuracy = hit_counts.iter().sum::<u32>() as f64
        / hit_attempt_counts.iter().sum::<u32>() as f64
        * 100.0;

    // Convert hit amount Vecs to a HashMap counting the number of times each amount appears
    let hit_counts: HashMap<u32, u32> =
        hit_amounts.iter().fold(HashMap::new(), |mut acc, &value| {
            *acc.entry(value).or_insert(0) += 1;
            acc
        });

    // Convert hit counts into a probability distribution
    let hit_dist = hit_counts
        .iter()
        .map(|(&key, &value)| (key, value as f64 / hit_counts.values().sum::<u32>() as f64))
        .collect();

    (avg_ttk, avg_accuracy, hit_dist)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::equipment::CombatStyle;
    use crate::monster::Monster;
    use crate::player::{Player, PlayerStats};
    use crate::potions::Potion;
    use crate::prayers::{Prayer, PrayerBoost};
    use crate::rolls::calc_player_melee_rolls;

    #[test]
    fn test_simulate_n_fights() {
        let mut player = Player::new();
        player.stats = PlayerStats::default();
        player.prayers.add(PrayerBoost::new(Prayer::Piety));
        player.add_potion(Potion::SuperCombat);

        player.equip("Torva full helm", None);
        player.equip("Amulet of torture", None);
        player.equip("Infernal cape", None);
        player.equip("Rada's blessing 4", None);
        player.equip("Ghrazi rapier", None);
        player.equip("Avernic defender", None);
        player.equip("Torva platebody", None);
        player.equip("Torva platelegs", None);
        player.equip("Ferocious gloves", None);
        player.equip("Primordial boots", None);
        player.equip("Ultor ring", None);

        player.update_bonuses();
        player.set_active_style(CombatStyle::Lunge);
        let mut monster = Monster::new("Ammonite Crab", None).unwrap();
        calc_player_melee_rolls(&mut player, &monster);
        let (ttk, accuracy, all_hits) = simulate_n_fights(&mut player, &mut monster, 100000);

        assert!(num::abs(ttk - 10.2) < 0.1);
        assert!(num::abs(accuracy - 99.04) < 0.1);
        assert_eq!(all_hits.len(), 100000);
    }
}
