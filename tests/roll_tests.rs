use osrs::equipment::{CombatStyle, CombatType};
use osrs::monster::Monster;
use osrs::player::Player;
use osrs::potions::{Potion, PotionBoost};
use osrs::prayers::{Prayer, PrayerBoost};
use osrs::rolls::{calc_player_melee_rolls, calc_player_ranged_rolls};
use rstest::rstest;

mod fixtures;
use fixtures::*;

#[rstest]
#[case(max_melee_player(), ammonite_crab(), &CombatType::Stab, 33525, 56)]
#[case(mid_level_melee_player(), ammonite_crab(), &CombatType::Slash, 24125, 39)]
#[case(max_melee_dhl_player(), vorkath(), &CombatType::Stab, 38880, 60)]
#[case(max_melee_keris_partisan_player(), kalphite(), &CombatType::Crush, 27714, 59)]
#[case(max_melee_blue_keris_partisan_player(), kalphite(), &CombatType::Crush, 36859, 59)]
#[case(mid_level_melee_barronite_player(), chaos_golem(), &CombatType::Crush, 18600, 35)]
#[case(max_melee_ursine_chainmace_player(), vetion(), &CombatType::Crush, 44700, 78)]
#[case(max_melee_ursine_chainmace_player(), ammonite_crab(), &CombatType::Crush, 29800, 52)]
#[case(max_melee_berserker_neck_obby_sword_player(), ammonite_crab(), &CombatType::Stab, 22797, 54)]
#[case(max_melee_silverlight_player(), kril(), &CombatType::Slash, 21456, 59)]
#[case(max_melee_darklight_player(), kril(), &CombatType::Slash, 21754, 59)]
#[case(max_melee_arclight_player(), kril(), &CombatType::Slash, 42554, 61)]
#[case(max_melee_silverlight_player(), duke(), &CombatType::Slash, 21456, 52)]
#[case(max_melee_darklight_player(), duke(), &CombatType::Slash, 21754, 52)]
#[case(max_melee_arclight_player(), duke(), &CombatType::Slash, 37297, 53)]
#[case(mid_level_melee_lbba_player(), kurask(), &CombatType::Slash, 22692, 49)]
#[case(max_melee_colossal_blade_player(), aberrant_spectre(), &CombatType::Slash, 29651, 60)]
#[case(max_melee_colossal_blade_player(), abhorrent_spectre(), &CombatType::Slash, 29651, 62)]
#[case(max_melee_colossal_blade_player(), general_graardor(), &CombatType::Slash, 29651, 64)]
#[case(max_melee_colossal_blade_player(), rune_dragon(), &CombatType::Slash, 29651, 66)]
#[case(mid_level_melee_bone_mace_player(), scurrius(), &CombatType::Crush, 21080, 44)]
#[case(slayer(max_melee_player()), ammonite_crab(), &CombatType::Stab, 39112, 63)]
#[case(slayer(mid_level_melee_player()), ammonite_crab(), &CombatType::Slash, 28145, 45)]
#[case(slayer(max_melee_dhl_player()), vorkath(), &CombatType::Stab, 45360, 67)]
#[case(slayer(max_melee_keris_partisan_player()), kalphite(), &CombatType::Crush, 32333, 66)]
#[case(slayer(max_melee_blue_keris_partisan_player()), kalphite(), &CombatType::Crush, 43002, 66)]
#[case(slayer(max_melee_ursine_chainmace_player()), vetion(), &CombatType::Crush, 52149, 87)]
#[case(slayer(max_melee_berserker_neck_obby_sword_player()), ammonite_crab(), &CombatType::Stab, 26596, 60)]
#[case(slayer(max_melee_silverlight_player()), kril(), &CombatType::Slash, 25032, 64)]
#[case(slayer(max_melee_darklight_player()), kril(), &CombatType::Slash, 25379, 64)]
#[case(slayer(max_melee_arclight_player()), kril(), &CombatType::Slash, 49646, 66)]
#[case(slayer(max_melee_silverlight_player()), duke(), &CombatType::Slash, 25032, 56)]
#[case(slayer(max_melee_darklight_player()), duke(), &CombatType::Slash, 25379, 56)]
#[case(slayer(max_melee_arclight_player()), duke(), &CombatType::Slash, 43513, 58)]
#[case(slayer(mid_level_melee_lbba_player()), kurask(), &CombatType::Slash, 26474, 55)]
#[case(slayer(max_melee_colossal_blade_player()), aberrant_spectre(), &CombatType::Slash, 34592, 67)]
#[case(slayer(max_melee_colossal_blade_player()), abhorrent_spectre(), &CombatType::Slash, 34592, 69)]
#[case(slayer(max_melee_colossal_blade_player()), general_graardor(), &CombatType::Slash, 34592, 71)]
#[case(slayer(max_melee_colossal_blade_player()), rune_dragon(), &CombatType::Slash, 34592, 73)]
#[case(slayer(mid_level_melee_bone_mace_player()), scurrius(), &CombatType::Crush, 24593, 49)]
#[case(salve_ei(max_melee_player()), vorkath(), &CombatType::Stab, 37548, 63)]
#[case(salve_i(max_melee_player()), vorkath(), &CombatType::Stab, 36505, 61)]
#[case(salve_ei(mid_level_melee_player()), vorkath(), &CombatType::Slash, 27450, 45)]
#[case(salve_i(mid_level_melee_player()), vorkath(), &CombatType::Slash, 26687, 44)]
#[case(salve_ei(max_melee_dhl_player()), vorkath(), &CombatType::Stab, 43416, 68)]
#[case(salve_ei(max_melee_ursine_chainmace_player()), vetion(), &CombatType::Crush, 49617, 87)]
#[case(salve_ei(max_melee_colossal_blade_player()), aberrant_spectre(), &CombatType::Slash, 32899, 68)]
#[case(salve_ei(max_melee_colossal_blade_player()), abhorrent_spectre(), &CombatType::Slash, 32899, 70)]
#[case(salve_ei(max_melee_colossal_blade_player()), bloat(), &CombatType::Slash, 32899, 74)]
#[case(salve_ei(slayer(max_melee_dhl_player())), vorkath(), &CombatType::Stab, 43416, 66)]
#[case(avarice_forinthry(max_melee_player()), revenant_dragon(), &CombatType::Stab, 44253, 74)]
#[case(avarice_forinthry(max_melee_colossal_blade_player()), revenant_dragon(), &CombatType::Slash, 39023, 84)]
#[case(avarice_forinthry(max_melee_ursine_chainmace_player()), revenant_dragon(), &CombatType::Crush, 58836, 102)]
#[case(full_obby_with_sword_player(), ammonite_crab(), &CombatType::Stab, 29174, 47)]
#[case(full_obby_with_sword_and_necklace_player(), ammonite_crab(), &CombatType::Stab, 25076, 55)]
#[case(salve_ei(full_obby_with_sword_player()), vorkath(), &CombatType::Stab, 31572, 53)]
#[case(max_melee_arclight_player(), count_draynor(), &CombatType::Slash, 25032, 39)]
#[case(max_melee_arclight_player(), vampyre_juvinate(), &CombatType::Slash, 25032, 36)]
#[case(max_melee_arclight_player(), vanstrom_klause(), &CombatType::Slash, 0, 0)]
#[case(efaritays_aid(max_melee_player()), count_draynor(), &CombatType::Stab, 33525, 58)] //TODO: Update these with new effect
#[case(efaritays_aid(max_melee_player()), vampyre_juvinate(), &CombatType::Stab, 33525, 53)]
#[case(efaritays_aid(max_melee_player()), vanstrom_klause(), &CombatType::Stab, 0, 0)]
#[case(max_melee_player(), count_draynor(), &CombatType::Stab, 33525, 56)]
#[case(max_melee_player(), vampyre_juvinate(), &CombatType::Stab, 0, 0)]
#[case(max_melee_player(), vanstrom_klause(), &CombatType::Stab, 0, 0)]
#[case(max_melee_blisterwood_flail_player(), count_draynor(), &CombatType::Crush, 30820, 56)]
#[case(max_melee_blisterwood_flail_player(), vampyre_juvinate(), &CombatType::Crush, 30820, 56)]
#[case(max_melee_blisterwood_flail_player(), vanstrom_klause(), &CombatType::Crush, 30820, 56)]
#[case(slayer(max_melee_arclight_player()), count_draynor(), &CombatType::Slash, 29204, 42)]
#[case(slayer(max_melee_arclight_player()), vampyre_juvinate(), &CombatType::Slash, 29204, 39)]
#[case(slayer(max_melee_arclight_player()), vanstrom_klause(), &CombatType::Slash, 0, 0)]
#[case(slayer(efaritays_aid(max_melee_player())), count_draynor(), &CombatType::Stab, 39112, 64)] //TODO: Update these with new effect
#[case(slayer(efaritays_aid(max_melee_player())), vampyre_juvinate(), &CombatType::Stab, 39112, 59)]
#[case(slayer(efaritays_aid(max_melee_player())), vanstrom_klause(), &CombatType::Stab, 0, 0)]
#[case(slayer(max_melee_blisterwood_flail_player()), count_draynor(), &CombatType::Crush, 35957, 62)]
#[case(slayer(max_melee_blisterwood_flail_player()), vampyre_juvinate(), &CombatType::Crush, 35957, 62)]
#[case(slayer(max_melee_blisterwood_flail_player()), vanstrom_klause(), &CombatType::Crush, 35957, 62)]

fn test_melee_player_rolls(
    #[case] mut player: Player,
    #[case] monster: Monster,
    #[case] combat_type: &CombatType,
    #[case] att_roll: u32,
    #[case] max_hits: u32,
) {
    calc_player_melee_rolls(&mut player, &monster);
    assert_eq!(player.att_rolls[combat_type], att_roll);
    assert_eq!(player.max_hits[combat_type], max_hits);
}

#[rstest]
fn test_max_ranged_zcb(mut max_ranged_player: Player, ammonite_crab: Monster) {
    calc_player_ranged_rolls(&mut max_ranged_player, &ammonite_crab);

    assert_eq!(max_ranged_player.att_rolls[&CombatType::Ranged], 50694);
    assert_eq!(max_ranged_player.max_hits[&CombatType::Ranged], 49);
}

#[rstest]
fn test_mid_level_ranged_rcb(mut mid_level_ranged_player: Player, ammonite_crab: Monster) {
    calc_player_ranged_rolls(&mut mid_level_ranged_player, &ammonite_crab);

    assert_eq!(
        mid_level_ranged_player.att_rolls[&CombatType::Ranged],
        29945
    );
    assert_eq!(mid_level_ranged_player.max_hits[&CombatType::Ranged], 30);
}

#[rstest]
fn test_max_ranged_blowpipe_dragon_darts(mut max_ranged_player: Player, ammonite_crab: Monster) {
    max_ranged_player.equip("Toxic blowpipe (dragon)");
    max_ranged_player.update_bonuses();
    calc_player_ranged_rolls(&mut max_ranged_player, &ammonite_crab);

    assert_eq!(max_ranged_player.att_rolls[&CombatType::Ranged], 35358);
    assert_eq!(max_ranged_player.max_hits[&CombatType::Ranged], 31);
}

#[rstest]
#[case("Ammonite Crab", (16983, 19))]
#[case("General Graardor", (36089, 43))]
#[case("K'ril Tsutsaroth", (54770, 70))]
#[case("Commander Zilyana", (59441, 79))]
fn test_max_ranged_tbow(
    #[case] monster_name: &str,
    #[case] expected_rolls: (u32, u32),
    mut max_ranged_player: Player,
) {
    let monster = Monster::new(monster_name).unwrap();
    max_ranged_player.equip("Twisted bow");
    max_ranged_player.gear.ammo = None;
    max_ranged_player.equip("Dragon arrow");
    max_ranged_player.update_bonuses();
    calc_player_ranged_rolls(&mut max_ranged_player, &monster);

    assert_eq!(
        max_ranged_player.att_rolls[&CombatType::Ranged],
        expected_rolls.0
    );
    assert_eq!(
        max_ranged_player.max_hits[&CombatType::Ranged],
        expected_rolls.1
    );
}

#[rstest]
#[case("Lizardman shaman (Chambers of Xeric) (Normal)", (48174, 60))]
#[case("Abyssal portal (Normal)", (55446, 71))]
#[case("Skeletal Mystic (Normal)", (50447, 63))]
#[case("Great Olm (Head)", (63627, 86))]
#[case("Great Olm (Head (Challenge Mode))", (63627, 99))]
fn test_max_ranged_tbow_cox(
    #[case] monster_name: &str,
    #[case] expected_rolls: (u32, u32),
    mut max_ranged_player: Player,
) {
    let monster = Monster::new(monster_name).unwrap();
    max_ranged_player.equip("Twisted bow");
    max_ranged_player.gear.ammo = None;
    max_ranged_player.equip("Dragon arrow");
    max_ranged_player.potions.ranged = Some(PotionBoost::new(Potion::OverloadPlus));
    max_ranged_player.calc_potion_boosts();
    max_ranged_player.reset_live_stats();
    max_ranged_player.update_bonuses();

    calc_player_ranged_rolls(&mut max_ranged_player, &monster);

    assert_eq!(
        max_ranged_player.att_rolls[&CombatType::Ranged],
        expected_rolls.0
    );
    assert_eq!(
        max_ranged_player.max_hits[&CombatType::Ranged],
        expected_rolls.1
    );
}

#[rstest]
fn test_max_ranged_tbow_zebak_400(mut max_ranged_player: Player, mut zebak: Monster) {
    zebak.info.toa_level = 400;
    zebak.scale_toa();

    max_ranged_player.equip("Twisted bow");
    max_ranged_player.gear.ammo = None;
    max_ranged_player.equip("Dragon arrow");
    max_ranged_player.potions.ranged = Some(PotionBoost::new(Potion::SmellingSalts));
    max_ranged_player.calc_potion_boosts();
    max_ranged_player.reset_live_stats();
    max_ranged_player.update_bonuses();

    calc_player_ranged_rolls(&mut max_ranged_player, &zebak);

    assert_eq!(max_ranged_player.att_rolls[&CombatType::Ranged], 63304);
    assert_eq!(max_ranged_player.max_hits[&CombatType::Ranged], 83);
}

#[rstest]
fn test_dhcb_vorkath(mut max_ranged_player: Player, vorkath: Monster) {
    max_ranged_player.equip("Dragon hunter crossbow");
    max_ranged_player.update_bonuses();
    calc_player_ranged_rolls(&mut max_ranged_player, &vorkath);

    assert_eq!(max_ranged_player.att_rolls[&CombatType::Ranged], 63133);
    assert_eq!(max_ranged_player.max_hits[&CombatType::Ranged], 61);
}

#[rstest]
fn test_webweaver_spindel(mut max_ranged_player: Player, spindel: Monster) {
    max_ranged_player.equip("Webweaver bow");
    max_ranged_player.update_bonuses();
    calc_player_ranged_rolls(&mut max_ranged_player, &spindel);

    assert_eq!(max_ranged_player.att_rolls[&CombatType::Ranged], 64752);
    assert_eq!(max_ranged_player.max_hits[&CombatType::Ranged], 51);
}

#[rstest]
fn test_webweaver_non_wildy_monster(mut max_ranged_player: Player, ammonite_crab: Monster) {
    max_ranged_player.equip("Webweaver bow");
    max_ranged_player.update_bonuses();
    calc_player_ranged_rolls(&mut max_ranged_player, &ammonite_crab);

    assert_eq!(max_ranged_player.att_rolls[&CombatType::Ranged], 43168);
    assert_eq!(max_ranged_player.max_hits[&CombatType::Ranged], 34);
}

#[rstest]
fn test_full_eclipse_atlatl_ranged_gear(mut max_ranged_player: Player, ammonite_crab: Monster) {
    max_ranged_player.equip("Eclipse moon helm");
    max_ranged_player.equip("Eclipse moon chestplate");
    max_ranged_player.equip("Eclipse moon tassets");
    max_ranged_player.equip("Eclipse atlatl");
    max_ranged_player.equip("Atlatl dart");
    max_ranged_player.update_bonuses();
    calc_player_ranged_rolls(&mut max_ranged_player, &ammonite_crab);

    assert_eq!(max_ranged_player.att_rolls[&CombatType::Ranged], 39760);
    assert_eq!(max_ranged_player.max_hits[&CombatType::Ranged], 22);
}

#[rstest]
fn test_full_eclipse_atlatl_melee_gear_rigour_all_pots(
    mut max_melee_player: Player,
    ammonite_crab: Monster,
) {
    max_melee_player.equip("Eclipse moon helm");
    max_melee_player.equip("Eclipse moon chestplate");
    max_melee_player.equip("Eclipse moon tassets");
    max_melee_player.equip("Eclipse atlatl");
    max_melee_player.equip("Atlatl dart");
    max_melee_player.update_bonuses();
    max_melee_player.set_active_style(CombatStyle::Rapid);
    max_melee_player
        .prayers
        .add(PrayerBoost::new(Prayer::Rigour));
    max_melee_player.potions.ranged = Some(PotionBoost::new(Potion::Ranging));
    max_melee_player.calc_potion_boosts();
    max_melee_player.reset_live_stats();
    calc_player_ranged_rolls(&mut max_melee_player, &ammonite_crab);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Ranged], 27122);
    assert_eq!(max_melee_player.max_hits[&CombatType::Ranged], 38);

    max_melee_player.stats.strength = 80;
    max_melee_player.calc_potion_boosts();
    max_melee_player.reset_live_stats();

    calc_player_ranged_rolls(&mut max_melee_player, &ammonite_crab);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Ranged], 27122);
    assert_eq!(max_melee_player.max_hits[&CombatType::Ranged], 32);
}

#[rstest]
fn test_max_ranged_zcb_slayer(mut max_ranged_player: Player, ammonite_crab: Monster) {
    max_ranged_player.equip("Slayer helmet (i)");
    max_ranged_player.update_bonuses();
    calc_player_ranged_rolls(&mut max_ranged_player, &ammonite_crab);

    assert_eq!(max_ranged_player.att_rolls[&CombatType::Ranged], 56828);
    assert_eq!(max_ranged_player.max_hits[&CombatType::Ranged], 56);
}

#[rstest]
fn test_mid_level_ranged_rcb_slayer(mut mid_level_ranged_player: Player, ammonite_crab: Monster) {
    mid_level_ranged_player.equip("Slayer helmet (i)");
    mid_level_ranged_player.update_bonuses();
    calc_player_ranged_rolls(&mut mid_level_ranged_player, &ammonite_crab);

    assert_eq!(
        mid_level_ranged_player.att_rolls[&CombatType::Ranged],
        33916
    );
    assert_eq!(mid_level_ranged_player.max_hits[&CombatType::Ranged], 34);
}

#[rstest]
fn test_max_ranged_blowpipe_dragon_darts_slayer(
    mut max_ranged_player: Player,
    ammonite_crab: Monster,
) {
    max_ranged_player.equip("Slayer helmet (i)");
    max_ranged_player.equip("Toxic blowpipe (dragon)");
    max_ranged_player.update_bonuses();
    calc_player_ranged_rolls(&mut max_ranged_player, &ammonite_crab);

    assert_eq!(max_ranged_player.att_rolls[&CombatType::Ranged], 39192);
    assert_eq!(max_ranged_player.max_hits[&CombatType::Ranged], 35);
}

#[rstest]
fn test_full_eclipse_atlatl_ranged_gear_slayer(
    mut max_ranged_player: Player,
    ammonite_crab: Monster,
) {
    max_ranged_player.equip("Slayer helmet (i)");
    max_ranged_player.equip("Eclipse atlatl");
    max_ranged_player.equip("Atlatl dart");
    max_ranged_player.update_bonuses();
    calc_player_ranged_rolls(&mut max_ranged_player, &ammonite_crab);

    assert_eq!(max_ranged_player.att_rolls[&CombatType::Ranged], 48500);
    assert_eq!(max_ranged_player.max_hits[&CombatType::Ranged], 24);
}

#[rstest]
fn test_full_eclipse_atlatl_melee_gear_rigour_all_pots_slayer(
    mut max_melee_player: Player,
    ammonite_crab: Monster,
) {
    max_melee_player.equip("Slayer helmet (i)");
    max_melee_player.equip("Eclipse atlatl");
    max_melee_player.equip("Atlatl dart");
    max_melee_player.update_bonuses();
    max_melee_player.set_active_style(CombatStyle::Rapid);
    max_melee_player
        .prayers
        .add(PrayerBoost::new(Prayer::Rigour));
    max_melee_player.potions.ranged = Some(PotionBoost::new(Potion::Ranging));
    max_melee_player.calc_potion_boosts();
    max_melee_player.reset_live_stats();
    calc_player_ranged_rolls(&mut max_melee_player, &ammonite_crab);

    assert_eq!(max_melee_player.att_rolls[&CombatType::Ranged], 18452);
    assert_eq!(max_melee_player.max_hits[&CombatType::Ranged], 45);
}

#[rstest]
fn test_dhcb_vorkath_slayer(mut max_ranged_player: Player, vorkath: Monster) {
    max_ranged_player.equip("Slayer helmet (i)");
    max_ranged_player.equip("Dragon hunter crossbow");
    max_ranged_player.update_bonuses();
    calc_player_ranged_rolls(&mut max_ranged_player, &vorkath);

    assert_eq!(max_ranged_player.att_rolls[&CombatType::Ranged], 68564);
    assert_eq!(max_ranged_player.max_hits[&CombatType::Ranged], 68);
}

#[rstest]
fn test_webweaver_spindel_slayer(mut max_ranged_player: Player, spindel: Monster) {
    max_ranged_player.equip("Slayer helmet (i)");
    max_ranged_player.equip("Webweaver bow");
    max_ranged_player.update_bonuses();
    calc_player_ranged_rolls(&mut max_ranged_player, &spindel);

    assert_eq!(max_ranged_player.att_rolls[&CombatType::Ranged], 69118);
    assert_eq!(max_ranged_player.max_hits[&CombatType::Ranged], 54);
}

#[rstest]
fn test_bone_shortbow_scurrius(mut mid_level_ranged_player: Player, scurrius: Monster) {
    mid_level_ranged_player.equip("Bone shortbow");
    mid_level_ranged_player.equip("Rune arrow");
    mid_level_ranged_player.update_bonuses();
    calc_player_ranged_rolls(&mut mid_level_ranged_player, &scurrius);

    assert_eq!(
        mid_level_ranged_player.att_rolls[&CombatType::Ranged],
        26216
    );
    assert_eq!(mid_level_ranged_player.max_hits[&CombatType::Ranged], 30);
}

#[rstest]
fn test_bone_shortbow_scurrius_slayer(mut mid_level_ranged_player: Player, scurrius: Monster) {
    mid_level_ranged_player.equip("Slayer helmet (i)");
    mid_level_ranged_player.equip("Bone shortbow");
    mid_level_ranged_player.equip("Rune arrow");
    mid_level_ranged_player.update_bonuses();
    calc_player_ranged_rolls(&mut mid_level_ranged_player, &scurrius);

    assert_eq!(
        mid_level_ranged_player.att_rolls[&CombatType::Ranged],
        29628
    );
    assert_eq!(mid_level_ranged_player.max_hits[&CombatType::Ranged], 33);
}
