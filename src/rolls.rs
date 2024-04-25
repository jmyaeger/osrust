use crate::constants::*;
use crate::equipment::{CombatStance, CombatType};
use crate::monster::Monster;
use crate::player::Player;
use crate::utils::Fraction;
use std::cmp::{max, min};
use std::collections::HashMap;

pub fn monster_def_rolls(monster: &Monster) -> HashMap<CombatType, i32> {
    let mut def_rolls = HashMap::new();
    for combat_type in &[
        (CombatType::Stab, monster.bonuses.defence.stab),
        (CombatType::Slash, monster.bonuses.defence.slash),
        (CombatType::Crush, monster.bonuses.defence.crush),
        (CombatType::Ranged, monster.bonuses.defence.ranged),
    ] {
        def_rolls.insert(
            combat_type.0,
            calc_roll(9 + monster.live_stats.defence as i32, combat_type.1),
        );
    }
    if !MAGIC_DEF_EXCEPTIONS.contains(&monster.info.name.as_str()) {
        def_rolls.insert(
            CombatType::Magic,
            calc_roll(
                9 + monster.live_stats.magic as i32,
                monster.bonuses.defence.magic,
            ),
        );
    } else {
        def_rolls.insert(
            CombatType::Magic,
            calc_roll(
                9 + monster.live_stats.defence as i32,
                monster.bonuses.defence.magic,
            ),
        );
    }
    def_rolls
}

pub fn calc_player_def_rolls(player: &mut Player) {
    let mut def_rolls = HashMap::new();
    let stance_bonus = match player.gear.weapon.combat_styles[&player.attrs.active_style].stance {
        CombatStance::Defensive | CombatStance::Longrange => 11,
        CombatStance::Controlled => 9,
        _ => 8,
    };
    let effective_level = player.live_stats.defence * player.prayers.defence + stance_bonus;
    let effective_magic = player.live_stats.magic * player.prayers.magic;
    for combat_type in &[
        (CombatType::Stab, player.bonuses.defence.stab),
        (CombatType::Slash, player.bonuses.defence.slash),
        (CombatType::Crush, player.bonuses.defence.crush),
        (CombatType::Ranged, player.bonuses.defence.ranged),
    ] {
        def_rolls.insert(
            combat_type.0,
            calc_roll(effective_level as i32, combat_type.1),
        );
    }
    def_rolls.insert(
        CombatType::Magic,
        calc_roll(
            (effective_magic * 7 / 10 + effective_level * 3 / 10) as i32,
            1,
        ),
    );
    player.def_rolls = def_rolls;
}

fn calc_roll(eff_lvl: i32, bonus: i32) -> i32 {
    eff_lvl * (bonus + 64)
}

fn calc_max_hit(eff_lvl: u16, bonus: u16) -> u16 {
    (eff_lvl * (bonus + 64) + 320) / 640
}

pub fn calc_all_player_rolls(player: &mut Player, monster: &Monster) {
    calc_player_melee_rolls(player, monster);
    calc_player_ranged_rolls(player, monster);
    calc_player_magic_rolls(player, monster);
    calc_player_def_rolls(player);
}

pub fn calc_player_melee_rolls(player: &mut Player, monster: &Monster) {
    let (eff_att, eff_str) = calc_eff_melee_lvls(player);

    let gear_bonus = melee_gear_bonus(player, monster);
    let inquisitor_boost = inquisitor_boost(player);
    let obsidian_boost = obsidian_boost(player);

    let base_max_hit = calc_max_hit(eff_str, player.bonuses.strength.melee as u16);
    let scaled_max_hit =
        gear_bonus.multiply_to_int(base_max_hit) + obsidian_boost.multiply_to_int(base_max_hit);

    let mut att_rolls = HashMap::new();
    let mut max_hits = HashMap::new();

    let combat_types = [
        (CombatType::Stab, player.bonuses.attack.stab),
        (CombatType::Slash, player.bonuses.attack.slash),
        (CombatType::Crush, player.bonuses.attack.crush),
    ];

    for &(combat_type, bonus) in &combat_types {
        let mut att_roll = calc_roll(eff_att as i32, bonus);
        att_roll = gear_bonus.multiply_to_int(att_roll);
        let mut max_hit = scaled_max_hit;

        if monster.is_dragon() && player.is_wearing("Dragon hunter lance") {
            att_roll = att_roll * 6 / 5;
            max_hit = max_hit * 6 / 5;
        }

        att_rolls.insert(combat_type, att_roll);
        max_hits.insert(combat_type, max_hit);
    }
    att_rolls.insert(
        CombatType::Crush,
        att_rolls[&CombatType::Crush] * inquisitor_boost / 1000,
    );
    max_hits.insert(
        CombatType::Crush,
        max_hits[&CombatType::Crush] * inquisitor_boost as u16 / 1000,
    );

    player.att_rolls = att_rolls;
    player.max_hits = max_hits;
}

pub fn calc_player_ranged_rolls(player: &mut Player, monster: &Monster) {
    let (eff_att, eff_str) = calc_eff_ranged_lvls(player);

    let crystal_bonus = crystal_bonus(player);
    let mut att_gear_bonus = ranged_gear_bonus(player, monster);
    let mut str_gear_bonus = att_gear_bonus;
    let str_bonus = if player.is_wearing("Eclipse atlatl") {
        player.bonuses.strength.melee
    } else {
        player.bonuses.strength.ranged
    };

    let mut att_roll = calc_roll(eff_att as i32, player.bonuses.attack.ranged);
    att_roll = att_roll * (1000 + 2 * crystal_bonus) / 1000;

    let mut max_hit = calc_max_hit(eff_str, str_bonus as u16);
    max_hit = max_hit * (1000 + crystal_bonus as u16) / 1000;

    if monster.is_dragon() && player.is_wearing("Dragon hunter crossbow") {
        att_gear_bonus += Fraction::new(3, 10);
        str_gear_bonus += Fraction::new(1, 4);
    }

    att_roll = att_gear_bonus.multiply_to_int(att_roll);
    max_hit = str_gear_bonus.multiply_to_int(max_hit);

    if player.is_wearing("Twisted bow") {
        let (tbow_acc_bonus, tbow_dmg_bonus) = monster.tbow_bonuses();
        att_roll = att_roll * tbow_acc_bonus / 100;
        max_hit = max_hit * tbow_dmg_bonus as u16 / 100;
    }
    player.att_rolls.insert(CombatType::Ranged, att_roll);
    player.max_hits.insert(CombatType::Ranged, max_hit);
}

pub fn calc_player_magic_rolls(player: &mut Player, monster: &Monster) {
    let base_max_hit = get_base_magic_hit(player);
    let mut magic_attack = player.bonuses.attack.magic;
    let mut magic_damage = 2 * player.bonuses.strength.magic as i32;

    if player.is_wearing("Tumeken's shadow") && player.combat_stance() != CombatStance::ManualCast {
        (magic_attack, magic_damage) = apply_shadow_boost(magic_attack, magic_damage, monster);
    }

    let eff_lvl = calc_eff_magic_lvl(player);
    let mut att_roll = eff_lvl as i32 * (magic_attack + 64);

    (att_roll, magic_damage) = apply_smoke_staff_bonus(att_roll, magic_damage, player);
    magic_damage = apply_virtus_bonus(magic_damage, player);

    let (att_roll, magic_damage, salve_active) =
        apply_salve_magic_boost(att_roll, magic_damage, player, monster);

    let mut max_hit = base_max_hit * (200 + magic_damage as u16) / 200;

    let (mut att_roll, wilderness_boost) = apply_wildy_staff_boost(att_roll, player, monster);

    if player.is_wearing("Tome of water") && player.is_using_water_spell() {
        att_roll = att_roll * 6 / 5;
    }

    let mut slayer_boost = 0u16;
    if !salve_active && player.is_wearing_imbued_black_mask() && player.boosts.on_task {
        att_roll = att_roll * 115 / 100;
        slayer_boost = 15u16;
    }

    max_hit = max_hit * (100 + slayer_boost + wilderness_boost) / 100;
    if player.is_wearing("Tome of fire") && player.is_using_fire_spell() {
        max_hit = max_hit * 3 / 2;
    } else if player.is_wearing("Tome of water") && player.is_using_water_spell() {
        max_hit = max_hit * 6 / 5;
    }

    player.att_rolls.insert(CombatType::Magic, att_roll);
    player.max_hits.insert(CombatType::Magic, max_hit);
}

fn calc_eff_melee_lvls(player: &Player) -> (u16, u16) {
    let att_stance_bonus = match player.combat_stance() {
        CombatStance::Accurate => 11,
        CombatStance::Controlled => 9,
        _ => 8,
    };
    let str_stance_bonus = match player.combat_stance() {
        CombatStance::Aggressive => 11,
        CombatStance::Controlled => 9,
        _ => 8,
    };
    let att_pray_boost = player.prayers.attack;
    let str_pray_boost = player.prayers.strength;
    let soulreaper_boost = player.boosts.soulreaper_stacks * player.live_stats.strength * 6 / 100;
    let mut eff_att = player.live_stats.attack * (100 + att_pray_boost) / 100 + att_stance_bonus;
    let mut eff_str = player.live_stats.strength * (100 + str_pray_boost) / 100
        + soulreaper_boost
        + str_stance_bonus;
    if player.set_effects.full_void | player.set_effects.full_elite_void {
        eff_att = eff_att * 11 / 10;
        eff_str = eff_str * 11 / 10;
    }
    (eff_att, eff_str)
}

fn calc_eff_ranged_lvls(player: &Player) -> (u16, u16) {
    let stance_bonus = match player.combat_stance() {
        CombatStance::Accurate => 11,
        _ => 8,
    };

    let range_att_pray_boost = player.prayers.ranged_att;
    let range_str_pray_boost = player.prayers.ranged_str;
    let str_level = if player.is_wearing("Eclipse atlatl") {
        player.live_stats.strength
    } else {
        player.live_stats.ranged
    };

    let mut eff_att = player.live_stats.ranged * (100 + range_att_pray_boost) / 100 + stance_bonus;
    let mut eff_str = str_level * (100 + range_str_pray_boost) / 100 + stance_bonus;

    if player.set_effects.full_elite_void {
        eff_att = eff_att * 11 / 10;
        eff_str = eff_str * 1125 / 1000;
    } else if player.set_effects.full_void {
        eff_att = eff_att * 11 / 10;
        eff_str = eff_str * 11 / 10;
    }
    (eff_att, eff_str)
}

fn melee_gear_bonus(player: &Player, monster: &Monster) -> Fraction {
    if player.is_wearing("Amulet of avarice") && monster.is_revenant() {
        if player.boosts.forinthry_surge {
            Fraction::new(135, 100)
        } else {
            Fraction::new(6, 5)
        }
    } else if monster.is_undead() && player.is_wearing_salve() {
        Fraction::new(7, 6)
    } else if monster.is_undead() && player.is_wearing_salve_e() {
        Fraction::new(6, 5)
    } else if player.boosts.on_task && player.is_wearing_black_mask() {
        Fraction::new(7, 6)
    } else {
        Fraction::from_integer(1)
    }
}

fn obsidian_boost(player: &Player) -> Fraction {
    if player.set_effects.full_obsidian
        && player.is_wearing_tzhaar_weapon()
        && player.is_using_melee()
    {
        Fraction::new(1, 10)
    } else {
        Fraction::from_integer(0)
    }
}

fn inquisitor_boost(player: &Player) -> i32 {
    let inquisitor_pieces = [&player.gear.head, &player.gear.body, &player.gear.legs]
        .iter()
        .filter_map(|slot| slot.as_ref())
        .filter(|armor| armor.name.contains("Inquisitor"))
        .count();

    let boost = if player.set_effects.full_inquisitor {
        25
    } else {
        5 * inquisitor_pieces as i32
    };

    1000 + boost
}

fn crystal_bonus(player: &Player) -> i32 {
    let mut crystal_bonus = 0;
    if player.is_wearing_crystal_bow() {
        if player.is_wearing("Crystal helm") {
            crystal_bonus += 25;
        }
        if player.is_wearing("Crystal body") {
            crystal_bonus += 75;
        }
        if player.is_wearing("Crystal legs") {
            crystal_bonus += 50;
        }
    }
    crystal_bonus
}

fn ranged_gear_bonus(player: &Player, monster: &Monster) -> Fraction {
    if player.is_wearing("Eclipse atlatl") {
        return melee_gear_bonus(player, monster);
    }
    let mut gear_bonus = Fraction::from_integer(1);
    if player.is_wearing("Amulet of avarice") && monster.is_revenant() {
        if player.boosts.forinthry_surge {
            gear_bonus = Fraction::new(135, 100)
        } else {
            gear_bonus = Fraction::new(6, 5)
        }
    } else if monster.is_undead() && player.is_wearing("Salve amulet (ei)") {
        gear_bonus = Fraction::new(6, 5)
    } else if player.is_wearing("Salve amulet (i)") {
        gear_bonus = Fraction::new(7, 6)
    } else if player.boosts.on_task && player.is_wearing_imbued_black_mask() {
        gear_bonus = Fraction::new(115, 100);
        if (player.boosts.in_wilderness || monster.is_in_wilderness())
            && player.is_wearing_wildy_bow()
        {
            gear_bonus += Fraction::new(1, 2);
        }
    }
    gear_bonus
}

fn get_base_magic_hit(player: &Player) -> u16 {
    if let Some(spell) = &player.attrs.spell {
        spell.max_hit(player)
    } else if player.is_wearing_salamander() {
        salamander_max_hit(player)
    } else {
        charged_staff_max_hit(player)
    }
}

fn salamander_max_hit(player: &Player) -> u16 {
    let factor = match player.gear.weapon.name.as_str() {
        "Swamp lizard" => 120,
        "Orange salamander" => 123,
        "Red salamander" => 141,
        "Black salamander" => 156,
        "Tecu salamander" => 168,
        _ => panic!("Unimplemented salamander: {}", player.gear.weapon.name),
    };
    (1 + 2 * player.live_stats.magic * factor) / 1280
}

fn charged_staff_max_hit(player: &Player) -> u16 {
    let visible_magic = player.live_stats.magic;
    match player.gear.weapon.name.as_str() {
        "Starter staff" => 8,
        "Warped sceptre" => (8 * visible_magic + 96) / 37,
        "Trident of the seas" | "Trident of the seas (e)" => max(1, visible_magic / 3 - 5),
        "Thammaron's sceptre" => max(1, visible_magic / 3 - 8),
        "Accursed sceptre" => max(1, visible_magic / 3 - 6),
        "Trident of the swamp" | "Trident of the swamp (e)" => max(1, visible_magic / 3 - 2),
        "Sanguinesti staff" => max(1, visible_magic / 3 - 1),
        "Dawnbringer" => visible_magic / 6 - 1,
        "Tumeken's shadow" => visible_magic / 3 + 1,
        "Bone staff" => max(1, visible_magic / 3 - 5) + 10,
        "Crystal staff (basic)" | "Corrupted staff (basic)" => 23,
        "Crystal staff (attuned)" | "Corrupted staff (attuned)" => 31,
        "Crystal staff (perfected)" | "Corrupted staff (perfected)" => 39,
        _ => panic!(
            "Magic max hit could not be determined for {}",
            player.gear.weapon.name
        ),
    }
}

fn apply_shadow_boost(magic_attack: i32, magic_damage: i32, monster: &Monster) -> (i32, i32) {
    let multiplier = if monster.is_toa_monster() { 4 } else { 3 };
    let magic_attack = magic_attack * multiplier;
    let magic_damage = min(200, magic_damage * multiplier);
    (magic_attack, magic_damage)
}

fn calc_eff_magic_lvl(player: &Player) -> u16 {
    let stance_bonus = if player.combat_stance() == CombatStance::Accurate {
        11
    } else {
        9
    };
    let magic_pray_boost = player.prayers.magic;
    let void_bonus = if player.set_effects.full_void || player.set_effects.full_elite_void {
        Fraction::new(145, 100)
    } else {
        Fraction::from_integer(1)
    };
    let visible_magic = player.live_stats.magic;

    void_bonus.multiply_to_int(visible_magic * (100 + magic_pray_boost) / 100) + stance_bonus
}

fn apply_smoke_staff_bonus(att_roll: i32, magic_damage: i32, player: &Player) -> (i32, i32) {
    let mut att_roll = att_roll;
    let mut magic_damage = magic_damage;
    if player.is_wearing_smoke_staff() && player.is_using_standard_spell() {
        att_roll = att_roll * 11 / 10;
        magic_damage += 20;
    }
    (att_roll, magic_damage)
}

fn apply_virtus_bonus(magic_damage: i32, player: &Player) -> i32 {
    if player.is_using_ancient_spell() {
        magic_damage
            + [
                player.gear.head.as_ref(),
                player.gear.body.as_ref(),
                player.gear.legs.as_ref(),
            ]
            .iter()
            .filter(|slot| slot.is_some() && slot.as_ref().unwrap().name.contains("Virtus"))
            .count() as i32
                * 6
    } else {
        magic_damage
    }
}

fn apply_salve_magic_boost(
    att_roll: i32,
    magic_damage: i32,
    player: &Player,
    monster: &Monster,
) -> (i32, i32, bool) {
    let mut att_roll = att_roll;
    let mut magic_damage = magic_damage;
    let mut salve_active = true;

    if player.is_wearing("Amulet of avarice") && monster.is_revenant() {
        if player.boosts.forinthry_surge {
            att_roll = att_roll * 135 / 100;
            magic_damage += 70;
        } else {
            att_roll = att_roll * 6 / 5;
            magic_damage += 40;
        }
    } else if player.is_wearing("Salve amulet (ei)") && monster.is_undead() {
        att_roll = att_roll * 6 / 5;
        magic_damage += 40;
    } else if player.is_wearing("Salve amulet (i)") {
        att_roll = att_roll * 115 / 100;
        magic_damage += 30;
    } else {
        salve_active = false;
    }

    (att_roll, magic_damage, salve_active)
}

fn apply_wildy_staff_boost(att_roll: i32, player: &Player, monster: &Monster) -> (i32, u16) {
    if (player.boosts.in_wilderness || monster.is_in_wilderness())
        && player.is_wearing_wildy_staff()
    {
        (att_roll * 3 / 2, 50u16)
    } else {
        (att_roll, 0u16)
    }
}
