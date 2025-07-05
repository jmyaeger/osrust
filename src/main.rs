#![allow(unused_imports)]
use osrs::calc::analysis::SimulationStats;
use osrs::calc::rolls;
use osrs::calc::rolls::calc_active_player_rolls;
use osrs::combat::attacks::standard::get_attack_functions;
use osrs::combat::simulation::{Simulation, simulate_n_fights};
use osrs::combat::thralls::Thrall;
use osrs::sims::graardor::{GraardorConfig, GraardorFight, GraardorMethod};
use osrs::sims::hunleff::{AttackStrategy, HunllefConfig, HunllefEatStrategy, HunllefFight};
use osrs::sims::single_way::{SingleWayConfig, SingleWayFight};
use osrs::sims::vardorvis::{VardorvisConfig, VardorvisEatStrategy, VardorvisFight};
use osrs::types::equipment::{CombatStyle, Weapon};
use osrs::types::monster::{CombatStat, Monster};
use osrs::types::player::{GearSwitch, Player};
use osrs::types::potions::Potion;
use osrs::types::prayers::Prayer;
use osrs::types::stats::Stat;
use osrs::utils::{equipment_json, loadouts, logging::FightLogger, monster_json};

fn main() {
    // match monster_json::main() {
    //     Ok(_) => {}
    //     Err(e) => println!("{}", e),
    // }

    // match equipment_json::main() {
    //     Ok(_) => {}
    //     Err(e) => println!("{}", e),
    // }

    // simulate_door_altar_graardor();

    simulate_single_way();

    // simulate_hunllef();

    // simulate_vardorvis();
}

#[allow(unused)]
fn simulate_single_way() {
    let mut player = loadouts::max_melee_player();
    // let mut player = loadouts::bowfa_crystal_player();
    // player.equip("Eclipse moon helm", None);
    // player.equip("Eclipse moon chestplate", None);
    // player.equip("Eclipse moon tassets", None);
    // player.equip("Eclipse atlatl", None);
    // player.equip("Atlatl dart", None);
    // player.equip("Amulet of strength", None);

    // player.equip("Berserker ring (i)", None);
    // player.equip("Mixed hide boots", None);
    // player.equip("Barrows gloves", None);
    // player.equip("Ava's assembler", None);
    // player.stats.ranged = Stat::new(90);
    // player.stats.strength = Stat::new(90);
    // player.update_bonuses();
    // player.update_set_effects();
    // player.set_active_style(CombatStyle::Rapid);
    // player.prayers.add(Prayer::Deadeye);
    // player.add_potion(Potion::SmellingSalts);

    let mut monster = Monster::new("Nex", None).unwrap();
    // monster.drain_stat(CombatStat::Defence, 20, None);
    // monster.base_def_rolls = rolls::monster_def_rolls(&monster);
    // monster.def_rolls.clone_from(&monster.base_def_rolls);
    // monster.info.toa_level = 300;
    // monster.info.toa_path_level = 1;
    // monster.scale_toa();

    calc_active_player_rolls(&mut player, &monster);
    println!("Max hit: {}", player.max_hits.get(player.combat_type()));
    println!(
        "Max att roll: {}",
        player.att_rolls.get(player.combat_type())
    );

    let config = SingleWayConfig {
        thralls: Some(Thrall::GreaterMagic),
    };

    let simulation = SingleWayFight::new(player, monster, config, None, false);
    let results = simulate_n_fights(Box::new(simulation), 100000);
    let stats = SimulationStats::new(&results);

    println!("Ttk: {}", stats.ttk);
    println!("Acc: {}", stats.accuracy);
    // println!("Avg. leftover burn: {}", stats.avg_leftover_burn);
}

#[allow(unused)]
fn simulate_hunllef() {
    let mut player = Player::new();
    player.stats.ranged = Stat::new(93);
    player.stats.magic = Stat::new(93);
    player.stats.defence = Stat::new(70);
    player.stats.hitpoints = Stat::new(90);
    player.stats.attack = Stat::new(80);
    player.stats.strength = Stat::new(80);
    player.reset_current_stats();
    player.equip("Corrupted staff (perfected)", None);
    player.equip("Crystal helm (basic)", None);
    player.equip("Crystal body (basic)", None);
    player.equip("Crystal legs (basic)", None);
    player.update_bonuses();
    player.set_active_style(CombatStyle::Accurate);
    player.prayers.add(Prayer::MysticVigour);
    player.prayers.add(Prayer::SteelSkin);

    let hunllef = Monster::new("Corrupted Hunllef", None).unwrap();
    calc_active_player_rolls(&mut player, &hunllef);

    let mage_switch = GearSwitch::from(&player);

    player.equip("Corrupted bow (perfected)", None);
    // player.equip("Corrupted bow (attuned)", None);
    player.update_bonuses();
    player.set_active_style(CombatStyle::Rapid);
    player.prayers.add(Prayer::Deadeye);

    calc_active_player_rolls(&mut player, &hunllef);

    let ranged_switch = GearSwitch::from(&player);

    // player.gear.weapon = Weapon::default();
    // player.set_active_style(CombatStyle::Kick);
    player.equip("Corrupted halberd (perfected)", None);
    player.set_active_style(CombatStyle::Swipe);
    player.update_bonuses();
    player.prayers.add(Prayer::Piety);

    calc_active_player_rolls(&mut player, &hunllef);

    let melee_switch = GearSwitch::from(&player);
    player.switches.push(mage_switch);
    player.switches.push(ranged_switch);
    player.switches.push(melee_switch);

    player.switch(&"Ranged".to_string());

    let fight_config = HunllefConfig {
        food_count: 25,
        eat_strategy: HunllefEatStrategy::EatAtHp(69),
        redemption_attempts: 0,
        attack_strategy: AttackStrategy::TwoT3Weapons {
            style1: "Magic".to_string(),
            style2: "Ranged".to_string(),
        },
        lost_ticks: 0,
        logger: FightLogger::new(false, "hunllef"),
    };
    // let fight_config = HunllefConfig {
    //     food_count: 24,
    //     eat_strategy: HunllefEatStrategy::EatAtHp(15),
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
}

#[allow(unused)]
fn simulate_vardorvis() {
    let mut player = loadouts::max_melee_player();
    player.equip("Abyssal tentacle", None);
    player.equip("Dragon defender", None);
    // player.equip("Amulet of blood fury", None);
    // player.equip("Bellator ring", None);
    player.equip("Oathplate chest", None);
    player.equip("Oathplate legs", None);
    player.equip("Oathplate helm", None);
    player.equip("Berserker ring (i)", None);
    // player.equip("Justiciar chestguard", None);
    // player.equip("Justiciar legguards", None);
    // player.equip("Justiciar faceguard", None);
    // player.equip("Ring of suffering (i)", Some("Recoil"));
    player.equip("Echo boots", None);
    player.update_bonuses();
    player.update_set_effects();
    player.set_active_style(CombatStyle::Lash);

    let vard = Monster::new("Vardorvis", Some("Post-quest")).unwrap();
    calc_active_player_rolls(&mut player, &vard);

    let fight_config = VardorvisConfig {
        food_heal_amount: 22,
        food_eat_delay: 3,
        eat_strategy: VardorvisEatStrategy::EatAtHp(20),
        logger: FightLogger::new(false, "vardorvis"),
    };

    let mut fight = VardorvisFight::new(player, fight_config);
    let results = simulate_n_fights(Box::new(fight), 100000);
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

#[allow(unused)]
fn simulate_door_altar_graardor() {
    let mut player = loadouts::bowfa_crystal_player();
    player.stats.ranged = Stat::new(87);
    player.stats.defence = Stat::new(80);
    player.reset_current_stats();
    player.prayers.add(Prayer::EagleEye);
    player.prayers.add(Prayer::SteelSkin);
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
