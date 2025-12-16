#![allow(unused)]
use std::rc::Rc;

use osrs::calc::analysis::SimulationStats;
use osrs::calc::rolls;
use osrs::calc::rolls::calc_active_player_rolls;
use osrs::combat::attacks::standard::get_attack_functions;
use osrs::combat::simulation::{Simulation, simulate_n_fights};
use osrs::combat::thralls::Thrall;
use osrs::sims::graardor::{GraardorConfig, GraardorFight, GraardorMethod};
use osrs::sims::hunleff::{AttackStrategy, HunllefConfig, HunllefEatStrategy, HunllefFight};
use osrs::sims::single_way::{
    DeathCharge, SingleWayConfig, SingleWayFight, SpecCondition, SpecConfig, SpecRestorePolicy,
    SpecStrategy, SpecStrategyState,
};
use osrs::sims::vardorvis::{VardorvisConfig, VardorvisEatStrategy, VardorvisFight};
use osrs::types::equipment::{CombatStyle, Weapon};
use osrs::types::monster::{CombatStat, Monster};
use osrs::types::player::{GearSwitch, Player, SwitchType};
use osrs::types::potions::Potion;
use osrs::types::prayers::Prayer;
use osrs::types::stats::Stat;
use osrs::utils::{loadouts, logging::FightLogger};

fn main() {
    let start_time = std::time::Instant::now();
    // simulate_door_altar_graardor();

    // simulate_single_way();

    simulate_hunllef();

    // simulate_vardorvis();

    let end_time = std::time::Instant::now();

    println!(
        "Total elapsed time: {:.2} seconds",
        (end_time - start_time).as_secs_f64()
    )
}

#[allow(unused)]
fn simulate_single_way() {
    let mut player = loadouts::max_melee_player();
    player.equip("Avernic treads (max)", None);
    player.equip("Oathplate helm", None);
    player.equip("Oathplate chest", None);
    player.equip("Oathplate legs", None);
    // player.equip("Neitiznot faceguard", None);
    // player.equip("Bandos chestplate", None);
    // player.equip("Bandos tassets", None);
    player.equip("Scythe of vitur", Some("Charged"));
    // player.equip("Lightbearer", None);
    player.add_potion(Potion::OverloadPlus);

    player.update_bonuses();
    player.update_set_effects();
    player.set_active_style(CombatStyle::Chop);

    let mut monster = Monster::new("Great Olm", Some("Left claw")).unwrap();
    // let single_shield_hp = monster.stats.hitpoints.base;
    // monster.stats.hitpoints = Stat::new(single_shield_hp * 3, None);
    // monster.info.toa_level = 400;
    // monster.info.toa_path_level = 0;
    // monster.scale_toa();

    calc_active_player_rolls(&mut player, &monster);
    // println!("Max hit: {}", player.max_hits.get(player.combat_type()));
    // println!(
    //     "Max att roll: {}",
    //     player.att_rolls.get(player.combat_type())
    // );

    let config = SingleWayConfig {
        thralls: Some(Thrall::GreaterMelee),
        remove_final_attack_delay: true,
    };

    let mut main_hand = GearSwitch::from(&player);
    player.switches.push(main_hand);

    // player.equip("Voidwaker", None);
    // player.set_active_style(CombatStyle::Slash);
    // let vw_switch = GearSwitch::new(
    //     SwitchType::Spec("Voidwaker spec".to_string()),
    //     &player,
    //     &monster,
    // );
    // let vw_spec_strategy = SpecStrategy::new(&vw_switch, None);
    // player.switches.push(vw_switch);

    // player.equip("Dragon warhammer", None);
    // player.set_active_style(CombatStyle::Pound);
    // let dwh_switch = GearSwitch::new(SwitchType::Spec("DWH spec".to_string()), &player, &monster);
    // let dwh_spec_strategy = SpecStrategy::builder(&dwh_switch)
    //     .with_max_attempts(1)
    //     .build();
    // player.switches.push(dwh_switch);

    player.equip("Dragon claws", None);
    player.set_active_style(CombatStyle::Slash);
    let dclaws_switch = GearSwitch::new(
        SwitchType::Spec("Dragon claws spec".into()),
        &player,
        &monster,
    );
    let dclaws_spec_strategy = SpecStrategy::builder(&dclaws_switch)
        .with_max_attempts(1)
        .build();
    player.switches.push(dclaws_switch);

    player.equip("Burning claws", None);
    player.set_active_style(CombatStyle::Slash);
    let bclaws_switch = GearSwitch::new(
        SwitchType::Spec("Burning claws spec".into()),
        &player,
        &monster,
    );
    let bclaws_spec_strategy = SpecStrategy::builder(&bclaws_switch)
        .with_max_attempts(1)
        .build();
    player.switches.push(bclaws_switch);

    player.equip("Bandos godsword", None);
    player.set_active_style(CombatStyle::Slash);
    let bgs_switch = GearSwitch::new(SwitchType::Spec("BGS spec".into()), &player, &monster);
    let bgs_spec_strategy = SpecStrategy::builder(&bgs_switch)
        .with_target_def_reduction(20)
        .with_max_attempts(2)
        .build();
    player.switches.push(bgs_switch);

    player.equip("Elder maul", None);
    player.set_active_style(CombatStyle::Pound);
    let maul_switch = GearSwitch::new(
        SwitchType::Spec("Elder maul spec".into()),
        &player,
        &monster,
    );
    let maul_spec_strategy = SpecStrategy::builder(&maul_switch)
        .with_max_attempts(1)
        .build();
    player.switches.push(maul_switch);

    player.switch(&SwitchType::Melee);
    let spec_config = SpecConfig::new(
        vec![maul_spec_strategy],
        SpecRestorePolicy::RestoreEveryKill,
        None,
        false,
    );

    let simulation = SingleWayFight::new(player, monster, config, Some(spec_config), false);
    let results = simulate_n_fights(Box::new(simulation), 1_000_000);
    let stats = SimulationStats::new(&results);

    println!("Ttk: {}", stats.ttk);
    println!("Acc: {}", stats.accuracy);
    // println!("Avg. leftover burn: {}", stats.avg_leftover_burn);
}

#[allow(unused)]
fn simulate_hunllef() {
    let mut player = Player::new();
    player.stats.ranged = Stat::new(92, None);
    player.stats.magic = Stat::new(92, None);
    player.stats.defence = Stat::new(75, None);
    player.stats.hitpoints = Stat::new(85, None);
    player.stats.attack = Stat::new(78, None);
    player.stats.strength = Stat::new(86, None);
    player.reset_current_stats(false);
    player.equip("Corrupted staff (perfected)", None);
    player.equip("Crystal helm (basic)", None);
    player.equip("Crystal body (basic)", None);
    player.equip("Crystal legs (basic)", None);
    player.update_bonuses();
    player.set_active_style(CombatStyle::Accurate);
    player.add_prayer(Prayer::MysticMight);
    // player.add_prayer(Prayer::SteelSkin);

    let hunllef = Monster::new("Corrupted Hunllef", None).unwrap();
    calc_active_player_rolls(&mut player, &hunllef);

    let mage_switch = GearSwitch::from(&player);

    player.equip("Corrupted bow (perfected)", None);
    // player.equip("Corrupted bow (attuned)", None);
    player.update_bonuses();
    player.set_active_style(CombatStyle::Rapid);
    player.add_prayer(Prayer::EagleEye);

    calc_active_player_rolls(&mut player, &hunllef);

    let ranged_switch = GearSwitch::from(&player);

    // player.gear.weapon = Weapon::default();
    // player.set_active_style(CombatStyle::Kick);
    player.equip("Corrupted halberd (perfected)", None);
    player.set_active_style(CombatStyle::Swipe);
    player.update_bonuses();
    player.add_prayer(Prayer::Piety);

    calc_active_player_rolls(&mut player, &hunllef);

    let melee_switch = GearSwitch::from(&player);
    player.switches.push(mage_switch);
    player.switches.push(ranged_switch);
    player.switches.push(melee_switch);

    player.switch(&SwitchType::Ranged);

    let fight_config = HunllefConfig {
        food_count: 30,
        eat_strategy: HunllefEatStrategy::EatAtHp(64),
        redemption_attempts: 0,
        attack_strategy: AttackStrategy::TwoT3Weapons {
            style1: SwitchType::Magic,
            style2: SwitchType::Ranged,
        },
        lost_ticks: 0,
        logger: FightLogger::new(false, "hunllef"),
        armor_tier: 0,
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
    let results = simulate_n_fights(Box::new(fight), 1_000_000);
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
    // player.stats.attack = Stat::new(92, None);
    // player.stats.strength = Stat::new(98, None);
    // player.stats.defence = Stat::new(91, None);
    // player.reset_current_stats(false);
    player.equip("Soulreaper axe", None);
    // player.equip("Blade of saeldor (c)", None);
    // player.equip("Dragon defender", None);
    player.equip("Oathplate chest", None);
    player.equip("Oathplate legs", None);
    player.equip("Oathplate helm", None);
    // player.equip("Berserker ring (i)", None);
    // player.equip("Barrows gloves", None);
    // player.equip("Dragon boots", None);
    player.equip("Bellator ring", None);
    player.equip("Avernic treads (max)", None);
    player.update_bonuses();
    player.update_set_effects();
    player.set_active_style(CombatStyle::Hack);

    let vard = Monster::new("Vardorvis", Some("Post-quest")).unwrap();
    calc_active_player_rolls(&mut player, &vard);

    let fight_config = VardorvisConfig {
        food_heal_amount: 22,
        food_eat_delay: 3,
        eat_strategy: VardorvisEatStrategy::EatAtHp(10),
        thralls: Some(Thrall::GreaterMagic),
        logger: FightLogger::new(false, "vardorvis"),
    };

    let mut fight = VardorvisFight::new(player, fight_config);
    let results = simulate_n_fights(Box::new(fight), 1_000_000);
    let stats = SimulationStats::new(&results);

    let mut odds_of_gm = 0.0;
    for (ticks, prob) in stats.ttk_dist {
        if ticks < 92 {
            odds_of_gm += prob;
        }
    }

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
    println!("Probability of hitting GM time: {odds_of_gm}");
}

#[allow(unused)]
fn simulate_door_altar_graardor() {
    let mut player = loadouts::bowfa_crystal_player();
    player.stats.ranged = Stat::new(87, None);
    player.stats.defence = Stat::new(80, None);
    player.reset_current_stats(false);
    player.add_prayer(Prayer::EagleEye);
    player.add_prayer(Prayer::SteelSkin);
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
