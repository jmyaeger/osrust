#[cfg(test)]
mod spec_tests {
    use osrs::combat::simulation::Simulation;
    use osrs::sims::single_way::*;
    use osrs::types::equipment::CombatStyle;
    use osrs::types::monster::{CombatStat, Monster};
    use osrs::types::player::SwitchType;
    use osrs::types::player::{GearSwitch, Player};
    use osrs::types::potions::Potion;
    use osrs::types::prayers::Prayer;
    use osrs::types::stats::PlayerStats;

    // Helper function to create a test player
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

    // Test basic spec strategy creation
    #[test]
    fn test_spec_strategy_creation() {
        let player = create_test_player();
        let monster = Monster::new("General Graardor", None).expect("Error creating monster.");
        let switch = GearSwitch::new(SwitchType::Custom("Test spec".into()), &player, &monster);

        let strategy = SpecStrategy::new(&switch, None);

        assert_eq!(strategy.switch_type.label(), "Test spec");
        assert_eq!(strategy.spec_cost, 25); // Fang spec cost
        assert!(strategy.conditions.is_empty());
        assert_eq!(strategy.state.attempt_count, 0);
        assert_eq!(strategy.state.success_count, 0);
    }

    // Test spec config creation and lowest cost
    #[test]
    fn test_spec_config_lowest_cost() {
        let player = create_test_player();
        let monster = Monster::new("General Graardor", None).expect("Error creating monster.");

        // Create multiple strategies with different costs
        let fang_switch =
            GearSwitch::new(SwitchType::Custom("Fang spec".into()), &player, &monster);
        let fang_strategy = SpecStrategy::new(&fang_switch, None);

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

        assert_eq!(config.lowest_cost(), Some(25)); // Fang is lower than claws
    }

    // Test condition evaluation
    #[test]
    fn test_condition_evaluation() {
        let player = create_test_player();
        let monster = Monster::new("General Graardor", None).expect("Error creating monster.");

        let switch = GearSwitch::new(SwitchType::Custom("Test spec".into()), &player, &monster);
        let mut strategy = SpecStrategy::new(&switch, None);

        // Test MaxAttempts condition
        strategy.add_condition(SpecCondition::MaxAttempts(2));
        let config = SpecConfig::new(
            vec![strategy.clone()],
            SpecRestorePolicy::NeverRestore,
            None,
            false,
        );
        let fight = SingleWayFight::new(
            player.clone(),
            monster.clone(),
            SingleWayConfig::default(),
            Some(config),
            false,
        )
        .expect("Error setting up single way fight.");

        // Should allow first two attempts
        assert!(check_spec_conditions(&strategy, &fight));
        strategy.state.attempt_count = 1;
        assert!(check_spec_conditions(&strategy, &fight));
        strategy.state.attempt_count = 2;
        assert!(!check_spec_conditions(&strategy, &fight));
    }

    // Test HP-based conditions
    #[test]
    fn test_hp_conditions() {
        let player = create_test_player();
        let mut monster = Monster::new("General Graardor", None).expect("Error creating monster.");
        monster.stats.hitpoints.current = 100;

        let switch = GearSwitch::new(SwitchType::Custom("Test spec".into()), &player, &monster);
        let mut strategy = SpecStrategy::new(&switch, None);

        // Test MonsterHpBelow
        strategy.add_condition(SpecCondition::MonsterHpBelow(50));
        let config = SpecConfig::new(
            vec![strategy.clone()],
            SpecRestorePolicy::NeverRestore,
            None,
            false,
        );
        let mut fight = SingleWayFight::new(
            player.clone(),
            monster.clone(),
            SingleWayConfig::default(),
            Some(config),
            false,
        )
        .expect("Error setting up single way fight.");

        assert!(!check_spec_conditions(&strategy, &fight)); // HP is 100, not below 50

        fight.monster.stats.hitpoints.current = 50;
        assert!(check_spec_conditions(&strategy, &fight)); // HP is 50, equals threshold

        fight.monster.stats.hitpoints.current = 49;
        assert!(check_spec_conditions(&strategy, &fight)); // HP is 49, below threshold
    }

    // Test stat reduction conditions
    #[test]
    fn test_stat_reduction_conditions() {
        let player = create_test_player();
        let monster = Monster::new("General Graardor", None).expect("Error creating monster.");

        let switch = GearSwitch::new(SwitchType::Custom("Test spec".into()), &player, &monster);
        let mut strategy = SpecStrategy::new(&switch, None);

        // Test TargetDefenceReduction
        strategy.add_condition(SpecCondition::TargetDefenceReduction(50));
        let config = SpecConfig::new(
            vec![strategy.clone()],
            SpecRestorePolicy::NeverRestore,
            None,
            false,
        );
        let mut fight = SingleWayFight::new(
            player.clone(),
            monster.clone(),
            SingleWayConfig::default(),
            Some(config),
            false,
        )
        .expect("Error setting up single way fight.");

        // Initially no reduction
        assert!(check_spec_conditions(&strategy, &fight));

        // Drain some defence
        fight.monster.drain_stat(&CombatStat::Defence, 30, None);
        assert!(check_spec_conditions(&strategy, &fight)); // 30 < 50, still can spec

        // Drain more
        fight.monster.drain_stat(&CombatStat::Defence, 20, None);
        assert!(!check_spec_conditions(&strategy, &fight)); // 50 = 50, can't spec anymore
    }

    // Test edge cases
    #[test]
    fn test_edge_cases() {
        let player = create_test_player();

        // Empty strategies
        let config = SpecConfig::new(vec![], SpecRestorePolicy::RestoreEveryKill, None, false);
        assert_eq!(config.strategies.len(), 0);
        // lowest_cost() should handle empty strategies gracefully

        // Test with immune monster
        let immune_monster = Monster::new("Dawn", None).expect("Error creating monster."); // Immune to melee
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

    // Test state reset
    #[test]
    fn test_state_reset() {
        let player = create_test_player();
        let monster = Monster::new("General Graardor", None).expect("Error creating monster.");

        let switch = GearSwitch::new(SwitchType::Custom("Test spec".into()), &player, &monster);
        let mut strategy = SpecStrategy::new(&switch, None);

        // Modify state
        strategy.state.attempt_count = 5;
        strategy.state.success_count = 3;

        // Reset
        strategy.state.reset();

        assert_eq!(strategy.state.attempt_count, 0);
        assert_eq!(strategy.state.success_count, 0);
    }

    // Helper function to check conditions (extracted from actual implementation)
    fn check_spec_conditions(strategy: &SpecStrategy, fight: &SingleWayFight) -> bool {
        strategy.conditions.iter().all(|condition| match condition {
            SpecCondition::MaxAttempts(attempts) => strategy.state.attempt_count < *attempts,
            SpecCondition::MinSuccesses(successes) => strategy.state.success_count < *successes,
            SpecCondition::MonsterHpAbove(hp) => fight.monster.stats.hitpoints.current > *hp,
            SpecCondition::MonsterHpBelow(hp) => fight.monster.stats.hitpoints.current <= *hp,
            SpecCondition::PlayerHpAbove(hp) => fight.player.stats.hitpoints.current > *hp,
            SpecCondition::PlayerHpBelow(hp) => fight.player.stats.hitpoints.current <= *hp,
            SpecCondition::TargetDefenceReduction(amt) => {
                fight
                    .monster
                    .stats
                    .defence
                    .base
                    .saturating_sub(fight.monster.stats.defence.current)
                    < *amt
            }
            SpecCondition::TargetMagicReduction(amt) => {
                fight
                    .monster
                    .stats
                    .magic
                    .base
                    .saturating_sub(fight.monster.stats.magic.current)
                    < *amt
            }
            SpecCondition::TargetMagicDefReduction(amt) => {
                let base_def = fight.monster.bonuses.defence.magic_base;
                let current_def = fight.monster.bonuses.defence.magic;
                (base_def - current_def) < *amt
            }
            SpecCondition::TargetAttackReduction(amt) => {
                fight
                    .monster
                    .stats
                    .attack
                    .base
                    .saturating_sub(fight.monster.stats.attack.current)
                    < *amt
            }
            SpecCondition::TargetStrengthReduction(amt) => {
                fight
                    .monster
                    .stats
                    .strength
                    .base
                    .saturating_sub(fight.monster.stats.strength.current)
                    < *amt
            }
            SpecCondition::TargetRangedReduction(amt) => {
                fight
                    .monster
                    .stats
                    .ranged
                    .base
                    .saturating_sub(fight.monster.stats.ranged.current)
                    < *amt
            }
        })
    }
}
