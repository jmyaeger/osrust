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
