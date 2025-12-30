use std::num::ParseIntError;

use thiserror::Error;

use crate::{
    combat::simulation::FightResult,
    types::{equipment::CombatStyle, player::SwitchType, spells::Spell},
};

#[derive(Error, Debug)]
pub enum DpsCalcError {
    #[error("No pickaxe bonus for {0}")]
    NoPickaxeBonus(String),
    #[error("Special attack not implemented in the DPS calc for {0}")]
    SpecNotImplemented(String),
    #[error("Missing hit distribution for {monster_name} at {hp} HP")]
    MissingHpHitDist { monster_name: String, hp: usize },
    #[error("Player attack roll error: {0}")]
    PlayerAttackRollError(#[from] PlayerError),
}

#[derive(Error, Debug)]
pub enum SimulationError {
    #[error("Player died before the monster did.")]
    PlayerDeathError(FightResult),
    #[error("Simulation config error: {0}")]
    ConfigError(String),
    #[error("Monster {0} is immune to the player.")]
    MonsterImmune(String),
    #[error("Equipped gear is not usable in the Gauntlet.")]
    InvalidGauntletGear,
    #[error("Monster attack error: {0}")]
    MonsterAttack(#[from] MonsterError),
    #[error("Error switching player styles: {0}")]
    SwitchingError(#[from] PlayerError),
    #[error("Error creating monster: {0}")]
    MonsterCreationError(String),
}

#[derive(Error, Debug)]
pub enum MonsterError {
    #[error("Unknown attribute: {0}")]
    UnknownMonsterAttribute(String),
    #[error("Invalid burn type: {0}")]
    InvalidBurnType(String),
    #[error("Error reading monster JSON: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Monster {0} not found.")]
    MonsterNotFound(String),
    #[error("Could not find monster JSON: {0}")]
    JsonFileNotFound(#[from] std::io::Error),
    #[error("Attack type must be specified for monster {0}.")]
    AttackTypeNotSpecified(String),
    #[error("No max hits found for {monster_name} for {attack_type} attack type.")]
    MaxHitNotFound {
        monster_name: String,
        attack_type: String,
    },
    #[error("Special attack type not supported.")]
    SpecialAttackNotSupported,
    #[error("None attack type not supported.")]
    NoneAttackNotSupported,
}

#[derive(Error, Debug)]
pub enum PlayerError {
    #[error("Players do not have generic ranged attack rolls.")]
    NoGenericRangedStyle,
    #[error("Gear switch not found: {0}")]
    GearSwitchNotFound(SwitchType),
    #[error("Player magic level too low to cast {0}.")]
    MagicLevelTooLow(Spell),
    #[error("Error parsing player stats: {0}")]
    StatParseError(#[from] std::num::ParseIntError),
    #[error("Error fetching player data: {0}")]
    StatLookupError(#[from] reqwest::Error),
    #[error("Equipped weapon {weapon_name} does not have the {style} combat style.")]
    CombatStyleMismatch {
        weapon_name: String,
        style: CombatStyle,
    },
}

#[derive(Error, Debug)]
pub enum MathError {
    #[error("Fraction has {0} components.")]
    InvalidFraction(usize),
    #[error("Error parsing fraction component: {0}")]
    NumeratorParseError(#[from] ParseIntError),
    #[error("Fraction denominator cannot be zero.")]
    DivideByZero,
}

#[derive(Error, Debug)]
pub enum GearError {
    #[error("Equipment not found: {name}")]
    EquipmentNotFound {
        name: String,
        version: Option<String>,
    },
    #[error("Error creating equipment: {0}")]
    CreationError(String),
    #[error("Item {item_name} is not a weapon  (slot: {slot}).")]
    NotAWeapon { item_name: String, slot: String },
    #[error("Item {0} is a weapon, not armor.")]
    NotArmor(String),
    #[error("Weapon {0} missing category field.")]
    MissingWeaponCategory(String),
    #[error("Weapon {0} is missing the speed field.")]
    MissingWeaponSpeed(String),
    #[error("Weapon {0} is missing the attack range field.")]
    MissingAttackRange(String),
    #[error("Weapon {0} is missing the is_two_handed field.")]
    MissingTwoHandedField(String),
    #[error("Unknown slot: {0}")]
    UnknownSlot(String),
    #[error("Error parsing equipment JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("Error opening equipment JSON: {0}")]
    JsonReadError(#[from] std::io::Error),
}
