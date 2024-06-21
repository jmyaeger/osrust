// Covers any type of effect that gets applied over time
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CombatEffect {
    Poison {
        tick_counter: Option<i32>,
        severity: u32,
    },
    Venom {
        tick_counter: Option<i32>,
        damage: u32,
    },
    Burn {
        tick_counter: Option<i32>,
        stacks: Vec<u32>,
    },
}

impl CombatEffect {
    pub fn apply(&mut self) -> u32 {
        match self {
            Self::Poison {
                ref mut tick_counter,
                ref mut severity,
            } => apply_poison(tick_counter, severity),
            Self::Venom {
                ref mut tick_counter,
                ref mut damage,
            } => apply_venom(tick_counter, damage),
            Self::Burn {
                ref mut tick_counter,
                ref mut stacks,
            } => apply_burn(tick_counter, stacks),
        }
    }

    pub fn tick_counter(&self) -> Option<i32> {
        match self {
            Self::Poison { tick_counter, .. } => *tick_counter,
            Self::Venom { tick_counter, .. } => *tick_counter,
            Self::Burn { tick_counter, .. } => *tick_counter,
        }
    }
}

#[derive(Debug)]
pub struct Poison {
    pub tick_counter: Option<i32>,
    pub severity: u32,
}

fn apply_poison(tick_counter: &mut Option<i32>, severity: &mut u32) -> u32 {
    // If the severity is 0, the poison effect has worn off and the tick counter can be reset
    if *severity == 0 {
        *tick_counter = None;
        0
    } else if let Some(mut tick) = tick_counter {
        // Otherwise, increment tick counter, apply poison damage every 30 ticks, and increase severity
        tick += 1;
        if tick == 30 {
            *tick_counter = Some(0);
            *severity -= 1;
            (*severity + 4) / 5
        } else {
            0
        }
    } else {
        // If severity is nonzero and tick counter is None, poison has just been inflicted
        *tick_counter = Some(0);
        *severity -= 1;
        (*severity + 4) / 5
    }
}

fn apply_venom(tick_counter: &mut Option<i32>, damage: &mut u32) -> u32 {
    if let Some(mut tick) = tick_counter {
        // Increment tick counter, apply venom damage every 30 ticks, and increase damage
        tick += 1;
        if tick == 30 {
            *tick_counter = Some(0);
            *damage += 2;
            *damage
        } else {
            0
        }
    } else {
        // If tick counter is None, venom has just been inflicted
        *tick_counter = Some(0);
        *damage = 6;
        *damage
    }
}

fn apply_burn(tick_counter: &mut Option<i32>, stacks: &mut Vec<u32>) -> u32 {
    // Default to a damage of 0
    let mut damage = 0;

    if let Some(tick) = tick_counter {
        // Apply 1 damage for each active stack every 4 ticks
        if *tick % 4 == 0 {
            damage += stacks.len() as u32;

            // Decrease each stack by 1 and remove stacks with 0 values
            *stacks = stacks.iter().map(|&s| s - 1).collect();
            stacks.retain(|s| *s > 0);

            // Burn effect has ended if there are no stacks
            if stacks.is_empty() {
                *tick_counter = None;
                return damage;
            }
        }
        *tick_counter = Some(tick_counter.unwrap_or(0) + 1);
    } else if !stacks.is_empty() {
        // If tick counter is None, burn has just been inflicted
        *tick_counter = Some(0);

        // Apply damage on the first active tick of the effect
        damage += stacks.len() as u32;

        // Decrease each stack by 1 and remove stacks with 0 values
        *stacks = stacks.iter().map(|&s| s.saturating_sub(1)).collect();
        stacks.retain(|s| *s > 0);
    };
    damage
}
