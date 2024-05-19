use osrs::combat::simulate_n_fights;
use osrs::dps_calc;
use osrs::equipment::CombatType;
use osrs::monster::Monster;
use osrs::player::Player;
use osrs::rolls::{calc_player_magic_rolls, calc_player_melee_rolls, calc_player_ranged_rolls};
use rstest::rstest;
mod fixtures;
use fixtures::*;

#[rstest]
fn test_max_melee_ammonite_crab_ttk(max_melee_player: Player, ammonite_crab: Monster) {
    let mut player = max_melee_player;
    let mut monster = ammonite_crab;
    calc_player_melee_rolls(&mut player, &monster);
    let (ttk, _, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    let dist = dps_calc::get_distribution(&player, &monster);
    let calc_ttk = dps_calc::get_ttk(dist, &player, &monster);
    assert!(num::abs(calc_ttk - ttk) < 0.1);
}
