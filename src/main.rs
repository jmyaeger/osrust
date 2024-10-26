use osrs::combat::simulate_n_fights;
use osrs::equipment::CombatStyle;
use osrs::equipment_db;
use osrs::loadouts;
use osrs::monster::Monster;
use osrs::monster_db;
use osrs::prayers::{Prayer, PrayerBoost};
use osrs::rolls::calc_active_player_rolls;
use osrs::sims::graardor::{GraardorConfig, GraardorFight, GraardorMethod};
use osrs::sims::single_way::SingleWayFight;

fn main() {
    // match monster_db::main() {
    //     Ok(_) => {}
    //     Err(e) => println!("{}", e),
    // }

    match equipment_db::main() {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }

    // simulate_door_altar_graardor();
}

fn simulate_single_way() {
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

fn simulate_door_altar_graardor() {
    let mut player = loadouts::bowfa_crystal_player();
    player.stats.ranged = 87;
    player.stats.defence = 80;
    player.reset_live_stats();
    player.prayers.add(PrayerBoost::new(Prayer::EagleEye));
    player.prayers.add(PrayerBoost::new(Prayer::SteelSkin));
    player.equip("Barrows gloves", None);
    player.equip("Zamorak d'hide boots", None);
    player.equip("Ava's assembler", None);
    player.equip("Amulet of fury", None);
    // player.equip("Ring of suffering (i)", Some("Uncharged"));
    player.equip("Explorer's ring 4", None);

    player.update_bonuses();

    calc_active_player_rolls(
        &mut player,
        &Monster::new("General Graardor", None).unwrap(),
    );

    let fight_config = GraardorConfig {
        method: GraardorMethod::DoorAltar,
        eat_hp: 20,
        heal_amount: 18,
    };

    let fight = GraardorFight::new(player, fight_config);

    let stats = simulate_n_fights(Box::new(fight), 1000000);

    println!("Average ttk: {:.2} seconds", stats.ttk);
    println!("Average accuracy: {:.2}%", stats.accuracy);
    println!("Success rate: {:.2}%", stats.success_rate * 100.0);
    println!(
        "Average number of food eaten per kill: {:.2}",
        stats.avg_food_eaten
    );
    println!(
        "Average damage taken per kill: {:.2}",
        stats.avg_damage_taken
    );
}
