use crate::attacks::{base_attack, damage_roll, AttackInfo, Hit};
use crate::effects::CombatEffect;
use crate::equipment::CombatType;
use crate::limiters::Limiter;
use crate::monster::Monster;
use crate::player::Player;
use crate::rolls::calc_player_magic_rolls;
use crate::spells::{SpecialSpell, Spell};
use rand::rngs::ThreadRng;
use std::cmp::min;

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
        hit.apply_limiters(rng, limiter);
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
    hit.apply_limiters(rng, limiter);

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
        hit.apply_limiters(rng, limiter);

        // Drains are twice as effective on demons
        let demon_mod = if monster.is_demon() { 2 } else { 1 };

        // Drain stats by 1 + 5% or 10%
        monster.drain_attack(monster.live_stats.attack * demon_mod / 20 + 1);
        monster.drain_strength(monster.live_stats.strength * demon_mod / 20 + 1);
        monster.drain_defence(monster.live_stats.defence * demon_mod / 20 + 1);
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

    let mut hit = base_attack(&info, rng);

    if hit.success {
        // Add delayed attack and heal if the hit is successful
        hit.apply_limiters(rng, limiter);
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
    hit.apply_limiters(rng, limiter);

    // Restore prayer by half the damage, up to 120 prayer points
    player.live_stats.prayer = min(120, player.live_stats.prayer + hit.damage / 2);

    // Restore previous spell and recalculate max hit
    if let Some(spell) = previous_spell {
        player.set_spell(spell);
    } else {
        player.attrs.spell = None;
    }

    calc_player_magic_rolls(player, monster);

    hit
}
