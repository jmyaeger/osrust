use rand::{rngs::ThreadRng, Rng};
use std::cmp::{max, min};

// Trait for any post-roll damage transforms applied by the opponent
pub trait Limiter {
    fn apply(&self, damage: u32, rng: &mut ThreadRng) -> u32;
}

pub struct Zulrah {}

impl Limiter for Zulrah {
    fn apply(&self, damage: u32, rng: &mut ThreadRng) -> u32 {
        // Zulrah rerolls any number over 50 to be within 45-50
        if damage > 50 {
            rng.gen_range(45..=50)
        } else {
            damage
        }
    }
}

pub struct Seren {}

impl Limiter for Seren {
    fn apply(&self, damage: u32, rng: &mut ThreadRng) -> u32 {
        // Seren rolls a second number between 22-24 and takes the lower of the two
        if damage > 22 {
            let second_roll = rng.gen_range(22..=24);
            min(damage, second_roll)
        } else {
            damage
        }
    }
}

pub struct Kraken {}

impl Limiter for Kraken {
    // Kraken divides ranged damage by 7
    fn apply(&self, damage: u32, _: &mut ThreadRng) -> u32 {
        if damage > 0 {
            max(1, damage / 7)
        } else {
            damage
        }
    }
}

pub struct VerzikP1 {
    pub limit: u32,
}

impl Limiter for VerzikP1 {
    fn apply(&self, damage: u32, rng: &mut ThreadRng) -> u32 {
        // Verzik P1 rolls a number betwwen 0-10 (melee) or 0-3 (other styles)
        // and takes the lower of the two
        let second_roll = rng.gen_range(0..=self.limit);
        min(damage, second_roll)
    }
}

pub struct Tekton {}

impl Limiter for Tekton {
    fn apply(&self, damage: u32, _: &mut ThreadRng) -> u32 {
        // Tekton divides magic damage by 5, with a minimum accurate hit of 1
        if damage > 0 {
            max(1, damage / 5)
        } else {
            damage
        }
    }
}

pub struct OneThirdDamage {}

impl Limiter for OneThirdDamage {
    fn apply(&self, damage: u32, _: &mut ThreadRng) -> u32 {
        // Divide damage by 3 (multiple NPCs use this)
        damage / 3
    }
}

pub struct Zogre {}

impl Limiter for Zogre {
    fn apply(&self, damage: u32, _: &mut ThreadRng) -> u32 {
        // Zogres take 1/4 damage from most weapons
        damage / 4
    }
}

pub struct ZogreCrumbleUndead {}

impl Limiter for ZogreCrumbleUndead {
    fn apply(&self, damage: u32, _: &mut ThreadRng) -> u32 {
        // Zogres take half damage from Crumble Undead
        damage / 2
    }
}

pub struct CorporealBeast {}

impl Limiter for CorporealBeast {
    fn apply(&self, damage: u32, _: &mut ThreadRng) -> u32 {
        // Corp takes half damage from most weapons
        damage / 2
    }
}
