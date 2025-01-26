use crate::calc::hit_dist::{HitDistribution, HitTransformer, Hitsplat, WeightedHit};
use crate::types::monster::Monster;
use std::cmp::min;

#[derive(Debug, PartialEq, Clone)]
pub struct BoltContext<'a> {
    pub ranged_lvl: u32,
    pub max_hit: u32,
    pub zcb: bool,
    pub spec: bool,
    pub kandarin_diary: bool,
    pub monster: &'a Monster,
}

impl<'a> BoltContext<'_> {
    pub fn new(
        ranged_lvl: u32,
        max_hit: u32,
        zcb: bool,
        spec: bool,
        kandarin_diary: bool,
        monster: &'a Monster,
    ) -> BoltContext<'a> {
        BoltContext {
            ranged_lvl,
            max_hit,
            zcb,
            spec,
            kandarin_diary,
            monster,
        }
    }
}

fn kandarin_factor(ctx: &BoltContext) -> f64 {
    if ctx.kandarin_diary {
        1.1
    } else {
        1.0
    }
}

fn bonus_damage_transform<'a>(
    ctx: &'a BoltContext,
    chance: f64,
    bonus_dmg: u32,
    accurate_only: bool,
) -> Box<dyn HitTransformer + 'a> {
    Box::new(move |h: &Hitsplat| {
        if h.accurate && ctx.zcb && ctx.spec {
            return HitDistribution::single(1.0, vec![Hitsplat::new(h.damage + bonus_dmg, true)]);
        }
        if !h.accurate && accurate_only {
            return HitDistribution::new(vec![WeightedHit::new(1.0, vec![*h])]);
        }
        HitDistribution::new(vec![
            WeightedHit::new(
                chance,
                vec![Hitsplat::new(h.damage + bonus_dmg, h.accurate)],
            ),
            WeightedHit::new(1.0 - chance, vec![*h]),
        ])
    })
}

pub fn opal_bolts<'a>(ctx: &'a BoltContext) -> Box<dyn HitTransformer + 'a> {
    let chance = 0.05 * kandarin_factor(ctx);
    let bonus_dmg = ctx.ranged_lvl / if ctx.zcb { 9 } else { 10 };
    bonus_damage_transform(ctx, chance, bonus_dmg, false)
}

pub fn pearl_bolts<'a>(ctx: &'a BoltContext) -> Box<dyn HitTransformer + 'a> {
    let chance = 0.06 * kandarin_factor(ctx);
    let divisor = if ctx.monster.is_fiery() { 15 } else { 20 };
    let bonus_dmg = ctx.ranged_lvl / if ctx.zcb { divisor - 2 } else { divisor };
    bonus_damage_transform(ctx, chance, bonus_dmg, false)
}

pub fn diamond_bolts<'a>(ctx: &'a BoltContext) -> Box<dyn HitTransformer + 'a> {
    let chance = 0.1 * kandarin_factor(ctx);
    let effect_max = ctx.max_hit * if ctx.zcb { 126 } else { 115 } / 100;
    let effect_dist = HitDistribution::linear(1.0, 0, effect_max);

    Box::new(move |h: &Hitsplat| {
        if h.accurate && ctx.zcb && ctx.spec {
            return effect_dist.clone();
        }
        let mut dist = effect_dist.scale_probability(chance);
        dist.add_hit(WeightedHit::new(1.0 - chance, vec![*h]));
        dist
    })
}

pub fn dragonstone_bolts<'a>(ctx: &'a BoltContext) -> Box<dyn HitTransformer + 'a> {
    if ctx.monster.is_fiery() || ctx.monster.is_dragon() {
        return Box::new(|h: &Hitsplat| {
            HitDistribution::new(vec![WeightedHit::new(1.0, vec![*h])])
        });
    }

    let chance = 0.06 * kandarin_factor(ctx);
    let bonus_dmg = ctx.ranged_lvl * 2 / if ctx.zcb { 9 } else { 10 };
    bonus_damage_transform(ctx, chance, bonus_dmg, true)
}

pub fn onyx_bolts<'a>(ctx: &'a BoltContext) -> Box<dyn HitTransformer + 'a> {
    if ctx.monster.is_undead() {
        return Box::new(|h: &Hitsplat| {
            HitDistribution::new(vec![WeightedHit::new(1.0, vec![*h])])
        });
    }

    let chance = 0.11 * kandarin_factor(ctx);
    let effect_max = ctx.max_hit * if ctx.zcb { 132 } else { 120 } / 100;
    let effect_dist = HitDistribution::linear(1.0, 0, effect_max);

    Box::new(move |h: &Hitsplat| {
        if !h.accurate {
            return HitDistribution::new(vec![WeightedHit::new(1.0, vec![*h])]);
        }
        if ctx.zcb && ctx.spec {
            return effect_dist.clone();
        }
        let mut dist = effect_dist.scale_probability(chance);
        dist.add_hit(WeightedHit::new(1.0 - chance, vec![*h]));
        dist
    })
}

pub fn ruby_bolts<'a>(ctx: &'a BoltContext) -> Box<dyn HitTransformer + 'a> {
    let chance = 0.06 * kandarin_factor(ctx);
    let cap = if ctx.zcb { 110 } else { 100 };
    let effect_dmg = ctx.monster.live_stats.hitpoints * if ctx.zcb { 22 } else { 20 } / 100;
    let effect_hit = HitDistribution::single(1.0, vec![Hitsplat::new(min(cap, effect_dmg), true)]);

    Box::new(move |h: &Hitsplat| {
        if h.accurate && ctx.zcb && ctx.spec {
            return effect_hit.clone();
        }
        let mut dist = effect_hit.scale_probability(chance);
        dist.add_hit(WeightedHit::new(1.0 - chance, vec![*h]));
        dist
    })
}
