use thiserror::Error;

use crate::combat::simulation::FightResult;

#[derive(Error, Debug)]
pub enum DpsCalcError {
    #[error("No pickaxe bonus for {0}")]
    NoPickaxeBonus(String),
    #[error("Special attack not implemented in the DPS calc for {0}")]
    SpecNotImplemented(String),
    #[error("Missing hit distribution for {monster_name} at {hp} HP")]
    MissingHpHitDist { monster_name: String, hp: usize },
}

#[derive(Error, Debug)]
pub enum SimulationError {
    #[error("Player died before the monster did.")]
    PlayerDeathError(FightResult),
    #[error("Simulation config error: {0}")]
    ConfigError(String),
    #[error("Monster {0} is immune to the player")]
    MonsterImmune(String),
}
