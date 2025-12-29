use thiserror::Error;

#[derive(Error, Debug)]
pub enum DpsCalcError {
    #[error("No pickaxe bonus for {0}")]
    NoPickaxeBonus(String),
    #[error("Special attack not implemented in the DPS calc for {0}")]
    SpecNotImplemented(String),
    #[error("Missing hit distribution for {monster_name} at {hp} HP")]
    MissingHpHitDist { monster_name: String, hp: usize },
}
