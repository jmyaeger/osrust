use osrs::combat::simulate_n_fights;
use osrs::equipment::CombatStyle;
use osrs::equipment_db;
use osrs::monster::Monster;
use osrs::player::{Player, PlayerStats};
use osrs::potions::Potion;
use osrs::prayers::{Prayer, PrayerBoost};
use osrs::rolls::calc_active_player_rolls;
// use osrs::monster_db;

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
    let mut player = Player::new();
    player.stats = PlayerStats::default();
    player.prayers.add(PrayerBoost::new(Prayer::Piety));
    player.add_potion(Potion::SuperCombat);

    player.equip("Blood moon helm", None);
    player.equip("Blood moon chestplate", None);
    player.equip("Blood moon tassets", None);
    player.equip("Barrows gloves", None);
    player.equip("Dragon boots", None);
    player.equip("Dual macuahuitl", None);
    player.equip("Rada's blessing 4", None);
    player.equip("Amulet of fury", None);
    player.equip("Fire cape", None);
    player.equip("Berserker ring (i)", None);

    // player.equip("Blood moon helm", None);
    // player.equip("Blood moon chestplate", None);
    // player.equip("Blood moon tassets", None);
    // player.equip("Barrows gloves", None);
    // player.equip("Dragon boots", None);
    // player.equip("Zamorakian hasta", None);
    // player.equip("Dragon defender", None);
    // player.equip("Rada's blessing 4", None);
    // player.equip("Amulet of fury", None);
    // player.equip("Fire cape", None);
    // player.equip("Berserker ring (i)", None);

    player.update_bonuses();
    player.update_set_effects();
    // player.set_active_style(CombatStyle::Lunge);
    player.set_active_style(CombatStyle::Spike);
    let mut monster = Monster::new("Ba-Ba", None).unwrap();
    monster.info.toa_level = 300;
    monster.scale_toa();

    calc_active_player_rolls(&mut player, &monster);
    let (ttk, acc, _) = simulate_n_fights(&mut player, &mut monster, 100000);

    println!("Ttk: {}", ttk);
    println!("Acc: {}", acc);
}
