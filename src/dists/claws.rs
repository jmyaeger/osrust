use crate::hit_dist::{AttackDistribution, HitDistribution, Hitsplat, WeightedHit};

fn generate_totals(
    acc_roll: u32,
    total_rolls: u32,
    accuracy: f64,
    max_hit: u32,
    high_offset: i32,
) -> (f64, u32, u32) {
    let low = max_hit * (total_rolls - acc_roll) / 4;
    let high = (max_hit as i32 + low as i32 + high_offset) as u32;
    let chance_prev_rolls_fail = (1.0 - accuracy).powi(acc_roll as i32);
    let chance_this_roll_passes = chance_prev_rolls_fail * accuracy;
    let chance_per_dmg = chance_this_roll_passes / (high - low + 1) as f64;

    (chance_per_dmg, low, high)
}

pub fn dragon_claw_dist(accuracy: f64, max_hit: u32) -> AttackDistribution {
    let mut dist = HitDistribution::default();

    for acc_roll in 0..4 {
        let (chance_per_dmg, low, high) = generate_totals(acc_roll, 4, accuracy, max_hit, -1);
        for dmg in low..=high {
            match acc_roll {
                0 => {
                    dist.add_hit(WeightedHit::new(
                        chance_per_dmg,
                        vec![
                            Hitsplat::new(dmg / 2, true),
                            Hitsplat::new(dmg / 4, true),
                            Hitsplat::new(dmg / 8, true),
                            Hitsplat::new(dmg / 8 + 1, true),
                        ],
                    ));
                }
                1 => {
                    dist.add_hit(WeightedHit::new(
                        chance_per_dmg,
                        vec![
                            Hitsplat::inaccurate(),
                            Hitsplat::new(dmg / 2, true),
                            Hitsplat::new(dmg / 4, true),
                            Hitsplat::new(dmg / 4 + 1, true),
                        ],
                    ));
                }
                2 => {
                    dist.add_hit(WeightedHit::new(
                        chance_per_dmg,
                        vec![
                            Hitsplat::inaccurate(),
                            Hitsplat::inaccurate(),
                            Hitsplat::new(dmg / 2, true),
                            Hitsplat::new(dmg / 2 + 1, true),
                        ],
                    ));
                }
                _ => {
                    dist.add_hit(WeightedHit::new(
                        chance_per_dmg,
                        vec![
                            Hitsplat::inaccurate(),
                            Hitsplat::inaccurate(),
                            Hitsplat::inaccurate(),
                            Hitsplat::new(dmg + 1, true),
                        ],
                    ));
                }
            }
        }
    }

    let chance_all_fail = (1.0 - accuracy).powi(4);
    dist.add_hit(WeightedHit::new(
        chance_all_fail * 2.0 / 3.0,
        vec![
            Hitsplat::new(1, false),
            Hitsplat::new(1, false),
            Hitsplat::inaccurate(),
            Hitsplat::inaccurate(),
        ],
    ));
    dist.add_hit(WeightedHit::new(
        chance_all_fail / 3.0,
        vec![
            Hitsplat::inaccurate(),
            Hitsplat::inaccurate(),
            Hitsplat::inaccurate(),
            Hitsplat::inaccurate(),
        ],
    ));

    AttackDistribution::new(vec![dist])
}

pub fn burning_claw_spec(accuracy: f64, max: u32) -> AttackDistribution {
    let mut dist = HitDistribution::new(vec![]);

    for acc_roll in 0..3 {
        let (chance_per_dmg, low, high) = generate_totals(acc_roll, 3, accuracy, max, 0);
        for dmg in low..=high {
            match acc_roll {
                0 => {
                    dist.add_hit(WeightedHit::new(
                        chance_per_dmg,
                        vec![
                            Hitsplat::new(dmg / 2, true),
                            Hitsplat::new(dmg / 4, true),
                            Hitsplat::new(dmg / 4, true),
                        ],
                    ));
                }
                1 => {
                    dist.add_hit(WeightedHit::new(
                        chance_per_dmg,
                        vec![
                            Hitsplat::new((dmg / 2).saturating_sub(1), true),
                            Hitsplat::new((dmg / 2).saturating_sub(1), true),
                            Hitsplat::new(2, true),
                        ],
                    ));
                }
                _ => {
                    dist.add_hit(WeightedHit::new(
                        chance_per_dmg,
                        vec![
                            Hitsplat::new(dmg.saturating_sub(2), true),
                            Hitsplat::new(1, true),
                            Hitsplat::new(1, true),
                        ],
                    ));
                }
            }
        }
    }

    let chance_all_fail = (1.0 - accuracy).powi(3);
    dist.add_hit(WeightedHit::new(
        chance_all_fail / 5.0,
        vec![
            Hitsplat::inaccurate(),
            Hitsplat::inaccurate(),
            Hitsplat::inaccurate(),
        ],
    ));
    dist.add_hit(WeightedHit::new(
        2.0 * chance_all_fail / 5.0,
        vec![
            Hitsplat::new(1, false),
            Hitsplat::inaccurate(),
            Hitsplat::inaccurate(),
        ],
    ));
    dist.add_hit(WeightedHit::new(
        2.0 * chance_all_fail / 5.0,
        vec![
            Hitsplat::new(1, false),
            Hitsplat::new(1, false),
            Hitsplat::inaccurate(),
        ],
    ));

    AttackDistribution::new(vec![dist])
}
