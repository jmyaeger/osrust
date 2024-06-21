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
    player.prayers.add(PrayerBoost::new(Prayer::Rigour));
    player.add_potion(Potion::SuperCombat);
    player.add_potion(Potion::Ranging);

    // player.equip("Blood moon helm", None);
    // player.equip("Blood moon chestplate", None);
    // player.equip("Blood moon tassets", None);
    // player.equip("Ferocious gloves", None);
    // player.equip("Primordial boots", None);
    // player.equip("Dual macuahuitl", None);
    // player.equip("Rada's blessing 4", None);
    // player.equip("Amulet of torture", None);
    // player.equip("Infernal cape", None);
    // player.equip("Ultor ring", None);

    player.equip("Eclipse moon helm", None);
    player.equip("Eclipse moon chestplate", None);
    player.equip("Eclipse moon tassets", None);
    player.equip("Barrows gloves", None);
    player.equip("Primordial boots", None);
    player.equip("Eclipse atlatl", None);
    player.equip("Atlatl dart", None);
    player.equip("Amulet of fury", None);
    player.equip("Ava's assembler", None);
    player.equip("Ultor ring", None);

    player.update_bonuses();
    player.update_set_effects();
    // player.set_active_style(CombatStyle::Lunge);
    player.set_active_style(CombatStyle::Rapid);
    let mut monster = Monster::new("Kephri", Some("Shielded")).unwrap();
    // monster.info.toa_level = 300;
    // monster.scale_toa();

    calc_active_player_rolls(&mut player, &monster);
    let (ttk, acc, _hit_dist) = simulate_n_fights(&mut player, &mut monster, 1000000);

    println!("Ttk: {}", ttk);
    println!("Acc: {}", acc);
}
