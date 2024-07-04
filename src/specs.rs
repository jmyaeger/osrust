use crate::attacks::{self, base_attack, damage_roll, AttackInfo, Hit};
use crate::constants::VERZIK_IDS;
use crate::effects::CombatEffect;
use crate::equipment::CombatType;
use crate::limiters::Limiter;
use crate::monster::{CombatStat, Monster, StatDrain};
use crate::player::Player;
use crate::rolls::{calc_player_magic_rolls, calc_player_melee_rolls};
use crate::spells::{SpecialSpell, Spell};
use num::clamp;
use rand::rngs::ThreadRng;
use rand::Rng;
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
    // Store base attack roll to restore afterwards
    let old_att_roll = player.att_rolls[&CombatType::Ranged];
    player.boosts.acb_spec = true;

    // Double accuracy
    player
        .att_rolls
        .insert(CombatType::Ranged, old_att_roll * 2);

    // Get the attack function corresponding to the bolt type being used
    let attack_fn = attacks::get_attack_functions(player);
    let hit = attack_fn(player, monster, rng, limiter);

    // Restore base attack roll
    player.att_rolls.insert(CombatType::Ranged, old_att_roll);
    player.boosts.acb_spec = false;

    hit
}

pub fn zcb_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    // Store base attack roll to restore afterwards
    let old_att_roll = player.att_rolls[&CombatType::Ranged];
    player.boosts.zcb_spec = true;

    // Double accuracy
    player
        .att_rolls
        .insert(CombatType::Ranged, old_att_roll * 2);

    // Get the attack function corresponding to the bolt type being used
    let attack_fn = attacks::get_attack_functions(player);
    let hit = attack_fn(player, monster, rng, limiter);

    // Restore base attack roll
    player.att_rolls.insert(CombatType::Ranged, old_att_roll);
    player.boosts.zcb_spec = false;

    hit
}

pub fn ags_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost max hit by 37.5% and accuracy by 100%
    info.max_hit = info.max_hit * 1375 / 1000;
    info.max_att_roll *= 2;

    // Always rolls against slash
    info.max_def_roll = monster.def_rolls[&CombatType::Slash];

    let mut hit = base_attack(&info, rng);
    if hit.success {
        hit.apply_transforms(monster, rng, limiter);
    }

    hit
}

pub fn dawnbringer_spec(
    _player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    _limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    // Rolls 75-150 damage regardless of bonuses or levels, but only on Verzik P1
    if VERZIK_IDS.contains(&monster.info.id.unwrap_or(0)) {
        Hit::accurate(damage_roll(75, 150, rng))
    } else {
        Hit::inaccurate()
    }
}

pub fn dragon_longsword_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost max hit by 25%
    info.max_hit = info.max_hit * 5 / 4;

    // Always rolls against slash
    info.max_def_roll = monster.def_rolls[&CombatType::Slash];

    let mut hit = base_attack(&info, rng);
    if hit.success {
        hit.apply_transforms(monster, rng, limiter);
    }

    hit
}

pub fn dragon_mace_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost accuracy by 25% and max hit by 50%
    info.max_att_roll = info.max_att_roll * 5 / 4;
    info.max_hit = info.max_hit * 3 / 2;

    // Always rolls against crush
    info.max_def_roll = monster.def_rolls[&CombatType::Crush];

    let mut hit = base_attack(&info, rng);
    if hit.success {
        hit.apply_transforms(monster, rng, limiter);
    }

    hit
}

pub fn dragon_sword_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost accuracy by 25% and max hit by 25%
    info.max_att_roll = info.max_att_roll * 5 / 4;
    info.max_hit = info.max_hit * 5 / 4;

    // Always rolls against stab
    info.max_def_roll = monster.def_rolls[&CombatType::Stab];

    let mut hit = base_attack(&info, rng);
    if hit.success {
        hit.apply_transforms(monster, rng, limiter);
    }

    hit
}

pub fn granite_hammer_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost accuracy by 50%
    info.max_att_roll = info.max_att_roll * 3 / 2;

    let mut hit = base_attack(&info, rng);

    // Add 5 damage in all cases, even if not originally successful
    hit.damage += 5;
    hit.success = true;

    hit.apply_transforms(monster, rng, limiter);

    hit
}

pub fn ballista_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost accuracy and max hit by 25%
    info.max_att_roll = info.max_att_roll * 5 / 4;
    info.max_hit = info.max_hit * 5 / 4;

    let mut hit = base_attack(&info, rng);
    if hit.success {
        hit.apply_transforms(monster, rng, limiter);
    }

    hit
}

pub fn magic_longbow_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    // Uses the same max hit calc as the seercull spec (only ammo ranged str is used)
    let max_hit = player.seercull_spec_max();

    // Always accurate
    let mut hit = Hit::accurate(damage_roll(0, max_hit, rng));

    hit.apply_transforms(monster, rng, limiter);

    hit
}

pub fn sara_blessed_sword_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost max hit by 25%
    info.max_hit = info.max_hit * 5 / 4;

    // Rolls against magic
    info.max_def_roll = monster.def_rolls[&CombatType::Magic];

    let mut hit = base_attack(&info, rng);
    if hit.success {
        hit.apply_transforms(monster, rng, limiter);
    }

    hit
}

pub fn voidwaker_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    // Rolls between 50-150% of max hit, always accurate
    let min_hit = player.max_hits[&CombatType::Stab] / 2;
    let max_hit = player.max_hits[&CombatType::Stab] * 3 / 2;

    let mut hit = Hit::accurate(damage_roll(min_hit, max_hit, rng));
    hit.apply_transforms(monster, rng, limiter);

    hit
}

pub fn volatile_staff_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    // Store previous spell if there is one
    let previous_spell = player.attrs.spell;

    // Set spell to Immolate and recalculate max hit
    player.set_spell(Spell::Special(SpecialSpell::Immolate));
    calc_player_magic_rolls(player, monster);

    let mut info = AttackInfo::new(player, monster);

    // Boost accuracy by 50%
    info.max_att_roll = info.max_att_roll * 3 / 2;

    let mut hit = base_attack(&info, rng);

    if hit.success {
        hit.apply_transforms(monster, rng, limiter);
    }

    // Restore previous spell and recalculate max hit
    if let Some(spell) = previous_spell {
        player.set_spell(spell);
    } else {
        player.attrs.spell = None;
    }

    calc_player_magic_rolls(player, monster);

    hit
}

pub fn abyssal_dagger_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Decreases max hit by 15% but boosts accuracy by 25%
    info.max_hit = info.max_hit * 85 / 100;
    info.max_att_roll = info.max_att_roll * 5 / 4;

    // Rolls against stab
    info.max_def_roll = monster.def_rolls[&CombatType::Stab];

    let mut hit = base_attack(&info, rng);
    if hit.success {
        hit.apply_transforms(monster, rng, limiter);

        // Only one accuracy roll, so if the first hit succeeds, the second hit is always accurate
        let mut hit2 = Hit::accurate(damage_roll(info.min_hit, info.max_hit, rng));
        hit2.apply_transforms(monster, rng, limiter);
        hit = hit.combine(&hit2);
    }

    hit
}

pub fn dark_bow_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Increase max hit by 50% if using dragon arrows and 30% otherwise
    let damage_mod = if player.is_wearing("Dragon arrow", None) {
        15
    } else {
        13
    };

    info.max_hit = info.max_hit * damage_mod / 10;

    // Clamp minimum hit to 8 if using dragon arrows and 5 otherwise
    let clamp_min = if player.is_wearing("Dragon arrow", None) {
        8
    } else {
        5
    };

    // Clamp max hit to 48
    let clamp_max = 48;

    let mut hit = base_attack(&info, rng);
    if hit.success {
        hit.damage = clamp(hit.damage, clamp_min, clamp_max);
        hit.apply_transforms(monster, rng, limiter);
    }
    let mut hit2 = base_attack(&info, rng);
    if hit2.success {
        hit2.damage = clamp(hit2.damage, clamp_min, clamp_max);
        hit2.apply_transforms(monster, rng, limiter);
    }
    hit = hit.combine(&hit2);

    hit
}

pub fn dragon_claw_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Rolls against slash
    info.max_def_roll = monster.def_rolls[&CombatType::Slash];

    // Up to four accuracy rolls are performed, with the first successful one determining damage rolls

    // First accuracy roll
    if base_attack(&info, rng).success {
        // 4-2-1-1: Second hit is half of first hit, third hit is half of second hit, fourth hit is equal to third hit + 1
        let mut hit1 = Hit::accurate(damage_roll(info.max_hit / 2, info.max_hit - 1, rng));
        let mut hit2 = Hit::accurate(hit1.damage / 2);
        let mut hit3 = Hit::accurate(hit2.damage / 2);
        let mut hit4 = Hit::accurate(hit3.damage + 1);

        hit1.apply_transforms(monster, rng, limiter);
        hit2.apply_transforms(monster, rng, limiter);
        hit3.apply_transforms(monster, rng, limiter);
        hit4.apply_transforms(monster, rng, limiter);

        return hit1.combine(&hit2).combine(&hit3).combine(&hit4);
    }

    // Second accuracy roll
    if base_attack(&info, rng).success {
        // 0-4-2-2: First hit misses, third is half of second hit, fourth is third hit + 1
        let mut hit1 = Hit::accurate(damage_roll(info.max_hit * 3 / 8, info.max_hit * 7 / 8, rng));
        let mut hit2 = Hit::accurate(hit1.damage / 2);
        let mut hit3 = Hit::accurate(hit2.damage + 1);

        hit1.apply_transforms(monster, rng, limiter);
        hit2.apply_transforms(monster, rng, limiter);
        hit3.apply_transforms(monster, rng, limiter);

        return hit1.combine(&hit2).combine(&hit3);
    }

    // Third accuracy roll
    if base_attack(&info, rng).success {
        // 0-0-3-3: First and second hit miss, fourth hit is equal to third hit + 1
        let mut hit1 = Hit::accurate(damage_roll(info.max_hit / 4, info.max_hit * 3 / 4, rng));
        let mut hit2 = Hit::accurate(hit1.damage + 1);

        hit1.apply_transforms(monster, rng, limiter);
        hit2.apply_transforms(monster, rng, limiter);

        return hit1.combine(&hit2);
    }

    // Fourth accuracy roll
    if base_attack(&info, rng).success {
        // 0-0-0-5: First three hits miss, fourth rolls between 25-125% of max hit
        return Hit::accurate(damage_roll(info.max_hit / 4, info.max_hit * 5 / 4, rng));
    }

    // If all accuracy rolls fail
    if rng.gen_range(0..2) == 0 {
        // 50% chance of 0-0-1-1
        let mut hit = Hit::accurate(2);
        hit.apply_transforms(monster, rng, limiter);

        hit
    } else {
        // 50% chance of 0-0-0-0
        Hit::inaccurate()
    }
}

pub fn dragon_dagger_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost accuracy and max hit by 15%
    info.max_hit = info.max_hit * 115 / 100;
    info.max_att_roll = info.max_att_roll * 115 / 100;

    // Rolls against slash
    info.max_def_roll = monster.def_rolls[&CombatType::Slash];

    // Rolls two independent hits
    let mut hit1 = base_attack(&info, rng);
    let mut hit2 = base_attack(&info, rng);

    hit1.apply_transforms(monster, rng, limiter);
    hit2.apply_transforms(monster, rng, limiter);

    hit1.combine(&hit2)
}

pub fn dragon_knife_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let info = AttackInfo::new(player, monster);

    // Rolls two independent hits with no boosts
    let mut hit1 = base_attack(&info, rng);
    let mut hit2 = base_attack(&info, rng);

    hit1.apply_transforms(monster, rng, limiter);
    hit2.apply_transforms(monster, rng, limiter);

    hit1.combine(&hit2)
}

pub fn magic_shortbow_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    // Uses same max hit as seercull spec and magic longbow
    let max_hit = player.seercull_spec_max();

    // Boost accuracy by 43%
    let mut info = AttackInfo::new(player, monster);
    info.max_att_roll = info.max_att_roll * 10 / 7;
    info.max_hit = max_hit;

    // Rolls two independent hits
    let mut hit1 = base_attack(&info, rng);
    let mut hit2 = base_attack(&info, rng);

    hit1.apply_transforms(monster, rng, limiter);
    hit2.apply_transforms(monster, rng, limiter);

    hit1.combine(&hit2)
}

pub fn sara_sword_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Boost max hit by 15%
    info.max_hit = info.max_hit * 115 / 100;

    // Rolls against magic but uses player's slash accuracy
    info.max_def_roll = monster.def_rolls[&CombatType::Magic];
    info.max_att_roll = player.att_rolls[&CombatType::Slash];

    let mut hit = base_attack(&info, rng);

    if hit.success {
        // Add a random amount between 1 and 16 to damage
        hit.damage += rng.gen_range(1..=16);
        hit.apply_transforms(monster, rng, limiter);
    }

    hit
}

pub fn zgs_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Double accuracy and increase max hit by 10%
    info.max_att_roll *= 2;
    info.max_hit = info.max_hit * 11 / 10;

    // Rolls against slash
    info.max_def_roll = monster.def_rolls[&CombatType::Slash];

    let mut hit = base_attack(&info, rng);

    if hit.success {
        hit.apply_transforms(monster, rng, limiter);

        // If the monster is freezable, freeze it for 32 ticks (minus freeze resistance)
        if monster.is_freezable() {
            monster.freeze(32);
        }
    }

    hit
}

pub fn ursine_chainmace_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Double accuracy
    info.max_att_roll *= 2;

    let mut hit = base_attack(&info, rng);

    if hit.success {
        hit.apply_transforms(monster, rng, limiter);

        // Apply a DoT effect for 20 damage over 10 ticks (4 every 2 ticks)
        monster.active_effects.push(CombatEffect::DamageOverTime {
            tick_counter: Some(0),
            tick_interval: 2,
            damage_per_hit: 4,
            total_hits: 5,
            apply_on_hit: false,
        })
    }

    hit
}

pub fn soulreaper_axe_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    // Store the number of soulreaper stacks for later
    let current_stacks = player.boosts.soulreaper_stacks;

    // Reset the number of stacks and recalculate rolls
    player.boosts.soulreaper_stacks = 0;
    calc_player_melee_rolls(player, monster);

    let mut info = AttackInfo::new(player, monster);

    // Increase max hit and accuracy by 6% per stack
    info.max_hit = info.max_hit * (100 + 6 * current_stacks) / 100;
    info.max_att_roll = info.max_att_roll * (100 + 6 * current_stacks as i32) / 100;

    let mut hit = base_attack(&info, rng);

    if hit.success {
        hit.apply_transforms(monster, rng, limiter);
    }

    // Restore HP lost while accumulating the stacks (8 per stack)
    player.heal(current_stacks * 8, None);

    hit
}

pub fn tonalztics_of_ralos_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    let mut info = AttackInfo::new(player, monster);

    // Rolls up to 3/4 of the "true" max hit for each hit
    info.max_hit = info.max_hit * 3 / 4;

    let drain_cap = Some(monster.stats.defence / 2);
    let drain_amount = monster.live_stats.magic / 10;

    let mut hit1 = base_attack(&info, rng);
    if hit1.success {
        hit1.apply_transforms(monster, rng, limiter);
        monster.drain_stat(CombatStat::Defence, drain_amount, drain_cap);
    }
    if player.gear.weapon.matches_version("Charged") {
        // Only the charged version does a second attack
        let mut hit2 = base_attack(&info, rng);
        if hit2.success {
            hit2.apply_transforms(monster, rng, limiter);
            monster.drain_stat(CombatStat::Defence, drain_amount, drain_cap);
        }
        return hit1.combine(&hit2);
    }

    hit1
}

pub fn dual_macuahuitl_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    // Only works if the player has full blood moon equipped
    if !player.set_effects.full_blood_moon {
        return (player.attack)(player, monster, rng, limiter);
    }

    let mut info1 = AttackInfo::new(player, monster);
    let mut info2 = info1.clone();

    // Boost max hit and min hit by 25%
    let max_hit = info1.max_hit * 5 / 4;
    let min_hit = info1.max_hit / 4;
    info1.max_hit = max_hit / 2;
    info2.max_hit = max_hit - max_hit / 2;
    info1.min_hit = min_hit / 2;
    info2.min_hit = min_hit - min_hit / 2;

    // Take damage equal to 25% of current HP
    let damage = player.live_stats.hitpoints / 4;
    player.take_damage(damage);

    // Roll two separate hits
    let mut hit1 = base_attack(&info1, rng);
    if hit1.success {
        hit1.apply_transforms(monster, rng, limiter);
    }
    let mut hit2 = if hit1.success {
        // Only roll the second hit if the first hit was accurate
        base_attack(&info2, rng)
    } else {
        Hit::inaccurate()
    };

    if hit2.success {
        hit2.apply_transforms(monster, rng, limiter);
    }

    // Next attack is guaranteed to be 3 ticks
    player.gear.weapon.speed = 3;

    hit1.combine(&hit2)
}

pub fn atlatl_spec(
    player: &mut Player,
    monster: &mut Monster,
    rng: &mut ThreadRng,
    limiter: &Option<Box<dyn Limiter>>,
) -> Hit {
    // Only works if the player has full eclipse moon equipped
    if !player.set_effects.full_eclipse_moon {
        return (player.attack)(player, monster, rng, limiter);
    }

    let mut stack_damage = 0;

    for effect in monster.active_effects.iter() {
        match effect {
            CombatEffect::Burn { stacks, .. } => {
                stack_damage += stacks.iter().sum::<u32>();
                break;
            }
            _ => continue,
        }
    }

    let mut info = AttackInfo::new(player, monster);
    info.max_hit += stack_damage;
    info.min_hit = stack_damage / 2;
    info.max_att_roll = info.max_att_roll * 3 / 2;

    let mut hit = base_attack(&info, rng);
    if hit.success {
        hit.apply_transforms(monster, rng, limiter);
    }

    hit
}
