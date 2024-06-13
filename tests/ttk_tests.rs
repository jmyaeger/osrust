use osrs::combat::simulate_n_fights;
use osrs::dps_calc;
use osrs::monster::Monster;
use osrs::player::Player;
use osrs::potions::Potion;
use osrs::rolls::calc_active_player_rolls;
use rstest::rstest;
mod fixtures;
use fixtures::*;

#[rstest]
#[case(max_melee_player())]
#[case(max_ranged_zcb_player())]
#[case(max_mage_sang_staff_player())]
fn test_max_setups_ammonite_crab_ttk(#[case] mut player: Player, ammonite_crab: Monster) {
    let mut monster = ammonite_crab;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.1);
}

#[rstest]
fn test_max_mage_brimstone_ring_kril_ttk(
    max_mage_sang_staff_brimstone_ring_player: Player,
    kril: Monster,
) {
    let mut monster = kril;
    let mut player = max_mage_sang_staff_brimstone_ring_player;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.1);
}

#[rstest]
#[case(max_melee_blade_player())]
#[case(max_melee_scythe_player())]
fn test_vardorvis_ttk(#[case] mut player: Player, vardorvis: Monster) {
    let mut monster = vardorvis;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.1);
}

#[rstest]
#[case(kephri_400())]
#[case(vorkath())]
fn test_fang_ttk(max_melee_fang_player: Player, #[case] mut monster: Monster) {
    let mut player = max_melee_fang_player;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.1);
}

#[rstest]
#[case(full_ahrims_aotd_player())]
#[case(full_ahrims_aotd_sunfire_player())]
#[case(full_dharoks_1hp_player())]
#[case(full_veracs_player())]
#[case(full_karils_aotd_player())]
#[case(max_melee_torags_hammers_player())]
fn test_barrows_gear_ttks(#[case] mut player: Player, scurrius: Monster) {
    let mut monster = scurrius;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.1);
}

#[rstest]
fn test_blue_keris_kq_ttk(
    max_melee_blue_keris_partisan_player: Player,
    kalphite_queen_p1: Monster,
) {
    let mut player = max_melee_blue_keris_partisan_player;
    let mut monster = kalphite_queen_p1;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.1);
}

#[rstest]
#[case("Opal dragon bolts (e)")]
#[case("Pearl dragon bolts (e)")]
#[case("Ruby dragon bolts (e)")]
#[case("Diamond dragon bolts (e)")]
#[case("Onyx dragon bolts (e)")]
#[case("Dragonstone dragon bolts (e)")]
fn test_enchanted_bolt_acb_ttks(#[case] bolt_name: &str) {
    let mut player = max_ranged_zcb_player();
    let mut monster = scurrius();
    player.equip("Armadyl crossbow", None);
    player.equip(bolt_name, None);
    player.update_bonuses();
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);
    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.1);
}

#[rstest]
#[case("Opal dragon bolts (e)")]
#[case("Pearl dragon bolts (e)")]
#[case("Ruby dragon bolts (e)")]
#[case("Diamond dragon bolts (e)")]
#[case("Onyx dragon bolts (e)")]
#[case("Dragonstone dragon bolts (e)")]
fn test_enchanted_bolt_zcb_ttks(#[case] bolt_name: &str) {
    let mut player = max_ranged_zcb_player();
    let mut monster = scurrius();
    player.equip(bolt_name, None);
    player.update_bonuses();
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.1);
}

#[rstest]
#[case(count_draynor())]
#[case(aberrant_spectre())]
#[case(abhorrent_spectre())]
#[case(general_graardor())]
fn test_scythe_against_different_sizes_ttk(
    max_melee_scythe_player: Player,
    #[case] mut monster: Monster,
) {
    let mut player = max_melee_scythe_player;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.1);
}

#[rstest]
#[case(urium_shade())]
#[case(ammonite_crab())]
fn test_gadderhammer_ttk(max_melee_player: Player, #[case] mut monster: Monster) {
    let mut player = max_melee_player;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.1);
}

#[rstest]
#[case(max_ranged_tonalztics_uncharged_player())]
#[case(max_ranged_tonalztics_charged_player())]
fn test_tonalztics_ttk(#[case] mut player: Player, scurrius: Monster) {
    let mut monster = scurrius;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.1);
}

#[rstest]
fn test_macuahuitl_no_set_effect_ttk(max_melee_macuahuitl_player: Player, scurrius: Monster) {
    let mut player = max_melee_macuahuitl_player;
    let mut monster = scurrius;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.1);
}

#[rstest]
#[case(max_ranged_tbow_player())]
#[case(max_ranged_zcb_ruby_player())]
fn test_max_range_zulrah(#[case] mut player: Player, zulrah_tanzanite: Monster) {
    let mut monster = zulrah_tanzanite;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.1);
}

#[rstest]
fn test_max_mage_shadow_zulrah(max_mage_shadow_player: Player, zulrah_magma: Monster) {
    let mut player = max_mage_shadow_player;
    let mut monster = zulrah_magma;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.1);
}

#[rstest]
fn test_max_mage_seren(max_mage_shadow_player: Player, seren: Monster) {
    let mut player = max_mage_shadow_player;
    let mut monster = seren;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);

    assert!(num::abs(calc_ttk - ttk) < 0.1);
}

#[rstest]
fn test_max_ranged_kraken(max_ranged_tbow_player: Player, kraken: Monster) {
    let mut player = max_ranged_tbow_player;
    let mut monster = kraken;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 10000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.5);
}

#[rstest]
#[case(max_melee_scythe_player())]
#[case(max_melee_blade_player())]
#[case(max_mage_dawnbringer_player())]
fn test_verzik_p1(#[case] mut player: Player, verzik_p1: Monster) {
    let mut monster = verzik_p1;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 10000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 1.0);
}

#[rstest]
fn test_max_mage_tekton(max_mage_shadow_player: Player, tekton: Monster) {
    let mut player = max_mage_shadow_player;
    let mut monster = tekton;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.1);
}

#[rstest]
fn max_mage_vasa_crystal(max_mage_shadow_player: Player, vasa_crystal: Monster) {
    let mut player = max_mage_shadow_player;
    let mut monster = vasa_crystal;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.1);
}

#[rstest]
#[case(olm_melee_hand())]
#[case(olm_head())]
fn test_olm_mage_offstyle(max_mage_shadow_player: Player, #[case] mut monster: Monster) {
    let mut player = max_mage_shadow_player;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.2);
}

#[rstest]
#[case(olm_mage_hand())]
#[case(olm_melee_hand())]
fn test_olm_ranged_offstyle(max_ranged_tbow_overload_player: Player, #[case] mut monster: Monster) {
    let mut player = max_ranged_tbow_overload_player;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.5);
}

#[rstest]
fn test_max_ranged_tbow_ice_demon(max_ranged_tbow_overload_player: Player, ice_demon: Monster) {
    let mut player = max_ranged_tbow_overload_player;
    let mut monster = ice_demon;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.1);
}

#[rstest]
fn test_max_melee_slagilith(max_melee_player: Player, slagilith: Monster) {
    let mut player = max_melee_player;
    let mut monster = slagilith;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.1);
}

#[rstest]
#[case(max_mage_harm_crumble_undead())]
#[case(max_range_comp_ogre_bow_player())]
#[case(max_melee_player())]
fn test_zogre_ttk(#[case] mut player: Player, zogre: Monster) {
    let mut monster = zogre;
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.1);
}

#[rstest]
fn test_ruby_bolts_zcb_zebak_500(max_ranged_zcb_ruby_player: Player, zebak: Monster) {
    let mut player = max_ranged_zcb_ruby_player;
    player.add_potion(Potion::SmellingSalts);

    let mut monster = zebak;
    monster.info.toa_level = 500;
    monster.scale_toa();
    calc_active_player_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.2);
}
