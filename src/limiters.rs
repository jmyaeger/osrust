use dyn_clone::DynClone;
use rand::{rngs::ThreadRng, Rng};
use std::cmp::{max, min};

// Trait for any post-roll damage transforms applied by the opponent
pub trait Limiter: DynClone {
    fn apply(&self, damage: u32, rng: &mut ThreadRng) -> u32;
}

dyn_clone::clone_trait_object!(Limiter);

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct OneThirdDamage {}

impl Limiter for OneThirdDamage {
    fn apply(&self, damage: u32, _: &mut ThreadRng) -> u32 {
        // Divide damage by 3 (multiple NPCs use this)
        damage / 3
    }
}

#[derive(Clone)]
pub struct Zogre {}

impl Limiter for Zogre {
    fn apply(&self, damage: u32, _: &mut ThreadRng) -> u32 {
        // Zogres take 1/4 damage from most weapons
        damage / 4
    }
}

#[derive(Clone)]
pub struct HalfDamage {}

impl Limiter for HalfDamage {
    fn apply(&self, damage: u32, _: &mut ThreadRng) -> u32 {
        damage / 2
    }
}

#[derive(Clone)]
pub struct HueycoatlTail {
    pub limit: u32,
}

impl Limiter for HueycoatlTail {
    fn apply(&self, damage: u32, rng: &mut ThreadRng) -> u32 {
        // Hueycoatl's tail rolls a number betwwen 0-9 (crush weapon) or 0-4 (other styles)
        // and takes the lower of the roll and the original damage roll
        let second_roll = rng.gen_range(0..=self.limit);
        min(damage, second_roll)
    }
}
