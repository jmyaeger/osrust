use rand::{rngs::ThreadRng, Rng};
use std::cmp::{max, min};

pub trait Limiter {
    fn apply(&self, damage: u32, rng: &mut ThreadRng) -> u32;
}

pub struct Zulrah {}

impl Limiter for Zulrah {
    fn apply(&self, damage: u32, rng: &mut ThreadRng) -> u32 {
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
        let second_roll = rng.gen_range(22..=24);
        min(damage, second_roll)
    }
}

pub struct Kraken {}

impl Limiter for Kraken {
    fn apply(&self, damage: u32, _: &mut ThreadRng) -> u32 {
        max(1, damage / 7)
    }
}

pub struct VerzikP1 {
    pub limit: u32,
}

impl Limiter for VerzikP1 {
    fn apply(&self, damage: u32, rng: &mut ThreadRng) -> u32 {
        let second_roll = rng.gen_range(0..=self.limit);
        min(damage, second_roll)
    }
}

pub struct Tekton {}

impl Limiter for Tekton {
    fn apply(&self, damage: u32, _: &mut ThreadRng) -> u32 {
        damage / 5
    }
}

pub struct OneThirdDamage {}

impl Limiter for OneThirdDamage {
    fn apply(&self, damage: u32, _: &mut ThreadRng) -> u32 {
        damage / 3
    }
}

pub struct Zogre {}

impl Limiter for Zogre {
    fn apply(&self, damage: u32, _: &mut ThreadRng) -> u32 {
        damage / 4
    }
}

pub struct ZogreCrumbleUndead {}

impl Limiter for ZogreCrumbleUndead {
    fn apply(&self, damage: u32, _: &mut ThreadRng) -> u32 {
        damage / 2
    }
}
