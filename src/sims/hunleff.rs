use crate::combat::{FightResult, FightVars, Simulation, SimulationError};
use crate::limiters::Limiter;
use crate::monster::{AttackType, Monster};
use crate::player::Player;
use rand::rngs::ThreadRng;

#[derive(Debug, PartialEq, Clone)]
pub struct HunleffConfig {
    pub food_count: u32, // Only normal paddlefish for now
    pub eat_strategy: EatStrategy,
    pub redemption_attempts: u32, // Attempt to use redemption a certain number of times at the beginning
    pub attack_strategy: AttackStrategy,
    pub corrupted: bool,
    pub armor_tier: ArmorTier,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EatStrategy {
    EatAtHp(u32),          // Eat as soon as HP goes below threshold
    TickEatOnly,           // Allow health to go below max hit and then tick eat
    EatToFullDuringNadoes, // Don't eat until tornadoes unless necessary, then eat to full
}

#[derive(Debug, PartialEq, Clone)]
pub enum AttackStrategy {
    TwoT3Weapons(Player, Player),
    FiveToOne {
        style1: Player,
        style2: Player,
        style3: Player,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArmorTier {
    None,
    One,
    Two,
    Three,
}

#[derive(Clone)]
pub struct HunleffFight {
    pub player: Player,
    pub hunleff: Monster,
    pub limiter: Option<Box<dyn Limiter>>,
    pub rng: ThreadRng,
    pub config: HunleffConfig,
}

impl HunleffFight {
    pub fn new(player: Player, config: HunleffConfig) -> HunleffFight {
        let hunleff = if config.corrupted {
            Monster::new("Corrupted Hunleff", None).unwrap()
        } else {
            Monster::new("Crystalline Hunleff", None).unwrap()
        };
        let limiter = crate::combat::assign_limiter(&player, &hunleff);
        let rng = rand::thread_rng();
        HunleffFight {
            player,
            hunleff,
            limiter,
            rng,
            config,
        }
    }
}
