use osrs::combat::simulate_n_fights;
use osrs::equipment::CombatStyle;
// use osrs::equipment_db;
use osrs::loadouts;
use osrs::monster::Monster;
// use osrs::monster_db;
use osrs::rolls::calc_active_player_rolls;
use osrs::sims::single_way::SingleWayFight;

fn main() {
    // match monster_db::main() {
    //     Ok(_) => {}
    //     Err(e) => println!("{}", e),
    // }

    // match equipment_db::main() {
    //     Ok(_) => {}
    //     Err(e) => println!("{}", e),
    // }

    simulate();
}

fn simulate() {
    let mut player = loadouts::max_melee_player();
    player.equip("Dragon hunter lance", None);
    player.equip("Slayer helmet (i)", None);
    player.equip("Bandos chestplate", None);
    player.equip("Bandos tassets", None);

    player.update_bonuses();
    player.set_active_style(CombatStyle::Lunge);

    let monster = Monster::new("Alchemical Hydra", Some("Fire")).unwrap();
    // monster.info.toa_level = 300;
    // monster.scale_toa();

    calc_active_player_rolls(&mut player, &monster);

    let simulation = SingleWayFight::new(player, monster);
    let stats = simulate_n_fights(Box::new(simulation), 1000000);

    println!("Ttk: {}", stats.ttk);
    println!("Acc: {}", stats.accuracy);
}
