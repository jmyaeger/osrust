use crate::combat::{FightResult, FightVars, Simulation};
use crate::limiters::Limiter;
use crate::{monster::Monster, player::Player};
use rand::rngs::ThreadRng;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum GraardorMethod {
    DoorAltar,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GraardorConfig {
    pub method: GraardorMethod,
    pub eat_hp: u32,
}

pub struct GraardorFight {
    pub player: Player,
    pub graardor: Monster,
    pub melee_minion: Monster,
    pub ranged_minion: Monster,
    pub mage_minion: Monster,
    pub limiter: Option<Box<dyn Limiter>>,
    pub rng: ThreadRng,
    pub config: GraardorConfig,
}

impl GraardorFight {
    pub fn new(player: Player, config: GraardorConfig) -> GraardorFight {
        let graardor = Monster::new("General Graardor", None).unwrap();
        let melee_minion = Monster::new("Sergeant Strongstack", None).unwrap();
        let ranged_minion = Monster::new("Sergeant Grimspike", None).unwrap();
        let mage_minion = Monster::new("Sergeant Steelwill", None).unwrap();
        let limiter = crate::combat::assign_limiter(&player, &graardor);
        let rng = rand::thread_rng();
        GraardorFight {
            player,
            graardor,
            melee_minion,
            ranged_minion,
            mage_minion,
            limiter,
            rng,
            config,
        }
    }
}

impl Simulation for GraardorFight {
    fn simulate(&mut self) -> FightResult {
        match self.config.method {
            GraardorMethod::DoorAltar => simulate_door_altar_fight(
                &mut self.player,
                &mut self.graardor,
                &mut self.melee_minion,
                &mut self.ranged_minion,
                &mut self.mage_minion,
                &mut self.rng,
                &self.limiter,
            ),
        }
    }

    fn is_immune(&self) -> bool {
        self.graardor.is_immune(&self.player)
    }

    fn player(&self) -> &Player {
        &self.player
    }

    fn monster(&self) -> &Monster {
        &self.graardor
    }

    fn set_attack_function(&mut self) {
        self.player.attack = crate::attacks::get_attack_functions(&self.player);
    }

    fn reset(&mut self) {
        self.player.reset_live_stats();
        self.graardor.reset();
        self.melee_minion.reset();
        self.ranged_minion.reset();
        self.mage_minion.reset();
    }
}

fn simulate_door_altar_fight(
    player: &mut Player,
    graardor: &mut Monster,
    melee_minion: &mut Monster,
    ranged_minion: &mut Monster,
    mage_minion: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> FightResult {
    let mut vars = FightVars::new();
    let mut mage_attack_tick = 1;
    let mut melee_attack_tick = 5;
    let player_attack = player.attack;

    while graardor.live_stats.hitpoints > 0 {
        if vars.tick_counter == vars.attack_tick {
            // Process player attack
            let hit = player_attack(player, graardor, rng, limiter);
            player.boosts.first_attack = false;
            graardor.take_damage(hit.damage);
            vars.hit_attempts += 1;
            vars.hit_count += if hit.success { 1 } else { 0 };
            vars.hit_amounts.push(hit.damage);
            vars.attack_tick += player.gear.weapon.speed;
        }

        // Process effects and apply damage
        let mut effect_damage = 0;
        for effect in &mut graardor.active_effects {
            effect_damage += effect.apply();
        }

        graardor.take_damage(effect_damage);
        graardor.clear_inactive_effects();

        // Process mage minion attack
        if vars.tick_counter == mage_attack_tick {}

        // Increment tick counter
        vars.tick_counter += 1;
    }

    FightResult::new()
}
