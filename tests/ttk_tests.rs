use osrs::combat::simulate_n_fights;
use osrs::dps_calc;
use osrs::monster::Monster;
use osrs::player::Player;
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
    player.equip("Armadyl crossbow");
    player.equip(bolt_name);
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
    player.equip(bolt_name);
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
