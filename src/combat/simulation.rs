use crate::combat::limiters;
use crate::constants::HUEYCOATL_TAIL_ID;
use crate::types::equipment::CombatType;
use crate::types::monster::Monster;
use crate::types::player::Player;
use crate::types::spells::{Spell, StandardSpell};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct FightResult {
    pub ttk_ticks: i32,
    pub hit_attempts: u32,
    pub hit_count: u32,
    pub hit_amounts: Vec<u32>,
    pub food_eaten: u32,
    pub damage_taken: u32,
    pub leftover_burn: u32,
    pub thrall_damage: u32,
}

impl FightResult {
    pub fn new() -> Self {
        Self::default()
    }
}

pub enum SimulationError {
    PlayerDeathError(FightResult),
    ConfigError(String),
}

impl std::fmt::Display for SimulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SimulationError::PlayerDeathError(_) => {
                write!(f, "Player died before the monster did.")
            }
            SimulationError::ConfigError(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::fmt::Debug for SimulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CumulativeResults {
    pub ttks_ticks: Vec<i32>,
    pub hit_attempt_counts: Vec<u32>,
    pub hit_counts: Vec<u32>,
    pub hit_amounts: Vec<u32>,
    pub player_deaths: usize,
    pub food_eaten: Vec<u32>,
    pub damage_taken: Vec<u32>,
    pub leftover_burn: Vec<u32>,
    pub thrall_damage: Vec<u32>,
}

impl CumulativeResults {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, result: &FightResult) {
        self.hit_attempt_counts.push(result.hit_attempts);
        self.hit_counts.push(result.hit_count);
        self.hit_amounts.extend(&result.hit_amounts);
        self.ttks_ticks.push(result.ttk_ticks);
        self.food_eaten.push(result.food_eaten);
        self.damage_taken.push(result.damage_taken);
        self.leftover_burn.push(result.leftover_burn);
        self.thrall_damage.push(result.thrall_damage);
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct FightVars {
    pub tick_counter: i32,
    pub hit_attempts: u32,
    pub hit_count: u32,
    pub hit_amounts: Vec<u32>,
    pub attack_tick: i32,
    pub thrall_attack_tick: i32,
    pub freeze_immunity: i32,
    pub freeze_resistance: u32,
    pub food_eaten: u32,
    pub damage_taken: u32,
    pub eat_delay: u32,
    pub thrall_damage: u32,
    pub spec_regen_timer: Option<i32>,
}

impl FightVars {
    pub fn new() -> Self {
        Self {
            thrall_attack_tick: 1,
            ..Default::default()
        }
    }
}

pub trait Simulation {
    fn simulate(&mut self) -> Result<FightResult, SimulationError>;
    fn is_immune(&self) -> bool;
    fn player(&self) -> &Player;
    fn monster(&self) -> &Monster;
    fn set_attack_function(&mut self);
    fn reset(&mut self);
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

    if ((monster.info.name.contains("Glowing crystal")
        && player.combat_type() == CombatType::Magic)
        || ((monster.matches_version("Left claw")
            || (monster.info.name.contains("Great Olm") && monster.matches_version("Head")))
            && player.combat_type() == CombatType::Magic))
        || (monster.matches_version("Right claw")
            || monster.matches_version("Left claw") && player.is_using_ranged())
        || (monster.info.name.contains("Ice demon")
            && !player.is_using_fire_spell()
            && !player.is_using_demonbane()
            && player.attrs.spell != Some(Spell::Standard(StandardSpell::FlamesOfZamorak)))
        || (monster.info.name.contains("Slagilith") && !player.gear.weapon.name.contains("pickaxe"))
    {
        return Some(Box::new(limiters::OneThirdDamage {}));
    }

    if ["Slash Bash", "Zogre", "Skogre"].contains(&monster.info.name.as_str()) {
        if player.attrs.spell == Some(Spell::Standard(StandardSpell::CrumbleUndead)) {
            return Some(Box::new(limiters::HalfDamage {}));
        } else if !player.is_using_ranged()
            || !player
                .gear
                .ammo
                .as_ref()
                .is_some_and(|ammo| ammo.name.contains(" brutal"))
            || !player.gear.weapon.name.contains("Comp ogre bow")
        {
            return Some(Box::new(limiters::Zogre {}));
        }
    }

    if monster.info.name.contains("Corporeal Beast") && !player.is_using_corpbane_weapon() {
        return Some(Box::new(limiters::HalfDamage {}));
    }

    if player.is_wearing("Efaritay's aid", None) && monster.vampyre_tier() == Some(2) {
        return Some(Box::new(limiters::HalfDamage {}));
    }

    if monster.info.id == Some(HUEYCOATL_TAIL_ID) {
        let using_crush = player.combat_type() == CombatType::Crush
            && player.bonuses.attack.crush > player.bonuses.attack.stab
            && player.bonuses.attack.crush > player.bonuses.attack.slash;
        let dist_max = if using_crush { 9 } else { 4 };
        return Some(Box::new(limiters::HueycoatlTail { limit: dist_max }));
    }

    None
}

pub fn simulate_n_fights(mut simulation: Box<dyn Simulation>, n: u32) -> CumulativeResults {
    // Check if the monster is immune before running simulations
    if simulation.is_immune() {
        panic!("The monster is immune to the player in this setup");
    }

    // Set up result variables
    let mut results = CumulativeResults::new();

    // Retrieve attack function and limiter
    simulation.set_attack_function();

    for _ in 0..n {
        // Run a single fight simulation and update the result variables
        let result = simulation.simulate();
        match result {
            Ok(result) => {
                results.push(&result);
            }
            Err(e) => match e {
                SimulationError::PlayerDeathError(result) => {
                    results.player_deaths += 1;
                    results.hit_amounts.extend(&result.hit_amounts);
                    results.hit_attempt_counts.push(result.hit_attempts);
                    results.hit_counts.push(result.hit_count);
                    results.food_eaten.push(result.food_eaten);
                    results.damage_taken.push(result.damage_taken);
                    results.leftover_burn.push(result.leftover_burn);
                    results.thrall_damage.push(result.thrall_damage);
                }
                SimulationError::ConfigError(e) => panic!("Configuration error: {e}"),
            },
        }
        simulation.reset();
    }

    results

    // Return a struct with average ttk, average accuracy, and hit distribution
    // SimulationStats::new(&results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calc::analysis::SimulationStats;
    use crate::calc::rolls::calc_player_melee_rolls;
    use crate::sims::single_way::{SingleWayConfig, SingleWayFight};
    use crate::types::equipment::CombatStyle;
    use crate::types::monster::Monster;
    use crate::types::player::Player;
    use crate::types::potions::Potion;
    use crate::types::prayers::Prayer;
    use crate::types::stats::PlayerStats;

    #[test]
    fn test_simulate_n_fights() {
        let mut player = Player::new();
        player.stats = PlayerStats::default();
        player.prayers.add(Prayer::Piety);
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
        let monster = Monster::new("Ammonite Crab", None).unwrap();
        calc_player_melee_rolls(&mut player, &monster);
        let simulation =
            SingleWayFight::new(player, monster, SingleWayConfig::default(), None, false);
        let results = simulate_n_fights(Box::new(simulation), 100000);
        let stats = SimulationStats::new(&results);

        assert!(num::abs(stats.ttk - 10.2) < 0.1);
        assert!(num::abs(stats.accuracy - 99.04) < 0.1);
    }
}
