use osrs::calc::analysis::SimulationStats;
use osrs::calc::dps_calc;
use osrs::calc::monster_scaling::scale_monster_hp_only;
use osrs::calc::rolls::calc_active_player_rolls;
use osrs::combat::simulation::simulate_n_fights;
use osrs::sims::single_way::{SingleWayConfig, SingleWayFight};
use osrs::types::equipment::CombatStyle;
use osrs::types::monster::Monster;
use osrs::types::player::Player;
use osrs::types::potions::Potion;
use rstest::rstest;
mod fixtures;
use fixtures::*;

#[rstest]
#[case(max_melee_player())]
#[case(max_ranged_zcb_player())]
#[case(max_mage_sang_staff_player())]
fn test_max_setups_ammonite_crab_ttk(#[case] mut player: Player, ammonite_crab: Monster) {
    let monster = ammonite_crab;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
}

#[rstest]
fn test_max_mage_brimstone_ring_kril_ttk(
    max_mage_sang_staff_brimstone_ring_player: Player,
    kril: Monster,
) {
    let monster = kril;
    let mut player = max_mage_sang_staff_brimstone_ring_player;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
}

#[rstest]
#[case(max_melee_blade_player())]
#[case(max_melee_scythe_player())]
fn test_vardorvis_ttk(#[case] mut player: Player, vardorvis: Monster) {
    let mut monster = vardorvis;
    scale_monster_hp_only(&mut monster, true);
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
}

#[rstest]
#[case(kephri_400())]
#[case(vorkath())]
fn test_fang_ttk(max_melee_fang_player: Player, #[case] monster: Monster) {
    let mut player = max_melee_fang_player;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    println!("calc_ttk: {calc_ttk}");
    println!("stats.ttk: {}", stats.ttk);
    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
}

#[rstest]
#[case(full_ahrims_aotd_player())]
#[case(full_ahrims_aotd_sunfire_player())]
#[case(full_dharoks_1hp_player())]
#[case(full_veracs_player())]
#[case(full_karils_aotd_player())]
#[case(max_melee_torags_hammers_player())]
fn test_barrows_gear_ttks(#[case] mut player: Player, scurrius: Monster) {
    let monster = scurrius;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
}

#[rstest]
fn test_blue_keris_kq_ttk(
    max_melee_blue_keris_partisan_player: Player,
    kalphite_queen_p1: Monster,
) {
    let mut player = max_melee_blue_keris_partisan_player;
    let monster = kalphite_queen_p1;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
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
    let monster = scurrius();
    let _ = player.equip("Armadyl crossbow", None);
    let _ = player.equip(bolt_name, None);
    player.update_bonuses();
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
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
    let monster = scurrius();
    let _ = player.equip(bolt_name, None);
    player.update_bonuses();
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
}

#[rstest]
#[case(count_draynor())]
#[case(aberrant_spectre())]
#[case(abhorrent_spectre())]
#[case(general_graardor())]
fn test_scythe_against_different_sizes_ttk(
    max_melee_scythe_player: Player,
    #[case] monster: Monster,
) {
    let mut player = max_melee_scythe_player;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
}

#[rstest]
#[case(urium_shade())]
#[case(ammonite_crab())]
fn test_gadderhammer_ttk(max_melee_player: Player, #[case] monster: Monster) {
    let mut player = max_melee_player;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
}

#[rstest]
#[case(max_ranged_tonalztics_uncharged_player())]
#[case(max_ranged_tonalztics_charged_player())]
fn test_tonalztics_ttk(#[case] mut player: Player, scurrius: Monster) {
    let monster = scurrius;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
}

#[rstest]
fn test_macuahuitl_no_set_effect_ttk(max_melee_macuahuitl_player: Player, scurrius: Monster) {
    let mut player = max_melee_macuahuitl_player;
    let monster = scurrius;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
}

#[rstest]
fn test_macuahuitl_no_set_effect_baba_ttk(max_melee_macuahuitl_player: Player, baba_300: Monster) {
    let mut player = max_melee_macuahuitl_player;
    player.set_active_style(CombatStyle::Spike);
    let monster = baba_300;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
}

#[rstest]
#[case(max_ranged_tbow_player())]
#[case(max_ranged_zcb_ruby_player())]
fn test_max_range_zulrah(#[case] mut player: Player, zulrah_tanzanite: Monster) {
    let monster = zulrah_tanzanite;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
}

#[rstest]
fn test_max_mage_shadow_zulrah(max_mage_shadow_player: Player, zulrah_magma: Monster) {
    let mut player = max_mage_shadow_player;
    let monster = zulrah_magma;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
}

#[rstest]
fn test_max_mage_seren(max_mage_shadow_player: Player, seren: Monster) {
    let mut player = max_mage_shadow_player;
    let monster = seren;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");

    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
}

#[rstest]
fn test_max_ranged_kraken(max_ranged_tbow_player: Player, kraken: Monster) {
    let mut player = max_ranged_tbow_player;
    let monster = kraken;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 10000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.5);
}

#[rstest]
#[case(max_melee_scythe_player())]
#[case(max_melee_blade_player())]
#[case(max_mage_dawnbringer_player())]
fn test_verzik_p1(#[case] mut player: Player, verzik_p1: Monster) {
    let monster = verzik_p1;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 10000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 1.0);
}

#[rstest]
fn test_max_mage_tekton(max_mage_shadow_player: Player, tekton: Monster) {
    let mut player = max_mage_shadow_player;
    let monster = tekton;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
}

#[rstest]
fn max_mage_vasa_crystal(max_mage_shadow_player: Player, vasa_crystal: Monster) {
    let mut player = max_mage_shadow_player;
    let monster = vasa_crystal;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
}

#[rstest]
#[case(olm_melee_hand())]
#[case(olm_head())]
fn test_olm_mage_offstyle(max_mage_shadow_player: Player, #[case] monster: Monster) {
    let mut player = max_mage_shadow_player;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.2);
}

#[rstest]
#[case(olm_mage_hand())]
#[case(olm_melee_hand())]
fn test_olm_ranged_offstyle(max_ranged_tbow_overload_player: Player, #[case] monster: Monster) {
    let mut player = max_ranged_tbow_overload_player;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.5);
}

#[rstest]
fn test_max_ranged_tbow_ice_demon(max_ranged_tbow_overload_player: Player, ice_demon: Monster) {
    let mut player = max_ranged_tbow_overload_player;
    let monster = ice_demon;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
}

#[rstest]
fn test_max_melee_slagilith(max_melee_player: Player, slagilith: Monster) {
    let mut player = max_melee_player;
    let monster = slagilith;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
}

#[rstest]
#[case(max_mage_harm_crumble_undead())]
#[case(max_range_comp_ogre_bow_player())]
#[case(max_melee_player())]
fn test_zogre_ttk(#[case] mut player: Player, zogre: Monster) {
    let monster = zogre;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.1);
}

#[rstest]
fn test_ruby_bolts_zcb_zebak_500(max_ranged_zcb_ruby_player: Player, zebak: Monster) {
    let mut player = max_ranged_zcb_ruby_player;
    player.add_potion(Potion::SmellingSalts);

    let mut monster = zebak;
    monster.info.toa_level = 500;
    monster.scale_toa();
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.2);
}

#[rstest]
#[case(max_ranged_zcb_ruby_player())]
#[case(max_melee_fang_player())]
#[case(max_melee_player())]
fn test_corp_limiters(#[case] mut player: Player, corp: Monster) {
    let monster = corp;
    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(
        player.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    let dist = dps_calc::get_distribution(&player, &monster, false)
        .expect("Error calculating attack distribution.");
    let calc_ttk =
        dps_calc::get_ttk(dist, &player, &monster, false, false).expect("Error calculating ttk.");
    assert!(num::abs(calc_ttk - stats.ttk) < 0.5);
}

#[rstest]
fn test_blood_moon_set(full_blood_moon_player: Player, baba_300: Monster) {
    let mut player1 = full_blood_moon_player;
    let monster = baba_300;

    player1.set_active_style(CombatStyle::Spike);

    let mut player2 = player1.clone();
    player2.set_effects.full_blood_moon = false;

    calc_active_player_rolls(&mut player1, &monster);
    calc_active_player_rolls(&mut player2, &monster);

    let simulation1 = SingleWayFight::new(
        player1.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results1 = simulate_n_fights(Box::new(simulation1), 100000);
    let stats1 = SimulationStats::new(&results1);

    let simulation2 = SingleWayFight::new(
        player2.clone(),
        monster.clone(),
        SingleWayConfig::default(),
        None,
        false,
    );
    let results2 = simulate_n_fights(Box::new(simulation2), 100000);
    let stats2 = SimulationStats::new(&results2);

    assert!(stats1.ttk < stats2.ttk);
}
