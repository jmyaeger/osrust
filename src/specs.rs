use crate::attacks::{base_attack, damage_roll, AttackInfo, Hit};
use crate::effects::CombatEffect;
use crate::equipment::CombatType;
use crate::limiters::Limiter;
use crate::monster::{CombatStat, Monster, StatDrain};
use crate::player::Player;
use crate::rolls::calc_player_magic_rolls;
use crate::spells::{SpecialSpell, Spell};
use rand::rngs::ThreadRng;
use std::cmp::max;

pub type SpecialAttackFn =
    fn(&mut Player, &mut Monster, &mut ThreadRng, &Option<Box<dyn Limiter>>) -> Hit;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SpecialAttack {
    pub cost: u32,
    pub function: SpecialAttackFn,
}

pub fn fang_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost accuracy by 50%
    info.max_att_roll = info.max_att_roll * 3 / 2;

    // Spec still has a min hit, as far as I know
    info.min_hit = info.max_hit * 15 / 100;

    let mut hit = base_attack(&info, rng);

    if hit.success {
        hit.apply_transforms(monster, rng, limiter);
    }

    hit
}

pub fn dragon_crossbow_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost max hit by 20%
    info.max_hit = info.max_hit * 6 / 5;

    let damage = damage_roll(info.min_hit, info.max_hit, rng);

    // Hit is always successful
    let mut hit = Hit::accurate(damage);
    hit.apply_transforms(monster, rng, limiter);

    hit
}

pub fn arclight_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Spec always rolls against stab
    info.max_def_roll = monster.def_rolls[&CombatType::Stab];

    let mut hit = base_attack(&info, rng);

    if hit.success {
        hit.apply_transforms(monster, rng, limiter);

        // Drains are twice as effective on demons
        let demon_mod = if monster.is_demon() { 2 } else { 1 };

        // Drain stats by 1 + 5% or 10%
        monster.drain_stat(
            CombatStat::Attack,
            monster.live_stats.attack * demon_mod / 20 + 1,
            None,
        );
        monster.drain_stat(
            CombatStat::Strength,
            monster.live_stats.strength * demon_mod / 20 + 1,
            None,
        );
        monster.drain_stat(
            CombatStat::Defence,
            monster.live_stats.defence * demon_mod / 20 + 1,
            None,
        );
    }

    hit
}

pub fn ancient_gs_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost accuracy by 100% and max hit by 10%
    info.max_att_roll *= 2;
    info.max_hit = info.max_hit * 11 / 10;

    // Spec always rolls against slash
    info.max_def_roll = monster.def_rolls[&CombatType::Slash];

    let mut hit = base_attack(&info, rng);

    if hit.success {
        // Add delayed attack and heal if the hit is successful
        hit.apply_transforms(monster, rng, limiter);
        monster.active_effects.push(CombatEffect::DelayedAttack {
            tick_delay: Some(9),
            damage: 25,
        });
        player.active_effects.push(CombatEffect::DelayedHeal {
            tick_delay: Some(9),
            heal: 25,
        })
    }

    hit
}

pub fn eldritch_staff_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    // Store previous spell if there is one
    let previous_spell = player.attrs.spell;

    // Set spell to Invocate and recalculate max hit
    player.set_spell(Spell::Special(SpecialSpell::Invocate));
    calc_player_magic_rolls(player, monster);

    // Perform an accurate hit
    let info = AttackInfo::new(player, monster);
    let mut hit = Hit::accurate(damage_roll(info.min_hit, info.max_hit, rng));
    hit.apply_transforms(monster, rng, limiter);

    // Restore prayer by half the damage, up to 120 prayer points
    player.restore_prayer(hit.damage / 2, Some(120));

    // Restore previous spell and recalculate max hit
    if let Some(spell) = previous_spell {
        player.set_spell(spell);
    } else {
        player.attrs.spell = None;
    }

    calc_player_magic_rolls(player, monster);

    hit
}

pub fn blowpipe_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost accuracy by 100% and max hit by 50%
    info.max_att_roll *= 2;
    info.max_hit = info.max_hit * 3 / 2;

    let mut hit = base_attack(&info, rng);

    if hit.success {
        hit.apply_transforms(monster, rng, limiter);

        // Heal the player for half of the damage
        player.heal(hit.damage / 2, None);
    }

    hit
}

pub fn sgs_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost accuracy by 100% and max hit by 10%
    info.max_att_roll *= 2;
    info.max_hit = info.max_hit * 11 / 10;

    // Spec always rolls against slash
    info.max_def_roll = monster.def_rolls[&CombatType::Slash];

    let mut hit = base_attack(&info, rng);

    if hit.success {
        hit.apply_transforms(monster, rng, limiter);

        // Heal player by half the damage (10 minimum) and restore prayer by 1/4 the damage (5 minimum)
        player.heal(max(10, hit.damage / 2), None);
        let prayer_restore = max(5, hit.damage / 4);
        player.restore_prayer(prayer_restore, None);
    }

    hit
}

pub fn bgs_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost accuracy by 100% and max hit by 21%
    info.max_att_roll *= 2;
    info.max_hit = info.max_hit * 121 / 100;

    // Spec always rolls against slash
    info.max_def_roll = monster.def_rolls[&CombatType::Slash];

    let mut hit = base_attack(&info, rng);

    if hit.success {
        hit.apply_transforms(monster, rng, limiter);
    }

    let cap = if monster.info.name.contains("Tekton") && !hit.success {
        Some(10)
    } else {
        None
    };

    let stat_order = vec![
        StatDrain::new(CombatStat::Defence, cap),
        StatDrain::new(CombatStat::Strength, cap),
        StatDrain::new(CombatStat::Attack, cap),
        StatDrain::new(CombatStat::Magic, cap),
        StatDrain::new(CombatStat::Ranged, cap),
    ];

    monster.drain_stats_in_order(hit.damage, stat_order);

    hit
}

pub fn bulwark_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost accuracy by 20%
    info.max_att_roll = info.max_att_roll * 6 / 5;

    // Spec always rolls against crush
    info.max_def_roll = monster.def_rolls[&CombatType::Crush];

    let mut hit = base_attack(&info, rng);

    if hit.success {
        hit.apply_transforms(monster, rng, limiter);
    }

    // Second hit and stat drains only occur in multi
    if player.boosts.in_multi {
        let mut hit2 = base_attack(&info, rng);
        if hit2.success {
            hit2.apply_transforms(monster, rng, limiter);
        }
        hit.combine(&hit2);

        // Reverse order of priority so that attack gets set to highest if it's equal to the other highest stat(s)
        let stats = vec![
            CombatStat::Magic,
            CombatStat::Ranged,
            CombatStat::Strength,
            CombatStat::Attack,
        ];

        // Find the highest stat of the monster
        let mut highest_stat = (CombatStat::Attack, 0);
        for stat in stats {
            match stat {
                CombatStat::Attack => {
                    if player.live_stats.attack > highest_stat.1 {
                        highest_stat = (stat, player.live_stats.attack);
                    }
                }
                CombatStat::Strength => {
                    if player.live_stats.strength > highest_stat.1 {
                        highest_stat = (stat, player.live_stats.strength);
                    }
                }
                CombatStat::Ranged => {
                    if player.live_stats.ranged > highest_stat.1 {
                        highest_stat = (stat, player.live_stats.ranged);
                    }
                }
                CombatStat::Magic => {
                    if player.live_stats.magic > highest_stat.1 {
                        highest_stat = (stat, player.live_stats.magic);
                    }
                }
                _ => unreachable!(),
            }
        }

        // If either attack or strength is the highest stat, drain both of them by 5%
        if highest_stat.0 == CombatStat::Attack || highest_stat.0 == CombatStat::Strength {
            monster.drain_stat(CombatStat::Attack, monster.live_stats.attack / 20, None);
            monster.drain_stat(CombatStat::Strength, monster.live_stats.strength / 20, None);
        } else {
            // Otherwise, drain the highest stat by 5%
            monster.drain_stat(highest_stat.0, highest_stat.1 / 20, None);
        }
    }

    hit
}

pub fn crystal_halberd_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost damage by 10%
    info.max_hit = info.max_hit * 11 / 10;

    // Spec always rolls against slash
    info.max_def_roll = monster.def_rolls[&CombatType::Slash];

    let mut hit = base_attack(&info, rng);

    if hit.success {
        hit.apply_transforms(monster, rng, limiter);
    }

    // Hits twice on monsters larger than 1x1
    if monster.info.size > 1 {
        // Second hit is 25% less accurate
        info.max_att_roll = info.max_att_roll * 3 / 4;

        let mut hit2 = base_attack(&info, rng);
        if hit2.success {
            hit2.apply_transforms(monster, rng, limiter);
        }
        hit.combine(&hit2);
    }

    hit
}

pub fn abyssal_whip_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost accuracy by 25%
    info.max_att_roll = info.max_att_roll * 5 / 4;

    let mut hit = base_attack(&info, rng);
    if hit.success {
        hit.apply_transforms(monster, rng, limiter);
    }
    hit
}

pub fn accursed_sceptre_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost damage and accuracy by 50%
    info.max_hit = info.max_hit * 3 / 2;
    info.max_att_roll = info.max_att_roll * 3 / 2;

    let mut hit = base_attack(&info, rng);
    if hit.success {
        hit.apply_transforms(monster, rng, limiter);

        // Drain magic and defence by up to 15% of base levels (less if already drained)
        let def_level_cap = monster.stats.defence - monster.stats.defence * 15 / 100;
        let magic_level_cap = monster.stats.magic - monster.stats.magic * 15 / 100;

        if monster.live_stats.defence > def_level_cap {
            let def_drain_cap = monster.live_stats.defence - def_level_cap;
            monster.drain_stat(CombatStat::Defence, def_drain_cap, Some(def_level_cap));
        }

        if monster.live_stats.magic > magic_level_cap {
            let magic_drain_cap = monster.live_stats.magic - magic_level_cap;
            monster.drain_stat(CombatStat::Magic, magic_drain_cap, Some(magic_level_cap));
        }
    }
    hit
}

pub fn webweaver_bow_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Accuracy is doubled, and each of the 4 hits does up to 40% of max hit
    info.max_att_roll *= 2;
    info.max_hit = info.max_hit * 2 / 5;

    let mut total_hit = Hit::default();

    // Hits 4 times, independently rolled
    for _ in 0..4 {
        let mut hit = base_attack(&info, rng);
        if hit.success {
            hit.apply_transforms(monster, rng, limiter);
        }
        total_hit = total_hit.combine(&hit);
    }

    total_hit
}

pub fn ancient_mace_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Always rolls against crush
    info.max_def_roll = monster.def_rolls[&CombatType::Crush];

    let mut hit = base_attack(&info, rng);
    if hit.success {
        hit.apply_transforms(monster, rng, limiter);
        player.restore_prayer(hit.damage, None);
    }

    hit
}

pub fn barrelchest_anchor_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);
    info.max_att_roll *= 2;
    info.max_hit = info.max_hit * 11 / 10;

    let mut hit = base_attack(&info, rng);

    // Stat drains happen before transforms, according to Mod Ash
    let drain_order = vec![
        StatDrain::new(CombatStat::Defence, None),
        StatDrain::new(CombatStat::Strength, None),
        StatDrain::new(CombatStat::Attack, None),
        StatDrain::new(CombatStat::Magic, None),
        StatDrain::new(CombatStat::Ranged, None),
    ];
    monster.drain_stats_in_order(hit.damage / 10, drain_order);

    if hit.success {
        hit.apply_transforms(monster, rng, limiter);
    }

    hit
}

pub fn dorgeshuun_weapon_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let info = AttackInfo::new(player, monster);

    // Always hits accurately if it's the first attack
    let mut hit = if player.boosts.first_attack {
        Hit::accurate(damage_roll(info.min_hit, info.max_hit, rng))
    } else {
        base_attack(&info, rng)
    };

    if hit.success {
        hit.apply_transforms(monster, rng, limiter);

        // Drains defence by damage, but only if it hasn't been drained already
        if monster.live_stats.defence == monster.stats.defence {
            monster.drain_stat(CombatStat::Defence, hit.damage, None);
        }
    }

    hit
}

pub fn dragon_scimitar_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost accuracy by 25%
    info.max_att_roll = info.max_att_roll * 5 / 4;

    // Always rolls against slash
    info.max_def_roll = monster.def_rolls[&CombatType::Slash];

    let mut hit = base_attack(&info, rng);
    if hit.success {
        hit.apply_transforms(monster, rng, limiter);
    }

    hit
}

pub fn dragon_warhammer_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost damage by 50%
    info.max_hit = info.max_hit * 3 / 2;

    // Store defence drain amount (30% of current level)
    let def_drain = monster.live_stats.defence * 3 / 10;

    if monster.info.name.contains("Tekton") {
        // DWH spec always hits on first attack on Tekton
        if player.boosts.first_attack {
            let mut hit = Hit::accurate(damage_roll(info.min_hit, info.max_hit, rng));
            hit.apply_transforms(monster, rng, limiter);
            monster.drain_stat(CombatStat::Defence, def_drain, None);

            hit
        } else {
            let mut hit = base_attack(&info, rng);
            if hit.success {
                hit.apply_transforms(monster, rng, limiter);
                monster.drain_stat(CombatStat::Defence, def_drain, None);
            } else {
                // DWH spec still drains 5% of Tekton's defence on a miss
                monster.drain_stat(CombatStat::Defence, monster.live_stats.defence / 20, None);
            }

            hit
        }
    } else {
        let mut hit = base_attack(&info, rng);
        if hit.success {
            hit.apply_transforms(monster, rng, limiter);
            monster.drain_stat(CombatStat::Defence, def_drain, None);
        }

        hit
    }
}

pub fn seercull_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Uses special max hit calc that only accounts for ranged ammo strength
    info.max_hit = player.seercull_spec_max();

    // Spec always hits and drains magic by amount of damage dealt
    let mut hit = Hit::accurate(damage_roll(info.min_hit, info.max_hit, rng));
    hit.apply_transforms(monster, rng, limiter);

    // TODO: Test whether the drain happens before or after transforms
    monster.drain_stat(CombatStat::Magic, hit.damage, None);

    hit
}

pub fn abyssal_bludgeon_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost max hit by 0.5% per missing prayer point
    let damage_mod = 1000 + 5 * (player.stats.prayer - player.live_stats.prayer);
    info.max_hit = info.max_hit * damage_mod / 1000;

    let mut hit = base_attack(&info, rng);
    if hit.success {
        hit.apply_transforms(monster, rng, limiter);
    }

    hit
}

pub fn acb_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);
}
