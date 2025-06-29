use crate::types::monster::AttackType;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Thrall {
    LesserMelee,
    LesserRanged,
    LesserMagic,
    SuperiorMelee,
    SuperiorRanged,
    SuperiorMagic,
    GreaterMelee,
    GreaterRanged,
    GreaterMagic,
}

impl Thrall {
    pub fn max_hit(&self) -> u32 {
        match *self {
            Thrall::LesserMelee | Thrall::LesserRanged | Thrall::LesserMagic => 1,
            Thrall::SuperiorMelee | Thrall::SuperiorRanged | Thrall::SuperiorMagic => 2,
            Thrall::GreaterMelee | Thrall::GreaterRanged | Thrall::GreaterMagic => 3,
        }
    }

    pub fn attack_type(&self) -> AttackType {
        match *self {
            Thrall::LesserMelee | Thrall::SuperiorMelee | Thrall::GreaterMelee => AttackType::Melee,
            Thrall::LesserRanged | Thrall::SuperiorRanged | Thrall::GreaterRanged => {
                AttackType::Ranged
            }
            Thrall::LesserMagic | Thrall::SuperiorMagic | Thrall::GreaterMagic => AttackType::Magic,
        }
    }
}
