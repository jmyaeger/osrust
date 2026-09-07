#![allow(unused)]
use osrs::calc::analysis::SimulationStats;
use osrs::calc::rolls::calc_active_player_rolls;
use osrs::combat::simulation::simulate_n_fights;
use osrs::combat::spec::{
    self, CoreCondition, SpecConfig, SpecRestorePolicy, SpecState, SpecStrategy,
};
use osrs::combat::thralls::Thrall;
use osrs::sims::graardor::{GraardorConfig, GraardorFight, GraardorMethod};
use osrs::sims::hunleff::{
    AttackStrategy, HunllefConfig, HunllefEatStrategy, HunllefFight, HunllefRedemptionStrat,
};
use osrs::sims::single_way::{SingleWayConfig, SingleWayFight};
use osrs::sims::vardorvis::{VardorvisConfig, VardorvisEatStrategy, VardorvisFight};
use osrs::types::equipment::{CombatStyle, GearBuilder, GearSlot};
use osrs::types::monster::Monster;
use osrs::types::player::{GearSwitch, Player, PlayerBuilder, SwitchType};
use osrs::types::potions::Potion;
use osrs::types::prayers::Prayer;
use osrs::types::stats::Stat;
use osrs::utils::loadouts;

fn main() {
    let start_time = std::time::Instant::now();
    // simulate_door_altar_graardor();

    simulate_single_way();

    // simulate_hunllef();

    // simulate_normal_gauntlet();

    // simulate_vardorvis();

    let end_time = std::time::Instant::now();

    println!(
        "Total elapsed time: {:.2} seconds",
        (end_time - start_time).as_secs_f64()
    )
}

#[allow(unused)]
fn simulate_single_way() {
    let gear = GearBuilder::new()
        .head("Torva full helm", None)
        .neck("Amulet of torture", None)
        .body("Torva platebody", None)
        .legs("Torva platelegs", None)
        .feet("Primordial boots", None)
        .ring("Ultor ring", None)
        .cape("Infernal cape", None)
        .weapon("Osmumten's fang", None)
        .shield("Avernic defender", None)
        .hands("Ferocious gloves", None)
        .build()
        .expect("Error building gear.");
    let mut player = PlayerBuilder::new()
        .attack(99)
        .strength(99)
        .defence(99)
        .ranged(99)
        .magic(99)
        .prayer(Prayer::Piety)
        .potion(Potion::SuperCombat)
        .gear(gear)
        .active_style(CombatStyle::Lunge)
        .build()
        .expect("Error building player.");

    let mut monster = Monster::new("Vorkath", Some("Post-quest")).expect("Error creating monster.");

    // let single_shield_hp = monster.stats.hitpoints.base;
    // monster.stats.hitpoints = Stat::new(single_shield_hp * 2, None);
    // monster.info.toa_level = 350;
    // monster.info.toa_path_level = 0;
    // monster.scale_toa(true, true);

    calc_active_player_rolls(&mut player, &monster);

    let config = SingleWayConfig {
        thralls: Some(Thrall::GreaterMagic),
        remove_final_attack_delay: false,
        reset_soulreaper_stacks: None,
    };

    let mut main_hand = GearSwitch::new(SwitchType::Melee, &player, &monster);
    player.switches.push(main_hand.clone());

    // let bp_spec_strategy: SpecStrategy<CoreCondition> = SpecStrategy::builder(&main_hand).build();

    // player.equip("Crimson kisten", None).unwrap();
    // player.equip("Avernic defender", None).unwrap();
    // player.set_active_style(CombatStyle::Pummel);
    // let bludgeon_switch = GearSwitch::new(
    //     SwitchType::Spec("Crimson kisten spec".into()),
    //     &player,
    //     &monster,
    // );
    // let bludgeon_spec_strategy: SpecStrategy<CoreCondition> =
    //     SpecStrategy::builder(&bludgeon_switch)
    //         .with_monster_hp_above(100)
    //         .build();
    // player.switches.push(bludgeon_switch);

    player.equip("Voidwaker", None).unwrap();
    player.set_active_style(CombatStyle::Slash);
    let vw_switch = GearSwitch::new(SwitchType::Spec("Voidwaker spec".into()), &player, &monster);
    let vw_spec_strategy: SpecStrategy<CoreCondition> = SpecStrategy::builder(&vw_switch)
        // .with_max_attempts(1)
        .build();
    player.switches.push(vw_switch);

    // player.equip("Dragon warhammer", None).unwrap();
    // player.equip("Avernic defender", None).unwrap();
    // player.set_active_style(CombatStyle::Pound);
    // let dwh_switch = GearSwitch::new(SwitchType::Spec("DWH spec".into()), &player, &monster);
    // let dwh_spec_strategy: SpecStrategy<CoreCondition> = SpecStrategy::builder(&dwh_switch)
    //     .with_max_attempts(1)
    //     .build();
    // player.switches.push(dwh_switch);

    // player.equip("Dragon claws", None).unwrap();
    // player.set_active_style(CombatStyle::Slash);
    // let dclaws_switch = GearSwitch::new(
    //     SwitchType::Spec("Dragon claws spec".into()),
    //     &player,
    //     &monster,
    // );
    // let dclaws_spec_strategy: SpecStrategy<CoreCondition> = SpecStrategy::builder(&dclaws_switch)
    //     .with_max_attempts(1)
    //     // .with_monster_hp_above(100)
    //     .build();
    // player.switches.push(dclaws_switch);

    // player.equip("Burning claws", None).unwrap();
    // player.set_active_style(CombatStyle::Slash);
    // let bclaws_switch = GearSwitch::new(
    //     SwitchType::Spec("Burning claws spec".into()),
    //     &player,
    //     &monster,
    // );
    // let bclaws_spec_strategy: SpecStrategy<CoreCondition> = SpecStrategy::builder(&bclaws_switch)
    //     // .with_max_attempts(2)
    //     // .with_monster_hp_above(50)
    //     // .with_monster_hp_below(50)
    //     .build();
    // player.switches.push(bclaws_switch);

    // player.equip("Bandos godsword", None).unwrap();
    // player.set_active_style(CombatStyle::Slash);
    // let bgs_switch = GearSwitch::new(SwitchType::Spec("BGS spec".into()), &player, &monster);
    // let bgs_spec_strategy: SpecStrategy<CoreCondition> = SpecStrategy::builder(&bgs_switch)
    //     .with_max_attempts(2)
    //     .with_min_successes(1)
    //     .build();
    // player.switches.push(bgs_switch);

    // player.equip("Elder maul", None).unwrap();
    // player.set_active_style(CombatStyle::Pound);
    // let maul_switch = GearSwitch::new(
    //     SwitchType::Spec("Elder maul spec".into()),
    //     &player,
    //     &monster,
    // );
    // let maul_spec_strategy: SpecStrategy<CoreCondition> = SpecStrategy::builder(&maul_switch)
    //     .with_min_successes(1)
    //     // .with_max_attempts(1)
    //     .build();
    // player.switches.push(maul_switch);

    // player.switch(&SwitchType::Melee);
    let spec_config = SpecConfig::new(
        vec![vw_spec_strategy],
        SpecRestorePolicy::NeverRestore,
        None,
        false,
    );

    let simulation = SingleWayFight::new(player, monster, config, Some(spec_config))
        .expect("Error setting up single way fight.");
    let results =
        simulate_n_fights(Box::new(simulation), 1_000_000, true).expect("Simulation failed.");
    let stats = SimulationStats::new(&results);

    println!("Ttk: {:.4} seconds", stats.ttk);
    println!("Acc: {:.4}%", stats.accuracy);
    println!("Avg. leftover burn: {}", stats.avg_leftover_burn);
}

#[allow(unused)]
fn simulate_hunllef() {
    let mut player = Player::new();
    // player.stats.ranged = Stat::new(81, None);
    // player.stats.magic = Stat::new(78, None);
    // player.stats.defence = Stat::new(75, None);
    // player.stats.hitpoints = Stat::new(85, None);
    // player.stats.attack = Stat::new(76, None);
    // player.stats.strength = Stat::new(85, None);
    // player.reset_current_stats(false);
    player.equip("Corrupted staff (perfected)", None).unwrap();
    player.equip("Corrupted helm (basic)", None).unwrap();
    player.equip("Corrupted body (basic)", None).unwrap();
    player.equip("Corrupted legs (basic)", None).unwrap();
    player.update_bonuses();
    player.set_active_style(CombatStyle::Accurate);
    player.add_prayer(Prayer::Augury);
    // player.add_prayer(Prayer::SteelSkin);

    let hunllef = Monster::new("Corrupted Hunllef", None).expect("Error creating monster.");
    calc_active_player_rolls(&mut player, &hunllef);

    let mage_switch = GearSwitch::new(SwitchType::Magic, &player, &hunllef);

    // player.equip("Corrupted bow (perfected)", None).unwrap();
    player.equip("Corrupted bow (attuned)", None).unwrap();
    player.update_bonuses();
    player.set_active_style(CombatStyle::Rapid);
    player.add_prayer(Prayer::Rigour);

    calc_active_player_rolls(&mut player, &hunllef);

    let ranged_switch = GearSwitch::new(SwitchType::Ranged, &player, &hunllef);

    // player.unequip_slot(&GearSlot::Weapon);
    // player.set_active_style(CombatStyle::Kick);
    player.equip("Corrupted sceptre", None).unwrap();
    player.set_active_style(CombatStyle::Pummel);
    // player.equip("Corrupted halberd (perfected)", None).unwrap();
    // player.set_active_style(CombatStyle::Swipe);
    player.update_bonuses();
    player.add_prayer(Prayer::Piety);

    calc_active_player_rolls(&mut player, &hunllef);

    let melee_switch = GearSwitch::new(SwitchType::Melee, &player, &hunllef);
    player.switches.push(mage_switch);
    player.switches.push(ranged_switch);
    player.switches.push(melee_switch);

    player.switch(&SwitchType::Ranged);

    // let fight_config = HunllefConfig {
    //     food_count: 20,
    //     eat_strategy: HunllefEatStrategy::EatAtHp(50),
    //     redemption_strategy: None,
    //     attack_strategy: AttackStrategy::TwoT3Weapons {
    //         style1: SwitchType::Ranged,
    //         style2: SwitchType::Magic,
    //     },
    //     lost_ticks: 0,
    //     logger: FightLogger::new(false, "hunllef").expect("Error initializing logger."),
    //     armor_tier: 0,
    // };
    let fight_config = HunllefConfig {
        food_count: 20,
        eat_strategy: HunllefEatStrategy::EatAtHp(50),
        redemption_strategy: None,
        attack_strategy: AttackStrategy::FiveToOne {
            main_style: SwitchType::Magic,
            other_style1: SwitchType::Ranged,
            other_style2: SwitchType::Melee,
        },
        lost_ticks: 0,
        armor_tier: 0,
        only_success_stats: true,
        crystalline: false,
    };

    let fight = HunllefFight::new(player, fight_config).expect("Error setting up Hunllef fight.");
    let results = simulate_n_fights(Box::new(fight), 1_000_000, true).expect("Simulation failed.");
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
fn simulate_normal_gauntlet() {
    let mut player = Player::new();
    // player.stats.ranged = Stat::new(81, None);
    // player.stats.magic = Stat::new(78, None);
    // player.stats.defence = Stat::new(75, None);
    // player.stats.hitpoints = Stat::new(85, None);
    // player.stats.attack = Stat::new(76, None);
    // player.stats.strength = Stat::new(85, None);
    // player.reset_current_stats(false);
    player.equip("Crystal staff (perfected)", None).unwrap();
    player.equip("Crystal helm (basic)", None).unwrap();
    player.equip("Crystal body (basic)", None).unwrap();
    player.equip("Crystal legs (basic)", None).unwrap();
    player.update_bonuses();
    player.set_active_style(CombatStyle::Accurate);
    player.add_prayer(Prayer::Augury);
    // player.add_prayer(Prayer::SteelSkin);

    let hunllef = Monster::new("Crystalline Hunllef", None).expect("Error creating monster.");
    calc_active_player_rolls(&mut player, &hunllef);

    let mage_switch = GearSwitch::new(SwitchType::Magic, &player, &hunllef);

    // player.equip("Corrupted bow (perfected)", None).unwrap();
    player.equip("Crystal bow (attuned)", None).unwrap();
    player.update_bonuses();
    player.set_active_style(CombatStyle::Rapid);
    player.add_prayer(Prayer::Rigour);

    calc_active_player_rolls(&mut player, &hunllef);

    let ranged_switch = GearSwitch::new(SwitchType::Ranged, &player, &hunllef);

    player.unequip_slot(&GearSlot::Weapon);
    player.set_active_style(CombatStyle::Kick);
    // player.equip("Crystal sceptre", None).unwrap();
    // player.set_active_style(CombatStyle::Pummel);
    // player.equip("Corrupted halberd (perfected)", None).unwrap();
    // player.set_active_style(CombatStyle::Swipe);
    player.update_bonuses();
    player.add_prayer(Prayer::Piety);

    calc_active_player_rolls(&mut player, &hunllef);

    let melee_switch = GearSwitch::new(SwitchType::Melee, &player, &hunllef);
    player.switches.push(mage_switch);
    player.switches.push(ranged_switch);
    player.switches.push(melee_switch);

    player.switch(&SwitchType::Magic);

    // let fight_config = HunllefConfig {
    //     food_count: 20,
    //     eat_strategy: HunllefEatStrategy::EatAtHp(50),
    //     redemption_strategy: None,
    //     attack_strategy: AttackStrategy::TwoT3Weapons {
    //         style1: SwitchType::Ranged,
    //         style2: SwitchType::Magic,
    //     },
    //     lost_ticks: 0,
    //     logger: FightLogger::new(false, "hunllef").expect("Error initializing logger."),
    //     armor_tier: 0,
    // };
    let fight_config = HunllefConfig {
        food_count: 2,
        eat_strategy: HunllefEatStrategy::TickEatOnly,
        redemption_strategy: Some(HunllefRedemptionStrat::BeforeEating(3)),
        attack_strategy: AttackStrategy::FiveToOne {
            main_style: SwitchType::Magic,
            other_style1: SwitchType::Ranged,
            other_style2: SwitchType::Melee,
        },
        lost_ticks: 0,
        armor_tier: 0,
        only_success_stats: true,
        crystalline: true,
    };

    let fight = HunllefFight::new(player, fight_config).expect("Error setting up Hunllef fight.");
    let results = simulate_n_fights(Box::new(fight), 1_000_000, true).expect("Simulation failed.");
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
    let rec_time = 239; // 2:23.40 in ticks
    let prep_time = 100;
    let rec_prob = results
        .ttks_ticks
        .iter()
        .filter(|&&t| t <= rec_time - prep_time)
        .count() as f64
        / results.ttks_ticks.len() as f64;
    println!(
        "Probability of beating rec with {:.1}s prep: {:.4} %",
        (prep_time as f64 * 0.6) as i32,
        rec_prob * 100.0
    );
    let min_time = results.ttks_ticks.iter().min().unwrap_or(&0);
    let max_time = results.ttks_ticks.iter().max().unwrap_or(&0);
    println!("Fastest time: {:.1}s", (*min_time as f64 * 0.6));
    println!("Slowest time: {:.1}s", (*max_time as f64 * 0.6));
}

#[allow(unused)]
fn simulate_vardorvis() {
    let mut player = loadouts::max_melee_player();
    // player.equip("Amulet of torture", None).unwrap();
    // player.equip("Dharok's platebody", None).unwrap();
    // player.equip("Verac's plateskirt", None).unwrap();
    // player
    //     .equip("Ring of suffering (i)", Some("Recoil"))
    //     .unwrap();
    // player.equip("Fire cape", None).unwrap();
    // player.stats.attack = Stat::new(92, None);
    // player.stats.strength = Stat::new(98, None);
    // player.stats.defence = Stat::new(91, None);
    // player.reset_current_stats(false);
    player.equip("Noxious halberd", None).unwrap();
    player.set_active_style(CombatStyle::Swipe);
    // player.equip("Blade of Saeldor (c)", None).unwrap();
    // player.equip("Dragon defender", None).unwrap();
    // player.equip("Bandos chestplate", None).unwrap();
    // player.equip("Bandos tassets", None).unwrap();
    // player.equip("Neitiznot faceguard", None).unwrap();
    player.equip("Oathplate chest", None).unwrap();
    player.equip("Oathplate legs", None).unwrap();
    player.equip("Oathplate helm", None).unwrap();
    player.equip("Berserker ring (i)", None).unwrap();
    // player.equip("Barrows gloves", None).unwrap();
    // player.equip("Dragon boots", None).unwrap();
    // player.equip("Bellator ring", None);
    // player.equip("Soulreaper axe", None).unwrap();
    player.update_bonuses();
    player.update_set_effects();

    let vard = Monster::new("Vardorvis", Some("Post-quest")).expect("Error creating monster.");
    calc_active_player_rolls(&mut player, &vard);

    let mut main_hand = GearSwitch::new(SwitchType::Melee, &player, &vard);
    player.switches.push(main_hand);

    player.equip("Voidwaker", None).unwrap();
    player.equip("Avernic defender", None).unwrap();
    player.set_active_style(CombatStyle::Slash);
    let vw_switch = GearSwitch::new(SwitchType::Spec("Voidwaker spec".into()), &player, &vard);
    let vw_spec_strategy: SpecStrategy<CoreCondition> = SpecStrategy::builder(&vw_switch)
        .with_monster_hp_above(100)
        .not_on_first_attack()
        .build();
    player.switches.push(vw_switch);

    player.equip("Burning claws", None).unwrap();
    player.set_active_style(CombatStyle::Slash);
    let bclaws_switch = GearSwitch::new(
        SwitchType::Spec("Burning claws spec".into()),
        &player,
        &vard,
    );
    let bclaws_spec_strategy: SpecStrategy<CoreCondition> = SpecStrategy::builder(&bclaws_switch)
        .with_monster_hp_below(600)
        .with_monster_hp_above(100)
        .build();
    player.switches.push(bclaws_switch);

    player.equip("Dragon claws", None).unwrap();
    player.set_active_style(CombatStyle::Slash);
    let dclaws_switch =
        GearSwitch::new(SwitchType::Spec("Dragon claws spec".into()), &player, &vard);
    let dclaws_spec_strategy: SpecStrategy<CoreCondition> = SpecStrategy::builder(&dclaws_switch)
        // .with_monster_hp_below(50)
        .with_monster_hp_above(100)
        .build();
    player.switches.push(dclaws_switch);

    player.equip("Dragon dagger", Some("Unpoisoned")).unwrap();
    player.equip("Avernic defender", None).unwrap();
    player.set_active_style(CombatStyle::Stab);
    let dds_switch = GearSwitch::new(
        SwitchType::Spec("Dragon dagger spec".into()),
        &player,
        &vard,
    );
    let dds_spec_strategy: SpecStrategy<CoreCondition> = SpecStrategy::builder(&dds_switch)
        // .with_monster_hp_below(50)
        .with_monster_hp_above(100)
        .build();
    player.switches.push(dds_switch);

    player.equip("Arkan blade", None).unwrap();
    player.set_active_style(CombatStyle::Slash);
    let arkan_switch = GearSwitch::new(SwitchType::Spec("Arkan blade spec".into()), &player, &vard);
    let arkan_spec_strategy: SpecStrategy<CoreCondition> = SpecStrategy::builder(&arkan_switch)
        // .with_monster_hp_below(50)
        .with_monster_hp_above(100)
        .build();
    player.switches.push(arkan_switch);

    player.equip("Crystal halberd", Some("Active")).unwrap();
    player.set_active_style(CombatStyle::Swipe);
    let chally_switch = GearSwitch::new(
        SwitchType::Spec("Crystal halberd spec".into()),
        &player,
        &vard,
    );
    let chally_spec_strategy: SpecStrategy<CoreCondition> = SpecStrategy::builder(&chally_switch)
        .with_monster_hp_below(50)
        // .with_monster_hp_above(100)
        .build();
    player.switches.push(chally_switch);

    player.switch(&SwitchType::Melee);
    let spec_config = SpecConfig::new(
        vec![bclaws_spec_strategy],
        SpecRestorePolicy::RestoreAfter(10),
        Some(osrs::combat::spec::DeathCharge::Double),
        false,
    );

    let fight_config = VardorvisConfig {
        food_heal_amount: 18,
        food_eat_delay: 2,
        eat_strategy: VardorvisEatStrategy::EatAtHp(10),
        thralls: Some(Thrall::GreaterMagic),
        spec_config: Some(spec_config),
        spec_state: SpecState::default(),
    };

    let mut fight =
        VardorvisFight::new(player, fight_config).expect("Error creating the Vardorvis fight.");
    let results = simulate_n_fights(Box::new(fight), 100_000, true).expect("Simulation failed.");
    let stats = SimulationStats::new(&results);

    let mut odds_of_gm = 0.0;
    for (ticks, prob) in stats.ttk_dist.iter().enumerate() {
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
    println!("Probability of hitting GM time: {:.4}%", odds_of_gm * 100.0);
}

#[allow(unused)]
fn simulate_door_altar_graardor() {
    let mut player = loadouts::bowfa_crystal_player();
    player.stats.ranged = Stat::new(87, None);
    player.stats.defence = Stat::new(80, None);
    player.reset_current_stats(false);
    player.add_prayer(Prayer::EagleEye);
    player.add_prayer(Prayer::SteelSkin);
    player.equip("Barrows gloves", None).unwrap();
    player.equip("Zamorak d'hide boots", None).unwrap();
    player.equip("Ava's assembler", None).unwrap();
    player.equip("Amulet of fury", None).unwrap();
    // player.equip("Ring of suffering (i)", Some("Uncharged")).unwrap();
    player.equip("Explorer's ring 4", None).unwrap();

    player.update_bonuses();

    calc_active_player_rolls(
        &mut player,
        &Monster::new("General Graardor", None).expect("Error creating monster."),
    );

    let fight_config = GraardorConfig {
        method: GraardorMethod::DoorAltar,
        eat_hp: 20,
        heal_amount: 18,
    };

    let fight = GraardorFight::new(player, fight_config).expect("Error setting up Graardor fight.");

    let results = simulate_n_fights(Box::new(fight), 1000000, true).expect("Simulation failed.");
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
