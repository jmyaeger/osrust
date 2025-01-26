use crate::constants::*;
use crate::types::equipment::{CombatStance, CombatStyle, CombatType};
use crate::types::monster::Monster;
use crate::types::player::Player;
use crate::types::prayers::Prayer;
use crate::types::spells::{Spell, StandardSpell};
use crate::utils::math::Fraction;
use std::cmp::{max, min};
use std::collections::HashMap;

pub fn monster_def_rolls(monster: &Monster) -> HashMap<CombatType, i32> {
    let mut def_rolls = HashMap::new();
    for combat_type in &[
        (CombatType::Stab, monster.bonuses.defence.stab),
        (CombatType::Slash, monster.bonuses.defence.slash),
        (CombatType::Crush, monster.bonuses.defence.crush),
        (CombatType::Light, monster.bonuses.defence.light),
        (CombatType::Standard, monster.bonuses.defence.standard),
        (CombatType::Heavy, monster.bonuses.defence.heavy),
        (
            CombatType::Ranged,
            (monster.bonuses.defence.light
                + monster.bonuses.defence.standard
                + monster.bonuses.defence.heavy)
                / 3,
        ),
    ] {
        def_rolls.insert(
            combat_type.0,
            calc_roll(9 + monster.live_stats.defence, combat_type.1),
        );
    }

    // Use magic level for magic defence in most cases
    if !MAGIC_DEF_EXCEPTIONS.contains(&monster.info.id.unwrap_or(0)) {
        def_rolls.insert(
            CombatType::Magic,
            calc_roll(9 + monster.live_stats.magic, monster.bonuses.defence.magic),
        );
    } else {
        // Use defence level in some special cases
        def_rolls.insert(
            CombatType::Magic,
            calc_roll(
                9 + monster.live_stats.defence,
                monster.bonuses.defence.magic,
            ),
        );
    }
    def_rolls
}

pub fn monster_att_rolls(monster: &Monster) -> HashMap<CombatType, i32> {
    let mut att_rolls = HashMap::new();
    let stance_bonus = 9;
    for combat_type in &[
        (
            CombatType::Stab,
            monster.live_stats.attack,
            monster.bonuses.attack.melee,
        ),
        (
            CombatType::Slash,
            monster.live_stats.attack,
            monster.bonuses.attack.melee,
        ),
        (
            CombatType::Crush,
            monster.live_stats.attack,
            monster.bonuses.attack.melee,
        ),
        (
            CombatType::Ranged,
            monster.live_stats.ranged,
            monster.bonuses.attack.ranged,
        ),
        (
            CombatType::Magic,
            monster.live_stats.magic,
            monster.bonuses.attack.magic,
        ),
    ] {
        let effective_level = combat_type.1 + stance_bonus;
        att_rolls.insert(combat_type.0, calc_roll(effective_level, combat_type.2));
    }
    att_rolls
}

pub fn calc_player_def_rolls(player: &mut Player) {
    let mut def_rolls = HashMap::new();

    let stance_bonus = match player.combat_stance() {
        CombatStance::Defensive | CombatStance::Longrange => 11,
        CombatStance::Controlled => 9,
        _ => 8,
    };

    let effective_level = player.live_stats.defence * (100 + player.prayers.defence) / 100;
    let effective_magic = player.live_stats.magic * (100 + player.prayers.magic_att) / 100;

    for combat_type in &[
        (CombatType::Stab, player.bonuses.defence.stab),
        (CombatType::Slash, player.bonuses.defence.slash),
        (CombatType::Crush, player.bonuses.defence.crush),
        (CombatType::Ranged, player.bonuses.defence.ranged),
    ] {
        def_rolls.insert(
            combat_type.0,
            calc_roll(effective_level + stance_bonus, combat_type.1),
        );
    }
    // Magic defence uses 70% magic level, 30% defence level
    def_rolls.insert(
        CombatType::Magic,
        calc_roll(
            effective_magic * 7 / 10 + effective_level * 3 / 10 + stance_bonus,
            player.bonuses.defence.magic,
        ),
    );
    player.def_rolls = def_rolls;
}

fn calc_roll(eff_lvl: u32, bonus: i32) -> i32 {
    max(0, eff_lvl as i32 * (bonus + 64))
}

fn calc_max_hit(eff_lvl: u32, bonus: i32) -> u32 {
    max(0, (eff_lvl as i32 * (bonus + 64) + 320) / 640) as u32
}

pub fn calc_active_player_rolls(player: &mut Player, monster: &Monster) {
    match player.combat_type() {
        CombatType::Stab | CombatType::Slash | CombatType::Crush => {
            calc_player_melee_rolls(player, monster);
        }
        CombatType::Light | CombatType::Standard | CombatType::Heavy => {
            calc_player_ranged_rolls(player, monster);
        }
        CombatType::Magic => {
            calc_player_magic_rolls(player, monster);

            // Calc melee rolls if it's needed for the blue moon effect
            if player.set_effects.full_blue_moon {
                let current_style = player.attrs.active_style.clone();
                player.set_active_style(CombatStyle::Swipe);
                calc_player_melee_rolls(player, monster);
                player.set_active_style(current_style);
            }
        }
        _ => {}
    };
    calc_player_def_rolls(player);
}

pub fn calc_player_melee_rolls(player: &mut Player, monster: &Monster) {
    let (eff_att, eff_str) = calc_eff_melee_lvls(player);

    // Get slayer and salve/avarice boosts
    let gear_bonus = melee_gear_bonus(player, monster);

    // Get inquisitor and obsidian boosts if applicable
    let inquisitor_boost = inquisitor_boost(player);
    let obsidian_boost = obsidian_boost(player);

    let base_max_hit = calc_max_hit(eff_str, player.bonuses.strength.melee);

    // Obsidian bonus is additive based on base max hit (verified in-game)
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
        let base_att_roll = calc_roll(eff_att, bonus);
        let mut att_roll = gear_bonus.multiply_to_int(base_att_roll)
            + obsidian_boost.multiply_to_int(base_att_roll);
        let mut max_hit = scaled_max_hit;

        // Dragon hunter lance, keris, arclight, etc.
        (att_roll, max_hit) = apply_melee_weapon_boosts(att_roll, max_hit, player, monster);

        // Silver weapons against vampyres; non-silver weapons return zeros
        (att_roll, max_hit) = apply_vampyre_boost(att_roll, max_hit, player, monster);

        att_rolls.insert(combat_type, att_roll);
        max_hits.insert(combat_type, max_hit);
    }

    // Apply inquisitor boost last
    att_rolls.insert(
        CombatType::Crush,
        att_rolls[&CombatType::Crush] * inquisitor_boost as i32 / 1000,
    );
    max_hits.insert(
        CombatType::Crush,
        max_hits[&CombatType::Crush] * inquisitor_boost / 1000,
    );

    player.att_rolls = att_rolls;
    player.max_hits = max_hits;
}

pub fn calc_player_ranged_rolls(player: &mut Player, monster: &Monster) {
    // TODO: Implement silver bolts against vampyres

    // Returns melee effective strength for eclipse atlatl
    let (eff_att, eff_str) = calc_eff_ranged_lvls(player);

    // Get crystal bow/armor bonus
    let crystal_bonus = crystal_bonus(player);

    // Get slayer/salve/avarice boosts and DHCB/wildy bow + slayer (which are additive)
    let (att_gear_bonus, str_gear_bonus) = ranged_gear_bonus(player, monster);

    // Eclipse atlatl uses melee strength bonuses
    let str_bonus = if player.is_wearing("Eclipse atlatl", None) {
        player.bonuses.strength.melee
    } else {
        player.bonuses.strength.ranged
    };

    // Crystal bow/armor bonus is applied before slayer, salve, etc.
    let mut att_roll = calc_roll(eff_att, player.bonuses.attack.ranged);
    att_roll = att_roll * (1000 + 2 * crystal_bonus as i32) / 1000;

    let mut max_hit = calc_max_hit(eff_str, str_bonus);
    max_hit = max_hit * (1000 + crystal_bonus) / 1000;

    // Apply slayer, salve, etc.
    att_roll = att_gear_bonus.multiply_to_int(att_roll);
    max_hit = str_gear_bonus.multiply_to_int(max_hit);

    // Apply DHCB (if not on task), twisted bow, etc, if applicable
    (att_roll, max_hit) = apply_ranged_weapon_boosts(att_roll, max_hit, player, monster);

    // TODO: Find out when vampyre boosts are applied
    (att_roll, max_hit) = apply_silver_bolts_bonus(att_roll, max_hit, player, monster);

    for &combat_type in &[CombatType::Light, CombatType::Standard, CombatType::Heavy] {
        player.att_rolls.insert(combat_type, att_roll);
        player.max_hits.insert(combat_type, max_hit);
    }
}

pub fn calc_player_magic_rolls(player: &mut Player, monster: &Monster) {
    // Base max hit of a spell or charged staff/salamander (based on magic level)
    let base_max_hit = get_base_magic_hit(player);

    // Apply chaos gauntlets for bolt spells and Charge for god spells
    let mut max_hit = apply_chaos_gauntlet_boost(base_max_hit, player);
    max_hit = apply_charge_boost(max_hit, player);

    let elemental_weakness = get_elemental_weakness_boost(player, monster);

    // Elemental weakness applies to base max hit before gear bonuses
    let weakness_str_boost = max_hit * elemental_weakness / 100;

    let mut magic_attack = player.bonuses.attack.magic;

    // Multiplied by 10 because it increments by 0.1%
    let mut magic_damage = (10.0 * player.bonuses.strength.magic) as u32;

    // Apply shadow multipliers to attack and damage bonuses, if applicable
    if player.is_wearing("Tumeken's shadow", Some("Charged"))
        && player.combat_stance() != CombatStance::ManualCast
    {
        (magic_attack, magic_damage) = apply_shadow_boost(magic_attack, magic_damage, monster);
    }

    magic_damage += 10 * player.prayers.magic_str;

    let eff_lvl = calc_eff_magic_lvl(player);
    let base_att_roll = calc_roll(eff_lvl, magic_attack);

    // Assuming that accuracy increase is similar to damage in that it uses base roll
    let weakness_att_boost = base_att_roll * elemental_weakness as i32 / 100;

    // Apply virtus robe boost for ancient spells
    magic_damage = apply_virtus_bonus(magic_damage, player);

    // Determine if salve boost is applicable and apply it if so
    // Smoke staff and Efaritay's aid boosts are applied additively here as well (source: Mod Ash)
    let (mut att_roll, magic_damage, salve_active) =
        apply_additive_magic_boosts(base_att_roll, magic_damage, player, monster);

    // "Primary" magic damage
    max_hit = max_hit * (1000 + magic_damage) / 1000;

    // Apply slayer boost only if salve boost is not active
    let mut slayer_boost = 0u32;
    if !salve_active && player.is_wearing_imbued_black_mask() && player.boosts.on_task {
        att_roll = att_roll * 115 / 100;
        slayer_boost = 15u32;
    }

    // Apply wildy staff boost to attack roll and store the damage boost
    let (mut att_roll, wilderness_boost) = apply_wildy_staff_boost(att_roll, player, monster);

    // Apply slayer and wilderness boosts
    max_hit = max_hit * (100 + slayer_boost) / 100;
    max_hit = max_hit * (100 + wilderness_boost) / 100;

    // Apply demonbane/mark of darkness accuracy boost
    if player.is_using_demonbane_spell() && monster.is_demon() {
        let bonus = match (
            player.boosts.mark_of_darkness,
            player.is_wearing("Purging staff", None),
        ) {
            (true, true) => Fraction::new(18, 10),
            (true, false) | (false, true) => Fraction::new(14, 10),
            (false, false) => Fraction::new(12, 10),
        };
        att_roll = bonus.multiply_to_int(att_roll);
    }

    // Apply dragonbane boosts - still works for DHL and DHCB when manual casting
    if monster.is_dragon() {
        if player.is_wearing_any(vec![
            ("Dragon hunter wand", None),
            ("Dragon hunter lance", None),
        ]) {
            max_hit = max_hit * 6 / 5;
        } else if player.is_wearing("Dragon hunter crossbow", None) {
            max_hit = max_hit * 5 / 4;
        }
    }

    // Apply elemental weakness boosts
    att_roll += weakness_att_boost;
    max_hit += weakness_str_boost;

    if player.boosts.sunfire.active && player.is_using_fire_spell() {
        player.boosts.sunfire.min_hit = max_hit / 10;
    }

    // Apply tome of fire/water damage bonuses (which are now pre-roll)
    if (player.is_wearing("Tome of fire", Some("Charged")) && player.is_using_fire_spell())
        || player.is_wearing("Tome of earth", Some("Charged")) && player.is_using_earth_spell()
    {
        max_hit = max_hit * 11 / 10;
    } else if player.is_wearing("Tome of water", Some("Charged")) && player.is_using_water_spell() {
        att_roll = att_roll * 11 / 10; //TODO: Check if this still exists
        max_hit = max_hit * 11 / 10;
    }

    player.att_rolls.insert(CombatType::Magic, att_roll);
    player.max_hits.insert(CombatType::Magic, max_hit);
}

fn calc_eff_melee_lvls(player: &Player) -> (u32, u32) {
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

    let is_using_burst_of_strength =
        player
            .prayers
            .active_prayers
            .as_ref()
            .map_or(false, |prayers| {
                prayers
                    .iter()
                    .any(|p| p.prayer_type == Prayer::BurstOfStrength)
            });

    // Soulreaper stacks boost effective strength level additively
    let mut eff_str = if is_using_burst_of_strength && player.live_stats.strength <= 20 {
        player.live_stats.strength + 1 + soulreaper_boost + str_stance_bonus
    } else {
        player.live_stats.strength * (100 + str_pray_boost) / 100
            + soulreaper_boost
            + str_stance_bonus
    };

    // Apply void set bonuses
    if player.set_effects.full_void | player.set_effects.full_elite_void {
        eff_att = eff_att * 11 / 10;
        eff_str = eff_str * 11 / 10;
    }

    (eff_att, eff_str)
}

fn calc_eff_ranged_lvls(player: &Player) -> (u32, u32) {
    let stance_bonus = match player.combat_stance() {
        CombatStance::Accurate => 11,
        _ => 8,
    };

    let range_att_pray_boost = player.prayers.ranged_att;
    let range_str_pray_boost = player.prayers.ranged_str;

    // Eclipse atlatl uses visible melee strength level for ranged strength
    let str_level = if player.is_wearing("Eclipse atlatl", None) {
        player.live_stats.strength
    } else {
        player.live_stats.ranged
    };

    let mut eff_att = player.live_stats.ranged * (100 + range_att_pray_boost) / 100 + stance_bonus;

    let is_using_sharp_eye = player
        .prayers
        .active_prayers
        .as_ref()
        .map_or(false, |prayers| {
            prayers.iter().any(|p| p.prayer_type == Prayer::SharpEye)
        });

    let mut eff_str = if is_using_sharp_eye && str_level <= 20 {
        str_level + 1 + stance_bonus
    } else {
        str_level * (100 + range_str_pray_boost) / 100 + stance_bonus
    };

    // Apply void set bonuses
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
    // Avarice, salve, and slayer bonuses

    if player.is_wearing("Amulet of avarice", None) && monster.is_revenant() {
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
        Fraction::new(1, 1)
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

fn apply_vampyre_boost(
    att_roll: i32,
    max_hit: u32,
    player: &Player,
    monster: &Monster,
) -> (i32, u32) {
    if let Some(tier) = monster.vampyre_tier() {
        if (1..=3).contains(&tier) {
            let (mut att_factor, max_hit_factor) = match (
                player.gear.weapon.name.as_str(),
                player.is_wearing_silver_weapon(),
                player.is_wearing("Efaritay's aid", None),
                tier,
            ) {
                ("Blisterwood flail" | "Blisterwood sickle", _, _, _) => {
                    (Fraction::new(105, 100), Fraction::new(1, 1))
                }
                ("Ivandis flail", _, _, _) => (Fraction::new(1, 1), Fraction::new(1, 1)),
                // Any other weapon against tier 3 or any non-silver weapon against tier 2 will return (0, 0)
                (_, _, _, 2 | 3) => (Fraction::new(0, 1), Fraction::new(0, 1)),
                _ => (Fraction::new(1, 1), Fraction::new(1, 1)),
            };

            // Efaritay's aid now gives silver weapons a 15% accuracy boost
            if player.is_wearing_silver_weapon() && player.is_wearing("Efaritay's aid", None) {
                att_factor *= Fraction::new(115, 100);
            }

            return (
                att_factor.multiply_to_int(att_roll),
                max_hit_factor.multiply_to_int(max_hit),
            );
        }
    }

    (att_roll, max_hit)
}

fn apply_melee_weapon_boosts(
    att_roll: i32,
    max_hit: u32,
    player: &Player,
    monster: &Monster,
) -> (i32, u32) {
    let mut att_roll = att_roll;
    let mut max_hit = max_hit;

    let (mut att_factor, mut max_hit_factor) = match player.gear.weapon.name.as_str() {
        "Dragon hunter lance" if monster.is_dragon() => (Fraction::new(6, 5), Fraction::new(6, 5)),
        "Dragon hunter wand" if monster.is_dragon() => (Fraction::new(3, 2), Fraction::new(6, 5)),
        "Keris partisan of breaching" if monster.is_kalphite() => {
            (Fraction::new(133, 100), Fraction::new(133, 100))
        }
        // Other keris variants against kalphites
        _ if monster.is_kalphite() && player.is_wearing_keris() => {
            (Fraction::new(1, 1), Fraction::new(133, 100))
        }
        "Barronite mace" if monster.is_golem() => (Fraction::new(1, 1), Fraction::new(115, 100)),
        // Wildy mace against wildy monsters
        _ if (monster.is_in_wilderness() || player.boosts.in_wilderness)
            && player.is_wearing_wildy_mace() =>
        {
            (Fraction::new(3, 2), Fraction::new(3, 2))
        }
        "Silverlight" | "Darklight" if monster.is_demon() => {
            (Fraction::new(0, 1), Fraction::new(3, 5))
        }
        "Arclight" | "Emberlight" if monster.is_demon() => {
            (Fraction::new(7, 10), Fraction::new(7, 10))
        }
        "Burning claws" if monster.is_demon() => (Fraction::new(1, 20), Fraction::new(1, 20)),
        "Leaf-bladed battleaxe" if monster.is_leafy() => {
            (Fraction::new(1, 1), Fraction::new(47, 40))
        }
        _ => (Fraction::new(1, 1), Fraction::new(1, 1)),
    };

    if player.is_wearing_any(vec![
        ("Silverlight", Some("Dyed")),
        ("Silverlight", None),
        ("Darklight", None),
        ("Arclight", Some("Charged")),
        ("Emberlight", None),
    ]) && monster.is_demon()
    {
        if monster.info.name.contains("Duke Sucellus") {
            att_factor *= Fraction::new(7, 10);
            max_hit_factor *= Fraction::new(7, 10);
        }
        att_factor += Fraction::new(1, 1);
        max_hit_factor += Fraction::new(1, 1);
    }

    att_roll = att_factor.multiply_to_int(att_roll);
    max_hit = max_hit_factor.multiply_to_int(max_hit);

    // Apply colossal blade and bone mace boosts additively
    match player.gear.weapon.name.as_str() {
        "Colossal blade" => {
            max_hit += 2 * min(monster.info.size, 5);
        }
        "Bone mace" => {
            if monster.is_rat() {
                max_hit += 10
            }
        }
        _ => {}
    }

    (att_roll, max_hit)
}

fn inquisitor_boost(player: &Player) -> u32 {
    let mut inquisitor_pieces = [&player.gear.head, &player.gear.body, &player.gear.legs]
        .iter()
        .filter_map(|slot| slot.as_ref())
        .filter(|armor| armor.name.contains("Inquisitor"))
        .count();

    if inquisitor_pieces > 0 {
        if player.is_wearing("Inquisitor's mace", None) {
            inquisitor_pieces *= 5;
        } else if inquisitor_pieces == 3 {
            inquisitor_pieces += 2;
        }
    }

    1000 + 5 * inquisitor_pieces as u32
}

fn crystal_bonus(player: &Player) -> u32 {
    player
        .is_wearing_crystal_bow()
        .then(|| {
            [
                ("Crystal helm", 25),
                ("Crystal body", 75),
                ("Crystal legs", 50),
            ]
            .iter()
            .filter(|(item, _)| player.is_wearing(item, Some("Active")))
            .map(|(_, bonus)| bonus)
            .sum()
        })
        .unwrap_or(0)
}

fn ranged_gear_bonus(player: &Player, monster: &Monster) -> (Fraction, Fraction) {
    // Eclipse atlatl uses melee slayer and salve boosts (TODO: check if true for non-imbued salve)

    let mut att_gear_bonus = Fraction::new(1, 1);
    let mut str_gear_bonus = Fraction::new(1, 1);

    if player.is_wearing("Amulet of avarice", None) && monster.is_revenant() {
        if player.boosts.forinthry_surge {
            att_gear_bonus = Fraction::new(135, 100);
            str_gear_bonus = Fraction::new(135, 100);
        } else {
            att_gear_bonus = Fraction::new(6, 5);
            str_gear_bonus = Fraction::new(6, 5);
        }
    } else if monster.is_undead() && player.is_wearing("Salve amulet(ei)", None) {
        att_gear_bonus = Fraction::new(6, 5);
        str_gear_bonus = Fraction::new(6, 5);
    } else if player.is_wearing("Salve amulet(i)", None) {
        att_gear_bonus = Fraction::new(7, 6);
        str_gear_bonus = Fraction::new(7, 6);
    } else if player.boosts.on_task && player.is_wearing_imbued_black_mask() {
        att_gear_bonus = Fraction::new(115, 100);
        str_gear_bonus = Fraction::new(115, 100);
        if (player.boosts.in_wilderness || monster.is_in_wilderness())
            && player.is_wearing_wildy_bow()
        {
            // Wildy bow boost is applied additively with slayer helm (verified in-game)
            att_gear_bonus += Fraction::new(1, 2);
            str_gear_bonus += Fraction::new(1, 2);
        } else if player.is_wearing("Dragon hunter crossbow", None) && monster.is_dragon() {
            // DHCB boost is applied additively with slayer helm (verified in-game)
            att_gear_bonus += Fraction::new(3, 10);
            str_gear_bonus += Fraction::new(1, 4);
        } else if player.is_wearing("Scorching bow", None) && monster.is_demon() {
            // Scorching bow boost is applied additively with slayer helm (verified in-game)
            att_gear_bonus += Fraction::new(3, 10);
            str_gear_bonus += Fraction::new(3, 10);
        }
    }

    if player.is_wearing("Eclipse atlatl", None) {
        str_gear_bonus = melee_gear_bonus(player, monster);
    }

    (att_gear_bonus, str_gear_bonus)
}

fn apply_silver_bolts_bonus(
    att_roll: i32,
    max_hit: u32,
    player: &Player,
    monster: &Monster,
) -> (i32, u32) {
    let mut att_roll = att_roll;
    let mut max_hit = max_hit;
    let vampyre_tier = monster.vampyre_tier();
    if player.is_wearing("Silver bolts", None)
        && player.is_using_crossbow()
        && vampyre_tier.is_some()
    {
        if player.is_wearing("Efaritay's aid", None) {
            att_roll = att_roll * 115 / 100;
        }
        match vampyre_tier {
            Some(1) => max_hit = max_hit * 11 / 10,
            Some(3) => {
                att_roll = 0;
                max_hit = 0;
            }
            _ => {}
        }
    }

    (att_roll, max_hit)
}

fn apply_ranged_weapon_boosts(
    att_roll: i32,
    max_hit: u32,
    player: &Player,
    monster: &Monster,
) -> (i32, u32) {
    let mut att_roll = att_roll;
    let mut max_hit = max_hit;

    let (att_factor, max_hit_factor) = match player.gear.weapon.name.as_str() {
        // DHCB is applied multiplicatively with anything but slayer helm
        "Dragon hunter crossbow" if monster.is_dragon() && mult_boost_applies(player, monster) => {
            (Fraction::new(13, 10), Fraction::new(5, 4))
        }
        "Twisted bow" => {
            let (tbow_acc_bonus, tbow_dmg_bonus) = monster.tbow_bonuses();
            (
                Fraction::new(tbow_acc_bonus, 100),
                Fraction::new(tbow_dmg_bonus, 100),
            )
        }
        "Scorching bow" if monster.is_demon() && mult_boost_applies(player, monster) => {
            (Fraction::new(13, 10), Fraction::new(13, 10))
        }

        // Wildy bow is applied multiplicatively with anything but slayer helm
        _ if (monster.is_in_wilderness() || player.boosts.in_wilderness)
            && player.is_wearing_wildy_bow()
            && mult_boost_applies(player, monster) =>
        {
            (Fraction::new(3, 2), Fraction::new(3, 2))
        }
        _ => (Fraction::new(1, 1), Fraction::new(1, 1)),
    };

    att_roll = att_factor.multiply_to_int(att_roll);
    max_hit = max_hit_factor.multiply_to_int(max_hit);

    // Apply bone shortbow bonus additively
    if player.is_wearing("Bone shortbow", None) && monster.is_rat() {
        max_hit += 10;
    }

    (att_roll, max_hit)
}

fn mult_boost_applies(player: &Player, monster: &Monster) -> bool {
    !(player.boosts.on_task && player.is_wearing_imbued_black_mask())
        || (player.is_wearing_salve_i() && monster.is_undead())
        || (player.is_wearing("Amulet of avarice", None) && monster.is_revenant())
}

fn get_base_magic_hit(player: &Player) -> u32 {
    if let Some(spell) = &player.attrs.spell {
        spell.max_hit(player)
    } else if player.is_wearing_salamander() {
        salamander_max_hit(player)
    } else {
        charged_staff_max_hit(player)
    }
}

fn salamander_max_hit(player: &Player) -> u32 {
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

fn charged_staff_max_hit(player: &Player) -> u32 {
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

fn apply_shadow_boost(magic_attack: i32, magic_damage: u32, monster: &Monster) -> (i32, u32) {
    let multiplier = if monster.is_toa_monster() { 4 } else { 3 };
    let magic_attack = magic_attack * multiplier;
    let magic_damage = min(1000, magic_damage * multiplier as u32);

    (magic_attack, magic_damage)
}

fn calc_eff_magic_lvl(player: &Player) -> u32 {
    let stance_bonus = if player.combat_stance() == CombatStance::Accurate {
        11
    } else {
        9
    };

    let magic_att_pray_boost = player.prayers.magic_att;

    let void_bonus = if player.set_effects.full_void || player.set_effects.full_elite_void {
        Fraction::new(145, 100)
    } else {
        Fraction::new(1, 1)
    };

    let visible_magic = player.live_stats.magic;

    void_bonus.multiply_to_int(visible_magic * (100 + magic_att_pray_boost) / 100) + stance_bonus
}

fn apply_virtus_bonus(magic_damage: u32, player: &Player) -> u32 {
    if player.is_using_ancient_spell() {
        magic_damage
            + [
                player.gear.head.as_ref(),
                player.gear.body.as_ref(),
                player.gear.legs.as_ref(),
            ]
            .iter()
            .filter(|slot| slot.is_some() && slot.as_ref().unwrap().name.contains("Virtus"))
            .count() as u32
                * 30
    } else {
        magic_damage
    }
}

fn apply_additive_magic_boosts(
    att_roll: i32,
    magic_damage: u32,
    player: &Player,
    monster: &Monster,
) -> (i32, u32, bool) {
    let mut att_roll = att_roll;
    let mut magic_damage = magic_damage;
    let mut salve_active = true;
    let mut att_roll_mod = 100;

    if player.is_wearing("Amulet of avarice", None) && monster.is_revenant() {
        if player.boosts.forinthry_surge {
            att_roll_mod += 35;
            magic_damage += 350;
        } else {
            att_roll_mod += 20;
            magic_damage += 200;
        }
    } else if player.is_wearing("Salve amulet(ei)", None) && monster.is_undead() {
        att_roll_mod += 20;
        magic_damage += 200;
    } else if player.is_wearing("Salve amulet(i)", None) {
        att_roll_mod += 15;
        magic_damage += 150;
    } else {
        salve_active = false;
    }

    if player.is_wearing_smoke_staff() && player.is_using_standard_spell() {
        att_roll_mod += 10;
        magic_damage += 100;
    }

    if let Some(1) = monster.vampyre_tier() {
        if player.is_wearing("Efaritay's aid", None) && player.is_wearing_silver_weapon() {
            att_roll_mod += 15;
        }
    }

    att_roll = att_roll * att_roll_mod / 100;

    (att_roll, magic_damage, salve_active)
}

fn apply_wildy_staff_boost(att_roll: i32, player: &Player, monster: &Monster) -> (i32, u32) {
    if (player.boosts.in_wilderness || monster.is_in_wilderness())
        && player.is_wearing_wildy_staff()
    {
        (att_roll * 3 / 2, 50)
    } else {
        (att_roll, 0)
    }
}

fn apply_chaos_gauntlet_boost(max_hit: u32, player: &Player) -> u32 {
    let bolt_spells = [
        StandardSpell::WindBolt,
        StandardSpell::WaterBolt,
        StandardSpell::EarthBolt,
        StandardSpell::FireBolt,
    ];

    if let Some(Spell::Standard(standard_spell)) = player.attrs.spell {
        if player.is_wearing("Chaos gauntlets", None) && bolt_spells.contains(&standard_spell) {
            return max_hit + 3;
        }
    }

    max_hit
}

fn apply_charge_boost(max_hit: u32, player: &Player) -> u32 {
    let god_spells = [
        StandardSpell::ClawsOfGuthix,
        StandardSpell::SaradominStrike,
        StandardSpell::FlamesOfZamorak,
    ];

    if player.boosts.charge_active {
        if let Some(Spell::Standard(standard_spell)) = player.attrs.spell {
            if god_spells.contains(&standard_spell) {
                return max_hit + 10;
            }
        }
    }

    max_hit
}

fn get_elemental_weakness_boost(player: &Player, monster: &Monster) -> u32 {
    let weakness = &monster.info.weakness;
    let weakness_applies = match weakness {
        Some(w) => match w.element.as_str() {
            "Fire" if player.is_using_fire_spell() => true,
            "Water" if player.is_using_water_spell() => true,
            "Air" if player.is_using_air_spell() => true,
            "Earth" if player.is_using_earth_spell() => true,
            _ => false,
        },
        None => false,
    };

    if weakness_applies {
        weakness.as_ref().unwrap().severity as u32
    } else {
        0
    }
}
