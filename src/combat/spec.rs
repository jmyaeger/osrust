use crate::constants;
use crate::types::monster::Monster;
use crate::types::player::{GearSwitch, Player, SwitchType};
use crate::types::timers::Timer;
use crate::utils::logging::FightLogger;

pub trait SpecCondition: Clone + PartialEq {
    type BossState;

    fn evaluate(&self, player: &Player, monster: &Monster, boss_state: &Self::BossState) -> bool;

    fn as_core(&self) -> Option<&CoreCondition>;
    fn from_core(core: CoreCondition) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoreCondition {
    MonsterHpBelow(u32),
    MonsterHpAbove(u32),
    MonsterHpPercentBelow(u8),
    MonsterHpPercentAbove(u8),
    PlayerHpBelow(u32),
    PlayerHpAbove(u32),
    TargetDefenceReduction(u32),
    TargetMagicReduction(u32),
    TargetMagicDefReduction(i32),
    TargetAttackReduction(u32),
    TargetStrengthReduction(u32),
    TargetRangedReduction(u32),
}

impl SpecCondition for CoreCondition {
    type BossState = ();

    fn evaluate(&self, player: &Player, monster: &Monster, _: &Self::BossState) -> bool {
        match self {
            Self::MonsterHpAbove(hp) => monster.stats.hitpoints.current > *hp,
            Self::MonsterHpBelow(hp) => monster.stats.hitpoints.current <= *hp,
            Self::MonsterHpPercentAbove(pct) => {
                monster.stats.hitpoints.current * 100 / monster.stats.hitpoints.base
                    > u32::from(*pct)
            }
            Self::MonsterHpPercentBelow(pct) => {
                monster.stats.hitpoints.current * 100 / monster.stats.hitpoints.base
                    <= u32::from(*pct)
            }
            Self::PlayerHpAbove(hp) => player.stats.hitpoints.current > *hp,
            Self::PlayerHpBelow(hp) => player.stats.hitpoints.current <= *hp,
            Self::TargetAttackReduction(amt) => {
                monster
                    .stats
                    .attack
                    .base
                    .saturating_sub(monster.stats.attack.current)
                    < *amt
            }
            Self::TargetStrengthReduction(amt) => {
                monster
                    .stats
                    .strength
                    .base
                    .saturating_sub(monster.stats.strength.current)
                    < *amt
            }
            Self::TargetDefenceReduction(amt) => {
                monster
                    .stats
                    .defence
                    .base
                    .saturating_sub(monster.stats.defence.current)
                    < *amt
            }
            Self::TargetRangedReduction(amt) => {
                monster
                    .stats
                    .ranged
                    .base
                    .saturating_sub(monster.stats.ranged.current)
                    < *amt
            }
            Self::TargetMagicReduction(amt) => {
                monster
                    .stats
                    .magic
                    .base
                    .saturating_sub(monster.stats.magic.current)
                    < *amt
            }
            Self::TargetMagicDefReduction(amt) => {
                monster
                    .bonuses
                    .defence
                    .magic_base
                    .saturating_sub(monster.bonuses.defence.magic)
                    < *amt
            }
        }
    }

    fn as_core(&self) -> Option<&CoreCondition> {
        Some(self)
    }

    fn from_core(core: CoreCondition) -> Self {
        core
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum DeathCharge {
    Single,
    Double,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpecConfig<C: SpecCondition> {
    pub strategies: Vec<SpecStrategy<C>>,
    pub restore_policy: SpecRestorePolicy,
    pub death_charge: Option<DeathCharge>,
    pub surge_potion: bool,
    lowest_cost: Option<u8>,
}

impl<C: SpecCondition> SpecConfig<C> {
    pub fn new(
        strategies: Vec<SpecStrategy<C>>,
        restore_policy: SpecRestorePolicy,
        death_charge: Option<DeathCharge>,
        surge_potion: bool,
    ) -> Self {
        let lowest_cost = strategies.iter().map(|s| s.spec_cost).min();
        Self {
            strategies,
            restore_policy,
            death_charge,
            surge_potion,
            lowest_cost,
        }
    }

    pub fn add_strategy(&mut self, strategy: SpecStrategy<C>, position: Option<usize>) {
        if let Some(ind) = position {
            self.strategies.insert(ind, strategy);
        } else {
            self.strategies.push(strategy);
        }
    }

    pub fn lowest_cost(&self) -> Option<u8> {
        self.lowest_cost
            .or_else(|| self.strategies.iter().map(|s| s.spec_cost).min())
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.strategies.is_empty() {
            return Err("No spec strategies defined".to_string());
        }

        for strategy in &self.strategies {
            Self::validate_conditions(&strategy.conditions)?;
        }

        Ok(())
    }

    fn validate_conditions(conditions: &[C]) -> Result<(), String> {
        // Check for HP conflicts
        let hp_above: Vec<_> = conditions
            .iter()
            .filter_map(|c| {
                if let Some(CoreCondition::MonsterHpAbove(hp)) = c.as_core() {
                    Some(*hp)
                } else {
                    None
                }
            })
            .collect();

        let hp_below: Vec<_> = conditions
            .iter()
            .filter_map(|c| {
                if let Some(CoreCondition::MonsterHpBelow(hp)) = c.as_core() {
                    Some(*hp)
                } else {
                    None
                }
            })
            .collect();

        for above in &hp_above {
            for below in &hp_below {
                if above >= below {
                    return Err(format!(
                        "Conflicting HP conditions: above {above} and below {below}"
                    ));
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpecRestorePolicy {
    RestoreEveryKill,
    RestoreAfter(u32),
    NeverRestore,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpecStrategy<C: SpecCondition> {
    pub conditions: Vec<C>,
    pub state: SpecStrategyState,
    pub switch_type: SwitchType,
    pub spec_cost: u8,
    pub max_attempts: Option<u8>,
    pub min_successes: Option<u8>,
}

impl<C: SpecCondition> SpecStrategy<C> {
    pub fn new(gear: &GearSwitch, conditions: Option<Vec<C>>) -> Self {
        let spec_cost = constants::SPEC_COSTS
            .iter()
            .find(|w| w.0 == gear.gear.weapon.name)
            .expect("Spec cost not found")
            .1;

        Self {
            conditions: conditions.unwrap_or_default(),
            state: SpecStrategyState::default(),
            switch_type: gear.switch_type.clone(),
            spec_cost,
            max_attempts: None,
            min_successes: None,
        }
    }

    pub fn add_condition(&mut self, condition: C) {
        self.conditions.push(condition);
    }

    pub fn can_execute(
        &self,
        player: &Player,
        monster: &Monster,
        boss_state: &C::BossState,
    ) -> bool {
        if !player.stats.spec.has_enough(self.spec_cost) {
            return false;
        }

        if let Some(max) = self.max_attempts
            && self.state.attempt_count >= max
        {
            return false;
        }

        if let Some(min) = self.min_successes
            && self.state.success_count >= min
        {
            return false;
        }

        self.conditions
            .iter()
            .all(|condition| condition.evaluate(player, monster, boss_state))
    }

    pub fn builder(gear: &GearSwitch) -> SpecStrategyBuilder<C> {
        SpecStrategyBuilder::new(gear)
    }

    pub fn reset(&mut self) {
        self.state.reset();
    }
}

#[derive(Debug)]
pub struct SpecStrategyBuilder<C: SpecCondition> {
    strategy: SpecStrategy<C>,
}

impl<C: SpecCondition> SpecStrategyBuilder<C> {
    pub fn new(gear: &GearSwitch) -> Self {
        Self {
            strategy: SpecStrategy::new(gear, None),
        }
    }

    fn with_condition(mut self, condition: C) -> Self {
        self.strategy.add_condition(condition);
        self
    }

    pub fn with_max_attempts(mut self, attempts: u8) -> Self {
        self.strategy.max_attempts = Some(attempts);
        self
    }

    pub fn with_min_successes(mut self, successes: u8) -> Self {
        self.strategy.min_successes = Some(successes);
        self
    }

    pub fn with_monster_hp_below(self, hp: u32) -> Self {
        self.with_condition(C::from_core(CoreCondition::MonsterHpBelow(hp)))
    }

    pub fn with_monster_hp_above(self, hp: u32) -> Self {
        self.with_condition(C::from_core(CoreCondition::MonsterHpAbove(hp)))
    }

    pub fn with_target_def_reduction(self, amount: u32) -> Self {
        self.with_condition(C::from_core(CoreCondition::TargetDefenceReduction(amount)))
    }

    pub fn build(self) -> SpecStrategy<C> {
        self.strategy
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpecStrategyState {
    pub attempt_count: u8,
    pub success_count: u8,
}

impl SpecStrategyState {
    pub fn reset(&mut self) {
        self.attempt_count = 0;
        self.success_count = 0;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpecState {
    pub kill_counter: u32,
    pub death_charge_procs: u8,
    pub death_charge_cd: Timer,
    pub surge_potion_cd: Timer,
    pub spec_regen_timer: Timer,
}

impl Default for SpecState {
    fn default() -> Self {
        Self {
            kill_counter: 0,
            death_charge_procs: 0,
            death_charge_cd: Timer::new(Some(constants::DEATH_CHARGE_CD)),
            surge_potion_cd: Timer::new(Some(constants::SURGE_POTION_CD)),
            spec_regen_timer: Timer::new(None),
        }
    }
}

impl SpecState {
    pub fn reset(&mut self) {
        self.kill_counter = 0;
        self.death_charge_procs = 0;
        self.death_charge_cd.reset();
        self.surge_potion_cd.reset();
        self.spec_regen_timer.reset();
    }

    pub fn increment_spec(
        &mut self,
        player: &mut Player,
        tick_counter: i32,
        logger: &mut FightLogger,
    ) {
        if self.spec_regen_timer.is_active() {
            self.spec_regen_timer.increment();
            if (player.is_wearing("Lightbearer", None)
                && self.spec_regen_timer.counter().is_multiple_of(25))
                || self.spec_regen_timer.counter().is_multiple_of(50)
            {
                player.stats.spec.regen();
                if logger.enabled {
                    logger.log_custom(
                        tick_counter,
                        format!(
                            "Player has regenerated 10 special attack energy ({} remaining)",
                            player.stats.spec.value()
                        )
                        .as_str(),
                    );
                }
            }
            if player.stats.spec.is_full() {
                self.spec_regen_timer.reset();
            }
        }
    }

    pub fn increment_timers(&mut self) {
        self.death_charge_cd.increment();
        self.surge_potion_cd.increment();

        if self.death_charge_cd.counter() == 0 && !self.death_charge_cd.is_active() {
            self.death_charge_procs = 0;
        }
    }

    pub fn process_death_charge<C: SpecCondition>(
        &mut self,
        player: &mut Player,
        config: &SpecConfig<C>,
    ) {
        if self.death_charge_cd.is_active() {
            let max_procs = match config.death_charge {
                Some(DeathCharge::Single) => 1,
                Some(DeathCharge::Double) => 2,
                None => 0,
            };
            if self.death_charge_procs < max_procs {
                player.stats.spec.death_charge();
                self.death_charge_procs += 1;
            }
        } else if config.death_charge.is_some() {
            player.stats.spec.death_charge();
            self.death_charge_procs += 1;
            self.death_charge_cd.activate();
        }
    }

    pub fn process_surge_potion<C: SpecCondition>(
        &mut self,
        player: &mut Player,
        config: &SpecConfig<C>,
    ) {
        if config.surge_potion
            && player.stats.spec.value() <= 75
            && !self.surge_potion_cd.is_active()
        {
            player.stats.spec.surge_potion();
            self.surge_potion_cd.activate();
        }
    }

    pub fn on_kill<C: SpecCondition>(
        &mut self,
        player: &mut Player,
        config: &mut SpecConfig<C>,
    ) -> bool {
        self.kill_counter += 1;
        self.process_death_charge(player, config);

        let should_restore = match config.restore_policy {
            SpecRestorePolicy::NeverRestore => false,
            SpecRestorePolicy::RestoreAfter(kills) => self.kill_counter >= kills,
            SpecRestorePolicy::RestoreEveryKill => true,
        };

        if should_restore {
            self.reset();
        }

        config.strategies.iter_mut().for_each(|s| s.state.reset());

        should_restore
    }
}
