#[cfg(test)]
mod spec_tests {
    use osrs::combat::simulation::Simulation;
    use osrs::combat::spec::{
        CoreCondition, SpecCondition, SpecConfig, SpecRestorePolicy, SpecStrategy,
    };
    use osrs::sims::single_way::*;
    use osrs::types::equipment::CombatStyle;
    use osrs::types::monster::{CombatStat, Monster};
    use osrs::types::player::SwitchType;
    use osrs::types::player::{GearSwitch, Player};
    use osrs::types::potions::Potion;
    use osrs::types::prayers::Prayer;
    use osrs::types::stats::PlayerStats;

    fn create_test_player() -> Player {
        let mut player = Player::new();
        player.stats = PlayerStats::default();
        player.add_prayer(Prayer::Piety);
        player.add_potion(Potion::SuperCombat);

        player.equip("Bandos chestplate", None).unwrap();
        player.equip("Bandos tassets", None).unwrap();
        player.equip("Dragon defender", None).unwrap();
        player.equip("Osmumten's fang", None).unwrap();
        player.update_bonuses();
        player.set_active_style(CombatStyle::Lunge);

        player
    }

    #[test]
    fn test_spec_strategy_creation() {
        let player = create_test_player();
        let monster = Monster::new("General Graardor", None).expect("Error creating monster.");
        let switch = GearSwitch::new(SwitchType::Custom("Test spec".into()), &player, &monster);

        let strategy: SpecStrategy<CoreCondition> = SpecStrategy::new(&switch, None);

        assert_eq!(strategy.switch_type.label(), "Test spec");
        assert_eq!(strategy.spec_cost, 25);
        assert!(strategy.conditions.is_empty());
        assert_eq!(strategy.state.attempt_count, 0);
        assert_eq!(strategy.state.success_count, 0);
    }

    #[test]
    fn test_spec_config_lowest_cost() {
        let player = create_test_player();
        let monster = Monster::new("General Graardor", None).expect("Error creating monster.");

        let fang_switch =
            GearSwitch::new(SwitchType::Custom("Fang spec".into()), &player, &monster);
        let fang_strategy: SpecStrategy<CoreCondition> = SpecStrategy::new(&fang_switch, None);

        let mut player2 = player.clone();
        player2.equip("Dragon claws", None).unwrap();
        let claw_switch =
            GearSwitch::new(SwitchType::Custom("Claw spec".into()), &player2, &monster);
        let claw_strategy = SpecStrategy::new(&claw_switch, None);

        let config = SpecConfig::new(
            vec![fang_strategy, claw_strategy],
            SpecRestorePolicy::RestoreEveryKill,
            None,
            false,
        );

        assert_eq!(config.lowest_cost(), Some(25));
    }

    #[test]
    fn test_max_attempts() {
        let player = create_test_player();
        let monster = Monster::new("General Graardor", None).expect("Error creating monster.");

        let switch = GearSwitch::new(SwitchType::Custom("Test spec".into()), &player, &monster);
        let mut strategy: SpecStrategy<CoreCondition> = SpecStrategy::builder(&switch)
            .with_max_attempts(2)
            .build();

        // Should allow first two attempts
        assert!(strategy.can_execute(&player, &monster, &()));
        strategy.state.attempt_count = 1;
        assert!(strategy.can_execute(&player, &monster, &()));
        strategy.state.attempt_count = 2;
        assert!(!strategy.can_execute(&player, &monster, &()));
    }

    #[test]
    fn test_min_successes() {
        let player = create_test_player();
        let monster = Monster::new("General Graardor", None).expect("Error creating monster.");

        let switch = GearSwitch::new(SwitchType::Custom("Test spec".into()), &player, &monster);
        let mut strategy: SpecStrategy<CoreCondition> = SpecStrategy::builder(&switch)
            .with_min_successes(2)
            .build();

        assert!(strategy.can_execute(&player, &monster, &()));
        strategy.state.success_count = 1;
        assert!(strategy.can_execute(&player, &monster, &()));
        strategy.state.success_count = 2;
        assert!(!strategy.can_execute(&player, &monster, &()));
    }

    #[test]
    fn test_hp_conditions() {
        let player = create_test_player();
        let mut monster = Monster::new("General Graardor", None).expect("Error creating monster.");
        monster.stats.hitpoints.current = 100;

        let switch = GearSwitch::new(SwitchType::Custom("Test spec".into()), &player, &monster);
        let strategy: SpecStrategy<CoreCondition> = SpecStrategy::builder(&switch)
            .with_monster_hp_below(50)
            .build();

        assert!(!strategy.can_execute(&player, &monster, &())); // HP is 100, not below 50

        monster.stats.hitpoints.current = 50;
        assert!(strategy.can_execute(&player, &monster, &())); // HP is 50, equals threshold

        monster.stats.hitpoints.current = 49;
        assert!(strategy.can_execute(&player, &monster, &())); // HP is 49, below threshold
    }

    #[test]
    fn test_hp_above_condition() {
        let player = create_test_player();
        let mut monster = Monster::new("General Graardor", None).expect("Error creating monster.");

        let switch = GearSwitch::new(SwitchType::Custom("Test spec".into()), &player, &monster);
        let strategy: SpecStrategy<CoreCondition> = SpecStrategy::builder(&switch)
            .with_monster_hp_above(100)
            .build();

        assert!(strategy.can_execute(&player, &monster, &())); // HP is 255, above 100

        monster.stats.hitpoints.current = 100;
        assert!(!strategy.can_execute(&player, &monster, &())); // HP is 100, not above 100

        monster.stats.hitpoints.current = 101;
        assert!(strategy.can_execute(&player, &monster, &())); // HP is 101, above 100
    }

    #[test]
    fn test_stat_reduction_conditions() {
        let player = create_test_player();
        let mut monster = Monster::new("General Graardor", None).expect("Error creating monster.");

        let switch = GearSwitch::new(SwitchType::Custom("Test spec".into()), &player, &monster);
        let strategy: SpecStrategy<CoreCondition> = SpecStrategy::builder(&switch)
            .with_target_def_reduction(50)
            .build();

        // Initially no reduction
        assert!(strategy.can_execute(&player, &monster, &()));

        // Drain some defence
        monster.drain_stat(&CombatStat::Defence, 30, None);
        assert!(strategy.can_execute(&player, &monster, &())); // 30 < 50, still can spec

        // Drain more
        monster.drain_stat(&CombatStat::Defence, 20, None);
        assert!(!strategy.can_execute(&player, &monster, &())); // 50 >= 50, can't spec anymore
    }

    #[test]
    fn test_core_condition_evaluate() {
        let player = create_test_player();
        let mut monster = Monster::new("General Graardor", None).expect("Error creating monster.");
        monster.stats.hitpoints.current = 75;

        // Test evaluate directly
        assert!(CoreCondition::MonsterHpBelow(100).evaluate(&player, &monster, &()));
        assert!(!CoreCondition::MonsterHpBelow(50).evaluate(&player, &monster, &()));
        assert!(CoreCondition::MonsterHpAbove(50).evaluate(&player, &monster, &()));
        assert!(!CoreCondition::MonsterHpAbove(100).evaluate(&player, &monster, &()));
    }

    #[test]
    fn test_edge_cases() {
        let player = create_test_player();

        // Empty strategies
        let config: SpecConfig<CoreCondition> =
            SpecConfig::new(vec![], SpecRestorePolicy::RestoreEveryKill, None, false);
        assert_eq!(config.strategies.len(), 0);

        // Test with immune monster
        let immune_monster = Monster::new("Dawn", None).expect("Error creating monster.");
        let fight = SingleWayFight::new(
            player.clone(),
            immune_monster,
            SingleWayConfig::default(),
            None,
            false,
        )
        .expect("Error setting up single way fight.");
        assert!(fight.is_immune());
    }

    #[test]
    fn test_state_reset() {
        let player = create_test_player();
        let monster = Monster::new("General Graardor", None).expect("Error creating monster.");

        let switch = GearSwitch::new(SwitchType::Custom("Test spec".into()), &player, &monster);
        let mut strategy: SpecStrategy<CoreCondition> = SpecStrategy::new(&switch, None);

        strategy.state.attempt_count = 5;
        strategy.state.success_count = 3;

        strategy.reset();

        assert_eq!(strategy.state.attempt_count, 0);
        assert_eq!(strategy.state.success_count, 0);
    }

    #[test]
    fn test_strategy_reset_via_method() {
        let player = create_test_player();
        let monster = Monster::new("General Graardor", None).expect("Error creating monster.");

        let switch = GearSwitch::new(SwitchType::Custom("Test spec".into()), &player, &monster);
        let mut strategy: SpecStrategy<CoreCondition> = SpecStrategy::new(&switch, None);

        strategy.state.attempt_count = 5;
        strategy.state.success_count = 3;

        // Test SpecStrategy::reset() calls state.reset()
        strategy.reset();
        assert_eq!(strategy.state.attempt_count, 0);
        assert_eq!(strategy.state.success_count, 0);
    }
}
