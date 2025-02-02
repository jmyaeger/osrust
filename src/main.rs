use osrs::combat::simulation::simulate_n_fights;
use osrs::types::equipment::CombatStyle;
// use osrs::equipment_db;
use osrs::types::monster::Monster;
use osrs::utils::loadouts;
use osrs::utils::logging::FightLogger;
// use osrs::monster_db;
use osrs::calc::rolls::calc_active_player_rolls;
use osrs::types::player::{GearSwitch, Player, SwitchType};
use osrs::types::potions::Potion;
use osrs::types::prayers::{Prayer, PrayerBoost};
// use osrs::rolls::monster_def_rolls;
use osrs::sims::graardor::{GraardorConfig, GraardorFight, GraardorMethod};
use osrs::sims::hunleff::{AttackStrategy, EatStrategy, HunllefConfig, HunllefFight};
use osrs::sims::single_way::SingleWayFight;
use osrs::types::stats::Stat;

fn main() {
    // match monster_db::main() {
    //     Ok(_) => {}
    //     Err(e) => println!("{}", e),
    // }

    // match equipment_db::main() {
    //     Ok(_) => {}
    //     Err(e) => println!("{}", e),
    // }

    // simulate_door_altar_graardor();

    // simulate_single_way();

    simulate_hunllef();
}

#[allow(unused)]
fn simulate_single_way() {
    let mut player = loadouts::max_melee_player();
    player.equip("Eclipse atlatl", None);
    player.equip("Eclipse moon helm", None);
    player.equip("Eclipse moon chestplate", None);
    player.equip("Eclipse moon tassets", None);
    player.equip("Atlatl dart", None);
    player.equip("Dizana's quiver", Some("Uncharged"));
    player.update_bonuses();
    player.update_set_effects();
    player.set_active_style(CombatStyle::Rapid);
    player.prayers.add(PrayerBoost::new(Prayer::Rigour));
    player.add_potion(Potion::Ranging);

    player.set_effects.full_eclipse_moon = false;

    let monster = Monster::new("Ammonite Crab", None).unwrap();
    // monster.bonuses.defence.standard = -63;
    // monster.def_rolls = monster_def_rolls(&monster);
    // monster.stats.hitpoints = 200;
    // monster.info.toa_level = 300;
    // monster.scale_toa();

    calc_active_player_rolls(&mut player, &monster);
    println!("Max hit: {}", player.max_hits.get(player.combat_type()));
    println!(
        "Max att roll: {}",
        player.att_rolls.get(player.combat_type())
    );

    let simulation = SingleWayFight::new(player, monster);
    let stats = simulate_n_fights(Box::new(simulation), 1000000);

    println!("Ttk: {}", stats.ttk);
    println!("Acc: {}", stats.accuracy);
    println!("Avg. leftover burn damage: {}", stats.avg_leftover_burn)
}

#[allow(unused)]
fn simulate_hunllef() {
    let mut player = Player::new();
    player.lookup_stats("IM Orphion");
    // player.stats.ranged = Stat::new(92);
    // player.stats.magic = Stat::new(84);
    // player.stats.defence = Stat::new(1);
    // player.stats.hitpoints = Stat::new(90);
    // player.stats.attack = Stat::new(75);
    // player.stats.strength = Stat::new(90);
    player.reset_current_stats();
    player.equip("Corrupted staff (perfected)", None);
    player.equip("Crystal helm (basic)", None);
    player.equip("Crystal body (basic)", None);
    player.equip("Crystal legs (basic)", None);
    player.update_bonuses();
    player.set_active_style(CombatStyle::Accurate);
    player.prayers.add(PrayerBoost::new(Prayer::MysticMight));
    player.prayers.add(PrayerBoost::new(Prayer::SteelSkin));

    let hunllef = Monster::new("Corrupted Hunllef", None).unwrap();
    calc_active_player_rolls(&mut player, &hunllef);

    let mage_switch = GearSwitch::from(&player);

    player.equip("Corrupted bow (perfected)", None);
    player.update_bonuses();
    player.set_active_style(CombatStyle::Rapid);
    player.prayers.add(PrayerBoost::new(Prayer::EagleEye));

    calc_active_player_rolls(&mut player, &hunllef);

    let ranged_switch = GearSwitch::from(&player);

    player.equip("Corrupted halberd (perfected)", None);
    player.update_bonuses();
    player.set_active_style(CombatStyle::Jab);
    player.prayers.add(PrayerBoost::new(Prayer::Piety));

    calc_active_player_rolls(&mut player, &hunllef);

    let melee_switch = GearSwitch::from(&player);
    player.switches.push(mage_switch);
    player.switches.push(ranged_switch);
    player.switches.push(melee_switch);

    player.switch(SwitchType::Ranged);

    let fight_config = HunllefConfig {
        food_count: 24,
        eat_strategy: EatStrategy::EatAtHp(68),
        redemption_attempts: 0,
        attack_strategy: AttackStrategy::TwoT3Weapons {
            style1: SwitchType::Ranged,
            style2: SwitchType::Magic,
        },
        lost_ticks: 50,
        logger: FightLogger::new(false, "hunllef"),
    };

    let fight = HunllefFight::new(player, fight_config);
    let stats = simulate_n_fights(Box::new(fight), 100000);
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

#[allow(unused)]
fn simulate_door_altar_graardor() {
    let mut player = loadouts::bowfa_crystal_player();
    player.stats.ranged = Stat::new(87);
    player.stats.defence = Stat::new(80);
    player.reset_current_stats();
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
        logger: FightLogger::new(false, "graardor"),
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
