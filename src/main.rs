#![allow(unused_imports)]
use osrs::calc::analysis::{plot_ttk_cdf, plot_ttk_dist, SimulationStats, TtkUnits};
use osrs::calc::rolls;
use osrs::calc::rolls::calc_active_player_rolls;
use osrs::combat::simulation::simulate_n_fights;
use osrs::types::equipment::{CombatStyle, Weapon};
use osrs::types::monster::{CombatStat, Monster};
use osrs::types::player::{GearSwitch, Player, SwitchType};
use osrs::types::potions::Potion;
use osrs::types::prayers::{Prayer, PrayerBoost};
use osrs::utils::equipment_db;
use osrs::utils::loadouts;
use osrs::utils::logging::FightLogger;
use osrs::utils::monster_db;
// use osrs::rolls::monster_def_rolls;
use osrs::sims::graardor::{GraardorConfig, GraardorFight, GraardorMethod};
use osrs::sims::hunleff::{AttackStrategy, EatStrategy, HunllefConfig, HunllefFight};
use osrs::sims::single_way::SingleWayFight;
use osrs::types::stats::Stat;

fn main() {
    match monster_db::main() {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }

    match equipment_db::main() {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }

    // simulate_door_altar_graardor();

    // simulate_single_way();

    // simulate_hunllef();
}

#[allow(unused)]
fn simulate_single_way() {
    // let mut player = loadouts::max_melee_player();
    let mut player = loadouts::bowfa_crystal_player();
    player.equip("Eclipse moon helm", None);
    player.equip("Eclipse moon chestplate", None);
    player.equip("Eclipse moon tassets", None);
    player.equip("Eclipse atlatl", None);
    player.equip("Atlatl dart", None);
    player.equip("Amulet of strength", None);

    player.equip("Berserker ring (i)", None);
    player.equip("Mixed hide boots", None);
    player.equip("Barrows gloves", None);
    player.equip("Ava's assembler", None);
    player.stats.ranged = Stat::new(90);
    player.stats.strength = Stat::new(90);
    player.update_bonuses();
    player.update_set_effects();
    player.set_active_style(CombatStyle::Rapid);
    player.prayers.add(PrayerBoost::new(Prayer::Deadeye));
    player.add_potion(Potion::SmellingSalts);
    // player.add_potion(Potion::SuperCombat);

    let mut monster = Monster::new("Zebak", None).unwrap();
    monster.drain_stat(CombatStat::Defence, 20, None);
    monster.base_def_rolls = rolls::monster_def_rolls(&monster);
    monster.def_rolls.clone_from(&monster.base_def_rolls);
    monster.info.toa_level = 300;
    monster.info.toa_path_level = 1;
    monster.scale_toa();

    calc_active_player_rolls(&mut player, &monster);
    println!("Max hit: {}", player.max_hits.get(player.combat_type()));
    println!(
        "Max att roll: {}",
        player.att_rolls.get(player.combat_type())
    );

    let simulation = SingleWayFight::new(player, monster);
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    println!("Ttk: {}", stats.ttk);
    println!("Acc: {}", stats.accuracy);
    println!("Avg. leftover burn: {}", stats.avg_leftover_burn);

    // plot_ttk_dist(&results, TtkUnits::Ticks, true);
    // plot_ttk_cdf(&results, TtkUnits::Ticks, true);
}

#[allow(unused)]
fn simulate_hunllef() {
    let mut player = Player::new();
    player.stats.ranged = Stat::new(92);
    player.stats.magic = Stat::new(92);
    player.stats.defence = Stat::new(75);
    player.stats.hitpoints = Stat::new(85);
    player.stats.attack = Stat::new(78);
    player.stats.strength = Stat::new(85);
    player.reset_current_stats();
    player.equip("Corrupted staff (perfected)", None);
    player.equip("Crystal helm (attuned)", None);
    player.equip("Crystal body (attuned)", None);
    player.equip("Crystal legs (attuned)", None);
    player.update_bonuses();
    player.set_active_style(CombatStyle::Accurate);
    player.prayers.add(PrayerBoost::new(Prayer::Augury));
    // player.prayers.add(PrayerBoost::new(Prayer::SteelSkin));

    let hunllef = Monster::new("Corrupted Hunllef", None).unwrap();
    calc_active_player_rolls(&mut player, &hunllef);

    let mage_switch = GearSwitch::from(&player);

    player.equip("Corrupted bow (perfected)", None);
    // player.equip("Corrupted bow (attuned)", None);
    player.update_bonuses();
    player.set_active_style(CombatStyle::Rapid);
    player.prayers.add(PrayerBoost::new(Prayer::Rigour));

    calc_active_player_rolls(&mut player, &hunllef);

    let ranged_switch = GearSwitch::from(&player);

    // player.gear.weapon = Weapon::default();
    // player.set_active_style(CombatStyle::Kick);
    player.equip("Corrupted halberd (perfected)", None);
    player.set_active_style(CombatStyle::Jab);
    player.update_bonuses();
    player.prayers.add(PrayerBoost::new(Prayer::Piety));

    calc_active_player_rolls(&mut player, &hunllef);

    let melee_switch = GearSwitch::from(&player);
    player.switches.push(mage_switch);
    player.switches.push(ranged_switch);
    player.switches.push(melee_switch);

    player.switch(SwitchType::Ranged);

    let fight_config = HunllefConfig {
        food_count: 30,
        eat_strategy: EatStrategy::EatAtHp(65),
        redemption_attempts: 0,
        attack_strategy: AttackStrategy::TwoT3Weapons {
            style1: SwitchType::Melee,
            style2: SwitchType::Ranged,
        },
        lost_ticks: 0,
        logger: FightLogger::new(false, "hunllef"),
    };
    // let fight_config = HunllefConfig {
    //     food_count: 24,
    //     eat_strategy: EatStrategy::EatAtHp(15),
    //     redemption_attempts: 0,
    //     attack_strategy: AttackStrategy::FiveToOne {
    //         main_style: SwitchType::Magic,
    //         other_style1: SwitchType::Ranged,
    //         other_style2: SwitchType::Melee,
    //     },
    //     lost_ticks: 0,
    //     logger: FightLogger::new(false, "hunllef"),
    // };

    let fight = HunllefFight::new(player, fight_config);
    let results = simulate_n_fights(Box::new(fight), 1000000);
    let stats = SimulationStats::new(&results);

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

    // plot_ttk_dist(&results, TtkUnits::Seconds, true);
    // plot_ttk_cdf(&results, TtkUnits::Seconds, true);
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

    let results = simulate_n_fights(Box::new(fight), 1000000);
    let stats = SimulationStats::new(&results);

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
