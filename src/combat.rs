use crate::attacks::get_attack_functions;
use crate::equipment::CombatType;
use crate::limiters;
use crate::monster::Monster;
use crate::player::Player;
use crate::sims::single_way;
use crate::spells::{Spell, StandardSpell};

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
    if monster.info.name.contains("Zulrah") {
        return Some(Box::new(limiters::Zulrah {}));
    }

    if monster.info.name.contains("Fragment of Seren") {
        return Some(Box::new(limiters::Seren {}));
    }

    if monster.info.name.as_str() == "Kraken (Kraken)" && player.combat_type() == CombatType::Ranged
    {
        return Some(Box::new(limiters::Kraken {}));
    }

    if monster.info.name.contains("Verzik")
        && monster.info.name.contains("P1")
        && !player.is_wearing("Dawnbringer")
    {
        let limit = if player.is_using_melee() { 10 } else { 3 };
        return Some(Box::new(limiters::VerzikP1 { limit }));
    }

    if monster.info.name.contains("Tekton") && player.combat_type() == CombatType::Magic {
        return Some(Box::new(limiters::Tekton {}));
    }

    if (monster.info.name.contains("Glowing crystal") && player.combat_type() == CombatType::Magic)
        || (monster.info.name.contains("(Left claw")
            || monster.info.name.contains("Great Olm (Head")
                && player.combat_type() == CombatType::Magic)
        || (monster.info.name.contains("(Right claw")
            || monster.info.name.contains("(Left claw")
                && player.combat_type() == CombatType::Ranged)
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
        } else if player.combat_type() != CombatType::Ranged
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
    None
}

pub fn simulate_n_fights(
    player: &mut Player,
    monster: &mut Monster,
    n: u32,
) -> (f64, f64, Vec<Vec<u32>>) {
    if monster.is_immune(player) {
        panic!("{} is immune to this setup", monster.info.name);
    }

    let mut ttks = Vec::new();
    let mut hit_counts = Vec::new();
    let mut hit_attempt_counts = Vec::new();
    let mut hit_amounts = Vec::new();
    let mut rng = rand::thread_rng();

    let limiter = assign_limiter(player, monster);
    player.attack = get_attack_functions(player);

    for _ in 0..n {
        let result = single_way::simulate_fight(player, monster, &mut rng, &limiter);
        ttks.push(result.ttk);
        hit_counts.push(result.hit_count);
        hit_attempt_counts.push(result.hit_attempts);
        hit_amounts.push(result.hit_amounts);
        monster.reset();
        player.reset_live_stats();
    }

    let avg_ttk = ttks.iter().sum::<f64>() / n as f64;
    let avg_accuracy = hit_counts.iter().sum::<u32>() as f64
        / hit_attempt_counts.iter().sum::<u32>() as f64
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
        player.stats = PlayerStats::default();
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
